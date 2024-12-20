# Match linestrings

This crate takes a source and target dataset consisting of LineStrings in a Euclidean space, and matches every target LineString with the most similar source LineString.

This is an early experiment forked from the NPW codebase; use with caution. It depends on some not thoroughly tested georust additions for LineString slicing in https://github.com/a-b-street/utils/.

## Roadmap

- Get good results for all NPW use cases
- Compare with rnetmatch
- Make a web app to interactively tune params and explore the results
- Depend on stabler LineString slicing
- Add tests
- Publish to crates.io

## How to use

Stay tuned.

## How it works

The approach is inspired by. https://github.com/nptscot/rnetmatch.

For each target linestring:

-  Calculate the AABB around the target, then buffer it by some amount (Otherwise, sources slightly away might be skipped entirely)
-  Use an r-tree to find all source candidates in that AABB
-  Slice the candidate to match up with the target better. Find the closest point on the source to the target's start and endpoint, then slice in between them
-  Calculate the ratio of lengths between the candidates
-  Calculate the difference in angle from the start -> endpoint (accounting for 180 degree differences of linestrings pointing opposite ways)
-  Calculate the distance between midpoints
-  Check these 3 metrics against user-specified thresholds
-  If any of the candidates pass the thresholds, return a match, copying over user-provided data from the source. (Right now it doesn't look for the best match, just any match. Probably could combine the 3 metrics and take a minimum, if it's clear how to compare the tuple of 3 metrics or make a linear combo of them.)
