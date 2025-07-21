import type { FeatureCollection, LineString } from "geojson";

export type Reviewed = "unreviewed" | "not sure" | number[];
export type TargetGJ = FeatureCollection<
  LineString,
  { has_match: boolean; reviewed: Reviewed }
>;
