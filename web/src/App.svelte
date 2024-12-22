<script lang="ts">
  import { onMount } from "svelte";
  import "@picocss/pico/css/pico.jade.min.css";
  import type { Map } from "maplibre-gl";
  import {
    GeoJSON,
    MapLibre,
    LineLayer,
    hoverStateFilter,
  } from "svelte-maplibre";
  import { Layout } from "svelte-utils/two_column_layout";
  import { emptyGeojson, bbox } from "svelte-utils/map";
  import type { Feature, FeatureCollection } from "geojson";
  import init, { matchLineStrings } from "backend";
  import Settings from "./Settings.svelte";

  let map: Map | undefined;

  let sourceGj = emptyGeojson();
  let sourceColor = "red";

  let targetGj = emptyGeojson();
  let targetColor = "blue";
  let hoveredTarget: Feature | null = null;

  let options = {
    buffer_meters: 20.0,
    angle_diff_threshold: 10.0,
    length_ratio_threshold: 1.1,
    midpt_dist_threshold: 15.0,
  };

  let matches: [number | null][] = [];

  onMount(async () => {
    await init();
  });

  $: matchSourceIdx = hoveredTarget == null ? null : matches[hoveredTarget.id];

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
      f.properties.has_match = matches[idx] != null;
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
          (x) => x != null,
        ).length} matching a source
      </p>

      <Settings bind:options onChange={recalculate} />
    {/if}
  </div>

  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style="https://api.maptiler.com/maps/dataviz/style.json?key=MZEJTanw3WpxRvt7qDfo"
      standardControls
      bind:map
      on:error={(e) => {
        // @ts-ignore ErrorEvent isn't exported
        console.log(e.detail.error);
      }}
    >
      <GeoJSON data={sourceGj}>
        <LineLayer
          manageHoverState
          paint={{
            "line-width": ["case", ["==", ["id"], matchSourceIdx ?? -1], 8, 5],
            "line-color": sourceColor,
            "line-opacity": hoverStateFilter(0.5, 1.0),
          }}
        />
      </GeoJSON>

      <GeoJSON data={targetGj}>
        <LineLayer
          manageHoverState
          paint={{
            "line-width": 8,
            "line-color": targetColor,
            "line-opacity": hoverStateFilter(
              ["case", ["get", "has_match"], 0.5, 0.2],
              1.0,
            ),
          }}
          bind:hovered={hoveredTarget}
        />
      </GeoJSON>
    </MapLibre>
  </div>
</Layout>
