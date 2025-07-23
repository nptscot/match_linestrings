# Test cases for LineString matching

There are many reasons you might have two LineString datasets you want to match together:

- Associating an OSM `footway=sidewalk` with the road, to determine the name or speed limit of the road it runs along
- Associating an OSM `highway=cycleway` with the road it runs along, or identifying an off-road segment
- Automatically identifying the OSM [is_sidepath](https://wiki.openstreetmap.org/wiki/Proposal:Key:is_sidepath) tag
- Matching an external dataset with road center-lines not based on OSM to OSM roads

There are a variety of ways to "map-match" or "snap" these two datasets together, but none work perfectly. I'd like to create a common benchmark of examples that somebody manually annotates with the correct matching, so that any implementation can then compare against it and use as test cases.

## The input

Each test case needs two files. The first is `sources.geojson`, containing LineString features with ascending numeric IDs corresponding to the feature's index. The properties do not matter. These represent the "roads", or things you want to match against.

The second is `targets.geojson`, representing the sidewalks or cycleways or external dataset that you want to match to the sources. It also has LineString features with ascending numeric IDs corresponding to the feature's index. The properties must contain two things:

1) `matching_sources`: a list of source IDs that this target corresponds to. `[]` indicates this LineString isn't parallel to any sources.
2) `reviewed`: the string `"unreviewed"`, `"not sure"`, or `"confirmed"` to indicate if the `matching_sources` are correct yet

Both files must split LineStrings when they cross at true intersections -- they must form a graph structure, acting as edges. OSM ways by default are usually not mapped this way, so they need to be split.

## Producing this input

First you need to pick some sources and targets. You could generate this from OSM data yourself by splitting ways when they cross at intersections and separating into roads and cycleways. A simple way is:

1) Go to <https://dabreegster.github.io/road-bundler>
2) Navigating somewhere interesting, picking an area with at least a few examples, but small enough (not an entire city in one file) to annotate reasonably quickly
3) "Import current view" or "Draw an area to import on the map"
4) Pressing "4" or clicking the 4th icon in the toolbar (Sidepaths)
5) "Destructively remove all footways"
6) "Download GJ of motorized and nonmotorized roads"
7) This should download two files in your browser

Then you can use a small tool to initially create the matching and annotate the file with this `reviewed` property.

1) Go to <https://nptscot.github.io/match_linestrings/review.html>
2) Load the two GeoJSON files you want to match
3) Press "Swap" if the red targets are not the cycleways
4) Press "Start review"
5) Review all targets. For each one, clicking sources to show they're matched or not, then confirming. For unclear cases, mark "not sure"
6) Press "Download reviews"

Please send the original `sources.geojson` file and the new `reviewed_targets.geojson` file for inclusion as a test case. File a [GH issue](https://github.com/nptscot/match_linestrings/issues/new), open a PR with the two files, email to <dabreegster@gmail.com>, etc.

## Examples

Most examples here contain OpenStreetMap data.

See <https://github.com/nptscot/match_linestrings/tree/main/tests/braganza_street> and <https://github.com/nptscot/match_linestrings/tree/main/tests/chuchyard_row>.
