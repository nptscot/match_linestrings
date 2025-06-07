use anyhow::Result;
use geo::{Distance, Euclidean, InterpolatableLine, Length, LineLocatePoint, LineString, Point};
use rstar::{primitives::GeomWithData, RTree, RTreeObject, AABB};
use serde::{Deserialize, Serialize};
use utils::{LineSplit, Mercator};

/// For every target LineString, look for all matching source LineStrings. The sources are stored
/// in the `rtree`. All geometry must be Euclidean.
pub fn match_linestrings<'a, T: Copy>(
    rtree: &RTree<GeomWithData<LineString, T>>,
    targets: impl Iterator<Item = &'a LineString>,
    opts: &Options,
) -> Vec<TargetMatches<T>> {
    let mut output = Vec::new();
    for target in targets {
        let mut matching_sources = Vec::new();
        for candidate in rtree
            .locate_in_envelope_intersecting(&buffer_aabb(target.envelope(), opts.buffer_meters))
        {
            if let Some((smaller_source, smaller_target)) =
                slice_lines_to_match(candidate.geom(), target)
            {
                if opts.accept(&smaller_target, &smaller_source) {
                    matching_sources.push(candidate.data);
                }
            }
        }
        output.push(TargetMatches { matching_sources });
    }
    output
}

/// For one target, record all of the matching sources.
#[derive(Serialize)]
pub struct TargetMatches<T: Copy> {
    pub matching_sources: Vec<T>,
    // TODO Plumb along CompareLineStrings debug too?
}

/// Same as `match_linestrings`, but for every target with no successful matches, write some
/// GeoJSON output to manually debug.
// TODO Rethink this -- do it in the web API, plumbing CompareLineStrings back.
pub fn debug_match_linestrings<'a, T: Copy>(
    rtree: &RTree<GeomWithData<LineString, T>>,
    targets: impl Iterator<Item = &'a LineString>,
    opts: &Options,
    // For transforming Euclidean coordinates to WGS84
    mercator: &Mercator,
    // If true, produce just one "debug_all.geojson". Otherwise, produce a "debug{idx}.geojson"
    // file for each failure.
    one_file: bool,
    // If this is set, only debug the given target index
    only_debug_idx: Option<usize>,
) -> Result<Vec<Option<T>>> {
    let mut all_out = if one_file {
        Some(geojson::FeatureWriter::from_writer(std::fs::File::create(
            "debug_all.geojson",
        )?))
    } else {
        None
    };

    let mut output = Vec::new();
    for (idx, target) in targets.enumerate() {
        let candidates = rtree
            .locate_in_envelope_intersecting(&buffer_aabb(target.envelope(), opts.buffer_meters))
            .collect::<Vec<_>>();
        // TODO If there are multiple hits, pick the best
        let best_hit = candidates
            .iter()
            .find(|obj| {
                slice_line_to_match(obj.geom(), target)
                    .map(|small| opts.accept(target, &small))
                    .unwrap_or(false)
            })
            .map(|obj| obj.data);

        if best_hit.is_none()
            && !candidates.is_empty()
            && only_debug_idx.map(|i| idx == i).unwrap_or(true)
        {
            let mut one_out = if one_file {
                None
            } else {
                Some(geojson::FeatureWriter::from_writer(std::fs::File::create(
                    format!("debug{idx}.geojson"),
                )?))
            };
            let out = all_out.as_mut().or_else(|| one_out.as_mut()).unwrap();

            let mut f = mercator.to_wgs84_gj(target);
            f.set_property("kind", "target");
            f.set_property("idx", idx);
            out.write_feature(&f)?;

            for obj in candidates {
                let cmp = CompareLineStrings::new(target, obj.geom());
                let mut f = mercator.to_wgs84_gj(obj.geom());
                f.properties = Some(serde_json::to_value(&cmp)?.as_object().unwrap().clone());
                f.set_property("kind", "full source");
                //out.write_feature(&f)?;

                if let Some(small) = slice_line_to_match(obj.geom(), target) {
                    let cmp = CompareLineStrings::new(target, &small);
                    f = mercator.to_wgs84_gj(&small);
                    f.properties = Some(serde_json::to_value(&cmp)?.as_object().unwrap().clone());
                    f.set_property("kind", "sliced source");
                    out.write_feature(&f)?;
                }
            }
        }

        output.push(best_hit);
    }
    Ok(output)
}

