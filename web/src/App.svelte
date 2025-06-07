<script lang="ts">
  import { bboxPolygon } from "@turf/bbox-polygon";
  import { booleanIntersects } from "@turf/boolean-intersects";
  import { onMount } from "svelte";
  import "@picocss/pico/css/pico.jade.min.css";
  import type { Map } from "maplibre-gl";
  import {
    GeoJSON,
    MapLibre,
    LineLayer,
    hoverStateFilter,
    type LayerClickInfo,
  } from "svelte-maplibre";
  import { Layout } from "svelte-utils/two_column_layout";
  import { emptyGeojson, bbox } from "svelte-utils/map";
  import { downloadGeneratedFile } from "svelte-utils";
  import type { Feature, FeatureCollection } from "geojson";
  import init, { matchLineStrings } from "backend";
  import Settings from "./Settings.svelte";

  let map: Map | undefined;

  let sourceGj = emptyGeojson();
  let sourceColor = "red";

  let targetGj = emptyGeojson();
  let targetColor = "blue";
  let hoveredTarget: Feature | null = null;
  let showTargetsWithMatches = true;

  let options = {
    buffer_meters: 20.0,
    angle_diff_threshold: 10.0,
    length_ratio_threshold: 1.1,
    midpt_dist_threshold: 15.0,
  };

  interface TargetMatches {
    // Indices of sources matching this target
    matching_sources: number[];
  }
  let matches: TargetMatches[] = [];

  onMount(async () => {
    await init();
  });

  $: matchingSourceIndices =
    hoveredTarget == null
      ? []
      : matches[hoveredTarget.id as number].matching_sources;

  function recalculate() {
    try {
      matches = JSON.parse(
        matchLineStrings(
          JSON.stringify(sourceGj),
          JSON.stringify(targetGj),
          options,
        ),
      );
    } catch (err) {
      window.alert(`Bug: ${err}`);
      return;
    }

    // Modify targetGj, so we can style based on matches
    for (let [idx, f] of targetGj.features.entries()) {
      f.properties!.has_match = matches[idx].matching_sources.length > 0;
    }
    targetGj = targetGj;
  }

  let fileInput: HTMLInputElement;
  async function loadFiles(e: Event) {
    if (!fileInput.files) {
      return;
    }
    let len = fileInput.files.length;
    if (len != 2) {
      window.alert("Select two GeoJSON files");
      return;
    }

    try {
      sourceGj = await loadFile(fileInput.files[0]);
      targetGj = await loadFile(fileInput.files[1]);
      zoomFit();
      hoveredTarget = null;
      recalculate();
    } catch (err) {
      window.alert(`Bad input file: ${err}`);
    }
  }

  async function loadFile(file: File): Promise<FeatureCollection> {
    let text = await file.text();
    let gj = JSON.parse(text);

    // Overwrite feature IDs
    let id = 0;
    for (let f of gj.features) {
      f.id = id++;
      // Make sure properties aren't null
      f.properties ??= {};
    }

    return gj;
  }

  function zoomFit() {
    let gj = {
      type: "FeatureCollection" as const,
      features: [...sourceGj.features, ...targetGj.features],
    };
    map?.fitBounds(bbox(gj), {
      animate: false,
      padding: 10,
    });
  }

  function swap() {
    [sourceGj, targetGj] = [targetGj, sourceGj];
    recalculate();
  }

  function removeProperties(f: Feature): Feature {
    let copy = JSON.parse(JSON.stringify(f));
    delete copy.properties;
    return copy;
  }

  function resetIDs(features: Feature[]): Feature[] {
    let id = 0;
    for (let f of features) {
      f.id = id++;
    }
    return features;
  }

  function generateTestCase(e: CustomEvent<LayerClickInfo>) {
    let clickedTarget = targetGj.features[e.detail.features[0].id as number];
    let box = bboxPolygon(bbox(clickedTarget));

    downloadGeneratedFile(
      "sources.geojson",
      JSON.stringify({
        type: "FeatureCollection",
        features: resetIDs(
          sourceGj.features
            .filter((f) => booleanIntersects(f, box))
            .map(removeProperties),
        ),
      }),
    );
    downloadGeneratedFile(
      "targets.geojson",
      JSON.stringify({
        type: "FeatureCollection",
        features: resetIDs(
          targetGj.features
            .filter((f) => booleanIntersects(f, box))
            .map(removeProperties),
        ),
      }),
    );
  }
</script>

<Layout>
  <div slot="left">
    <h1>Match LineStrings</h1>

    <label>
      Load two .geojson files
      <input bind:this={fileInput} on:change={loadFiles} type="file" multiple />
    </label>

    {#if sourceGj.features.length > 0}
      <hr />

      <div>
        <button on:click={swap}>Swap</button>
      </div>
      <div><button on:click={zoomFit}>Zoom to fit</button></div>

      <div style:background={sourceColor}>Sources</div>
      <p>{sourceGj.features.length} sources</p>

      <div style:background={targetColor}>Target</div>
      <p>
        {targetGj.features.length} targets, with {matches.filter(
          (x) => x.matching_sources.length > 0,
        ).length} matching a source
      </p>
      <label>
        <input type="checkbox" bind:checked={showTargetsWithMatches} />
        Show targets matching a source
      </label>

      <Settings bind:options onChange={recalculate} />
    {/if}
  </div>

  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style="https://api.maptiler.com/maps/dataviz/style.json?key=MZEJTanw3WpxRvt7qDfo"
      standardControls
      bind:map
      hash
      on:error={(e) => {
        // @ts-ignore ErrorEvent isn't exported
        console.log(e.detail.error);
      }}
    >
      <GeoJSON data={sourceGj}>
        <LineLayer
          manageHoverState
          paint={{
            "line-width": [
              "case",
              ["in", ["id"], ["literal", matchingSourceIndices]],
              8,
              5,
            ],
            "line-color": sourceColor,
            "line-opacity": hoverStateFilter(0.5, 1.0),
          }}
        />
      </GeoJSON>

      <GeoJSON data={targetGj}>
        <LineLayer
          manageHoverState
          filter={showTargetsWithMatches
            ? undefined
            : ["!", ["get", "has_match"]]}
          paint={{
            "line-width": 8,
            "line-color": targetColor,
            "line-opacity": hoverStateFilter(
              ["case", ["get", "has_match"], 0.5, 0.2],
              1.0,
            ),
          }}
          bind:hovered={hoveredTarget}
          on:click={generateTestCase}
        />
      </GeoJSON>
    </MapLibre>
  </div>
</Layout>
