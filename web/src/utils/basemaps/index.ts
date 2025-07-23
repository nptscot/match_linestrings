import type { StyleSpecification } from "maplibre-gl";

export { default as Basemaps } from "./Basemaps.svelte";

let maptilerKey = "MZEJTanw3WpxRvt7qDfo";

// We always need these, even in blank or raster styles
let glyphs = `https://api.maptiler.com/fonts/{fontstack}/{range}.pbf?key=${maptilerKey}`;

export let basemapStyles: Record<string, string | StyleSpecification> = {
  Blank: {
    version: 8,
    sources: {},
    layers: [],
    glyphs,
  },
  "Maptiler Streets": `https://api.maptiler.com/maps/streets-v2/style.json?key=${maptilerKey}`,
  "Maptiler Dataviz": `https://api.maptiler.com/maps/dataviz/style.json?key=${maptilerKey}`,
  "Maptiler Hybrid": `https://api.maptiler.com/maps/hybrid/style.json?key=${maptilerKey}`,
  "Maptiler OpenStreetMap": `https://api.maptiler.com/maps/openstreetmap/style.json?key=${maptilerKey}`,
  "ESRI World Imagery": {
    version: 8,
    sources: {
      "raster-tiles": {
        type: "raster",
        tiles: [
          "https://server.arcgisonline.com/ArcGIS/rest/services/World_Imagery/MapServer/tile/{z}/{y}/{x}",
        ],
        tileSize: 256,
        // See https://wiki.openstreetmap.org/wiki/Esri
        attribution:
          "ESRI &copy; <a href='http://www.esri.com' target='_blank'>ESRI</a>",
        minzoom: 0,
        maxzoom: 18,
      },
    },
    layers: [
      {
        id: "raster-basemap",
        type: "raster",
        source: "raster-tiles",
      },
    ],
    glyphs,
  },
};
