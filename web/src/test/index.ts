import type { FeatureCollection, LineString } from "geojson";

export type Reviewed = "unreviewed" | "not sure" | "confirmed";
export type TargetGJ = FeatureCollection<
  LineString,
  { matching_sources: number[]; reviewed: Reviewed }
>;

export let autosaveKey = "match-linestrings-review";
