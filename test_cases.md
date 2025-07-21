# Test cases for LineString matching

There are many reasons you might have two LineString datasets you want to match together:

- Associating an OSM `footway=sidewalk` with the road, to determine the name or speed limit of the road it runs along
- Associating an OSM `highway=cycleway` with the road it runs along, or identifying an off-road segment
- Automatically identifying the OSM [is_sidepath](https://wiki.openstreetmap.org/wiki/Proposal:Key:is_sidepath) tag
- Matching an external dataset with road center-lines not based on OSM to OSM roads

There are a variety of ways to "map-match" or "snap" these two datasets together, but none work perfectly. I'd like to create a common benchmark of examples that somebody manually annotates with the correct matching, so that any implementation can then compare against it and use as test cases.

## The input

Each test case needs two files. The first is `sources.geojson`, containing LineString features with ascending numeric IDs corresponding to the feature's index. The properties do not matter. These represent the "roads", or things you want to match against.

The second is `targets.geojson`, representing the sidewalks or cycleways or external dataset that you want to match to the sources. It also has LineString features with ascending numeric IDs corresponding to the feature's index. The properties can contain anything extra, but they must have a `reviewed` property that has one of four values:

1) The string `"unreviewed"`, indicating it hasn't yet been annotated
2) The string `"not sure"`, indicating it's debatable whether this LineString is parallel to some sources or not
3) An empty array `[]`, indicating this LineString is not parallel to any sources
4) A list of source IDs, such as `[25]` or `[17, 38, 59]`, indicating this LineString matches with the source features containing those IDs

## Producing this input

First you need to pick some sources and targets. You could generate the example with cycleways from OSM using Overpass by:

1) Going to <https://overpass-turbo.eu>
2) Navigating somewhere interesting, picking an area with at least a few examples, but small enough (not an entire city in one file) to annotate reasonably quickly
3) Producing `sources.geojson` by selecting all roads with a wizard query such as `highway=~"motorway.*|trunk.*|primary.*|secondary.*|tertiary.*|residential|service|unclassified"` and then exporting as GeoJSON
4) Producing `targets.geojson` by selecting all cycleways with a wizard query such as `highway=cycleway` and then exporting as GeoJSON

Then you can use a small tool to annotate the file with this `reviewed` property.

1) Go to <https://nptscot.github.io/match_linestrings/review.html>
2) Load the two GeoJSON files you want to match
3) Press "Swap" if the red targets are not the cycleways
4) Scrolling down and pressing "Start review"
5) Reviewing all targets. For each one, choosing "not sure", no matches, or specifying the sources that it matches to
6) Pressing "Download reviews"

Please send the original `sources.geojson` file and the new `reviewed_targets.geojson` file for inclusion as a test case. File a [GH issue](https://github.com/nptscot/match_linestrings/issues/new), open a PR with the two files, email to <dabreegster@gmail.com>, etc.

## Example

...
