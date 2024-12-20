use std::fs::File;
use std::io::BufReader;

use anyhow::{bail, Result};
use clap::Parser;
use geo::LineString;

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

    let source_features = read_features(&args.source)?;
    let target_features = read_features(&args.target)?;

    //let results =
    //match_linestrings::match_linestrings(&rtree, graph.roads.iter().map(|r| &r.linestring), &opts);

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
