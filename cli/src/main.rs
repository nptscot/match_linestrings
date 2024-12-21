use std::fs::File;
use std::io::{BufReader, BufWriter};

use anyhow::{bail, Result};
use clap::Parser;
use geo::{GeometryCollection, LineString};
use rstar::{primitives::GeomWithData, RTree};
use utils::Mercator;

#[derive(Parser)]
struct Args {
    /// Path to a .geojson file with LineStrings in WGS84. Since a Mercator projection is used to
    /// transform to Euclidean coordinates, the total covered area shouldn't exceed roughly a city.
    #[arg(long)]
    source: String,

    /// Path to a .geojson file with LineStrings in WGS84. Since a Mercator projection is used to
    /// transform to Euclidean coordinates, the total covered area shouldn't exceed roughly a city.
    #[arg(long)]
    target: String,

    /// Expand the bounding box around each target by this amount in all directions
    #[arg(long, default_value_t = 20.0)]
    buffer_meters: f64,

    /// How many degrees difference allowed between matches? LineStrings pointing in opposite
    /// directions are ignored.
    #[arg(long, default_value_t = 10.0)]
    angle_diff_threshold: f64,

    /// How large may the ratio of lengths between the candidates be? The ratio is always >= 1
    /// (longer.length / shorter.length)
    #[arg(long, default_value_t = 1.1)]
    length_ratio_threshold: f64,

    /// How far away can the midpoints of the candidates be, in meters?
    #[arg(long, default_value_t = 15.0)]
    midpt_dist_threshold: f64,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let opts = match_linestrings::Options {
        buffer_meters: args.buffer_meters,
        angle_diff_threshold: args.angle_diff_threshold,
        length_ratio_threshold: args.length_ratio_threshold,
        midpt_dist_threshold: args.midpt_dist_threshold,
    };

    println!(
        "Reading {} and {} and transforming to Mercator",
        args.source, args.target
    );
    let mut source_features = read_features(&args.source)?;
    let mut target_features = read_features(&args.target)?;

    // TODO Expensive clones
    let collection = GeometryCollection::from(
        source_features
            .iter()
            .chain(target_features.iter())
            .map(|f| f.linestring.clone())
            .collect::<Vec<_>>(),
    );
    let Some(mercator) = Mercator::from(collection) else {
        bail!("Empty inputs");
    };
    for f in source_features.iter_mut().chain(target_features.iter_mut()) {
        mercator.to_mercator_in_place(&mut f.linestring);
    }

    println!("Building RTree for the source");
    let source_rtree = RTree::bulk_load(
        source_features
            .iter()
            .map(|f| GeomWithData::new(f.linestring.clone(), f.idx))
            .collect(),
    );

    println!("Matching and writing output.geojson");
    let mut writer =
        geojson::FeatureWriter::from_writer(BufWriter::new(File::create("output.geojson")?));
    let matches = match_linestrings::match_linestrings(
        &source_rtree,
        target_features.iter().map(|f| &f.linestring),
        &opts,
    );
    for (target, matched_idx) in target_features.into_iter().zip(matches.into_iter()) {
        // Only write successful matches
        let Some(idx) = matched_idx else {
            continue;
        };
        let mut f = mercator.to_wgs84_gj(&target.linestring);
        f.set_property("original_props", target.props);
        f.set_property("matched_props", source_features[idx].props.clone());
        writer.write_feature(&f)?;
    }

    Ok(())
}

struct FeatureWithProps {
    linestring: LineString,
    props: geojson::JsonObject,
    idx: usize,
}

fn read_features(path: &str) -> Result<Vec<FeatureWithProps>> {
    let mut results = Vec::new();
    for f in geojson::FeatureReader::from_reader(BufReader::new(File::open(path)?)).features() {
        let f = f?;
        let linestring: LineString = f.clone().try_into()?;
        results.push(FeatureWithProps {
            linestring,
            props: f.properties.unwrap_or(serde_json::Map::new()),
            idx: results.len(),
        });
    }
    Ok(results)
}
