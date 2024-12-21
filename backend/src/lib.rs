use std::sync::Once;

use anyhow::Result;
use geo::{GeometryCollection, LineString};
use geojson::de::deserialize_geometry;
use rstar::{primitives::GeomWithData, RTree};
use serde::Deserialize;
use wasm_bindgen::prelude::*;

use utils::Mercator;

static START: Once = Once::new();

/// Takes two GeoJSONs, matches LineStrings, and returns the optional index of the best matching
/// feature for each target.
#[wasm_bindgen(js_name = matchLineStrings)]
pub fn match_linestrings(source_gj: String, target_gj: String) -> Result<String, JsValue> {
    // Panics shouldn't happen, but if they do, console.log them.
    console_error_panic_hook::set_once();
    START.call_once(|| {
        console_log::init_with_level(log::Level::Info).unwrap();
    });

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

    let opts = match_linestrings::Options {
        buffer_meters: 20.0,
        angle_diff_threshold: 10.0,
        length_ratio_threshold: 1.1,
        midpt_dist_threshold: 15.0,
    };

    let out = match_linestrings::match_linestrings(
        &source_rtree,
        targets.iter().map(|x| &x.geometry),
        &opts,
    );
    serde_json::to_string(&out).map_err(err_to_js)
}

#[derive(Deserialize)]
struct Input {
    #[serde(deserialize_with = "deserialize_geometry")]
    geometry: LineString,
}

fn err_to_js<E: std::fmt::Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}
