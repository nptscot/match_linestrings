<script lang="ts">
  import { onMount } from "svelte";
  import "@picocss/pico/css/pico.jade.min.css";
  import type { Map, MapMouseEvent } from "maplibre-gl";
  import {
    GeoJSON,
    MapLibre,
    LineLayer,
    hoverStateFilter,
    MapEvents,
  } from "svelte-maplibre";
  import { Layout } from "svelte-utils/two_column_layout";
  import { Popup, emptyGeojson, bbox } from "svelte-utils/map";
  import type { FeatureCollection } from "geojson";
  import init, { matchLineStrings } from "backend";

  let loaded = false;

  let pinnedColor = "green";
  let matchColor = "cyan";

  let sourceGj = emptyGeojson();
  let sourceColor = "red";
  let sourceOpacity = 0.8;

  let targetGj = emptyGeojson();
  let targetColor = "blue";
  let targetOpacity = 0.5;
  let pinnedTargetIdx: number | null = null;

  let map: Map | undefined;

  onMount(async () => {
    await init();
    loaded = true;
  });

  $: matches = recalculate(loaded, sourceGj, targetGj);
  $: matchSourceIdx = pinnedTargetIdx == null ? null : matches[pinnedTargetIdx];

  function recalculate(
    loaded: boolean,
    sourceGj: FeatureCollection,
    targetGj: FeatureCollection,
  ): [number | null][] {
    if (
      !loaded ||
      sourceGj.features.length == 0 ||
      targetGj.features.length == 0
    ) {
      return [];
    }
    try {
      return JSON.parse(
        matchLineStrings(JSON.stringify(sourceGj), JSON.stringify(targetGj)),
      );
    } catch (err) {
      window.alert(`Bug: ${err}`);
      return [];
    }
  }

  function onMapClick(e: MapMouseEvent) {
    pinnedTargetIdx = null;
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
      pinnedTargetIdx = null;
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
    [sourceOpacity, targetOpacity] = [targetOpacity, sourceOpacity];
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

      <div style:background={sourceColor}>Source</div>
      <label>
        Opacity:
        <input
          type="range"
          min="0.0"
          max="1.0"
          step="0.1"
          bind:value={sourceOpacity}
        />
      </label>

      <div style:background={targetColor}>Target</div>
      <label>
        Opacity:
        <input
          type="range"
          min="0.0"
          max="1.0"
          step="0.1"
          bind:value={targetOpacity}
        />
      </label>
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
      <MapEvents on:click={onMapClick} />

      <GeoJSON data={sourceGj}>
        <LineLayer
          manageHoverState
          eventsIfTopMost
          hoverCursor="pointer"
          paint={{
            "line-width": 8,
            "line-color": [
              "case",
              ["==", ["id"], matchSourceIdx ?? -1],
              matchColor,
              sourceColor,
            ],
            "line-opacity": hoverStateFilter(sourceOpacity, 1.0),
          }}
        >
          <Popup openOn="click" let:props>
            <pre>{JSON.stringify(props)}</pre>
          </Popup>
        </LineLayer>
      </GeoJSON>

      <GeoJSON data={targetGj}>
        <LineLayer
          manageHoverState
          eventsIfTopMost
          hoverCursor="pointer"
          paint={{
            "line-width": 8,
            "line-color": [
              "case",
              ["==", ["id"], pinnedTargetIdx ?? -1],
              pinnedColor,
              targetColor,
            ],
            "line-opacity": hoverStateFilter(targetOpacity, 1.0),
          }}
          on:click={(e) => (pinnedTargetIdx = e.detail.features[0].id)}
        />
      </GeoJSON>
    </MapLibre>
  </div>
</Layout>