#[derive(Deserialize)]
pub struct Options {
    /// Expand the bounding box around each target by this amount in all directions
    pub buffer_meters: f64,
    /// How many degrees difference allowed between matches? LineStrings pointing in opposite
    /// directions are ignored.
    pub angle_diff_threshold: f64,
    /// How large may the ratio of lengths between the candidates be? The ratio is always >= 1
    /// (longer.length / shorter.length)
    pub length_ratio_threshold: f64,
    /// How far away can the midpoints of the candidates be, in meters?
    pub midpt_dist_threshold: f64,
}

impl Options {
    fn accept(&self, ls1: &LineString, ls2: &LineString) -> bool {
        let cmp = CompareLineStrings::new(ls1, ls2);
        cmp.angle_diff <= self.angle_diff_threshold
            && cmp.length_ratio <= self.length_ratio_threshold
            && cmp.midpt_dist <= self.midpt_dist_threshold
    }
}

// Bundle all of the relevant calculations together both for actually using and for convenient
// debugging
#[derive(Serialize)]
struct CompareLineStrings {
    angle_main: f64,
    angle_candidate: f64,
    angle_diff: f64,

    length_main: f64,
    length_candidate: f64,
    length_ratio: f64,

    midpt_dist: f64,
}

impl CompareLineStrings {
    fn new(main: &LineString, candidate: &LineString) -> Self {
        let angle_main = angle_ls(main);
        let angle_candidate = angle_ls(candidate);
        let length_main = Euclidean.length(main);
        let length_candidate = Euclidean.length(candidate);
        let midpt_dist = midpoint_distance(main, candidate);

        Self {
            angle_main,
            angle_candidate,
            angle_diff: (angle_main - angle_candidate).abs(),
            length_main,
            length_candidate,
            // Always >= 1
            length_ratio: if length_main >= length_candidate {
                length_main / length_candidate
            } else {
                length_candidate / length_main
            },
            midpt_dist,
        }
    }
}

// Angle in degrees from first to last point. Ignores the "direction" of the line; returns [0,
// 180].
// TODO Needs unit testing!
fn angle_ls(ls: &LineString) -> f64 {
    let pt1 = ls.coords().next().unwrap();
    let pt2 = ls.coords().last().unwrap();
    let a1 = (pt2.y - pt1.y).atan2(pt2.x - pt1.x).to_degrees();
    // Normalize to [0, 360]
    let a2 = if a1 < 0.0 { a1 + 360.0 } else { a1 };
    // Ignore direction
    if a2 > 180.0 {
        a2 - 180.0
    } else {
        a2
    }
}

// Distance in meters between the middle of each linestring. Because ls1 and ls2 might point
// opposite directions, using the start/end point is unnecessarily trickier.
fn midpoint_distance(ls1: &LineString, ls2: &LineString) -> f64 {
    let pt1 = ls1.point_at_ratio_from_start(&Euclidean, 0.5).unwrap();
    let pt2 = ls2.point_at_ratio_from_start(&Euclidean, 0.5).unwrap();
    Euclidean.distance(pt1, pt2)
}

// Expand an AABB by some amount on all sides
fn buffer_aabb(aabb: AABB<Point>, buffer_meters: f64) -> AABB<Point> {
    AABB::from_corners(
        Point::new(
            aabb.lower().x() - buffer_meters,
            aabb.lower().y() - buffer_meters,
        ),
        Point::new(
            aabb.upper().x() + buffer_meters,
            aabb.upper().y() + buffer_meters,
        ),
    )
}

// Slice `a` to correspond to `b`, by finding the closest point along `a` matching `b`'s start and
// end point.
fn slice_line_to_match(a: &LineString, b: &LineString) -> Option<LineString> {
    let start = a.line_locate_point(&b.points().next().unwrap())?;
    let end = a.line_locate_point(&b.points().last().unwrap())?;
    // Note this uses a copy of an API that hasn't been merged into georust yet. It seems to work
    // fine in practice.
    a.line_split_twice(start, end)?.into_second()
}

// TODO Diagram of example cases would help
fn slice_lines_to_match(
    source: &LineString,
    target: &LineString,
) -> Option<(LineString, LineString)> {
    if Euclidean.length(source) >= Euclidean.length(target) {
        let smaller_source = slice_line_to_match(source, target)?;
        return Some((smaller_source, target.clone()));
    }

    let smaller_target = slice_line_to_match(target, source)?;
    Some((source.clone(), smaller_target))
}
