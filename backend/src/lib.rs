use std::sync::Once;

use anyhow::Result;
use geo::{GeometryCollection, LineString};
use geojson::de::deserialize_geometry;
use rstar::{primitives::GeomWithData, RTree};
use serde::Deserialize;
use wasm_bindgen::prelude::*;

use utils::Mercator;

static START: Once = Once::new();

/// Takes two GeoJSONs, matches LineStrings, and returns the indices of all matching
/// sources for each target.
#[wasm_bindgen(js_name = matchLineStrings)]
pub fn match_linestrings(
    source_gj: String,
    target_gj: String,
    raw_options: JsValue,
) -> Result<String, JsValue> {
    // Panics shouldn't happen, but if they do, console.log them.
    console_error_panic_hook::set_once();
    START.call_once(|| {
        console_log::init_with_level(log::Level::Info).unwrap();
    });

    let options: match_linestrings::Options = serde_wasm_bindgen::from_value(raw_options)?;
    let (source_rtree, targets) = prepare_input(source_gj, target_gj)?;

    let out: Vec<match_linestrings::TargetMatches<usize>> =
        match_linestrings::match_linestrings(&source_rtree, targets.iter(), &options);
    serde_json::to_string(&out).map_err(err_to_js)
}

fn prepare_input(
    source_gj: String,
    target_gj: String,
) -> Result<(RTree<GeomWithData<LineString, usize>>, Vec<LineString>), JsValue> {
    let mut sources: Vec<Input> =
        geojson::de::deserialize_feature_collection_str_to_vec(&source_gj).map_err(err_to_js)?;
    let mut targets: Vec<Input> =
        geojson::de::deserialize_feature_collection_str_to_vec(&target_gj).map_err(err_to_js)?;

    // TODO Expensive clones
    let collection = GeometryCollection::from(
        sources
            .iter()
            .chain(targets.iter())
            .map(|x| x.geometry.clone())
            .collect::<Vec<_>>(),
    );
    let Some(mercator) = Mercator::from(collection) else {
        return Err(JsValue::from_str("empty inputs"));
    };
    for x in sources.iter_mut().chain(targets.iter_mut()) {
        mercator.to_mercator_in_place(&mut x.geometry);
    }

    let source_rtree = RTree::bulk_load(
        sources
            .into_iter()
            .enumerate()
            .map(|(idx, x)| GeomWithData::new(x.geometry, idx))
            .collect(),
    );

    Ok((
        source_rtree,
        targets.into_iter().map(|x| x.geometry).collect(),
    ))
}

#[derive(Deserialize)]
struct Input {
    #[serde(deserialize_with = "deserialize_geometry")]
    geometry: LineString,
}

fn err_to_js<E: std::fmt::Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}
