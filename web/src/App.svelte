<script lang="ts">
  import "@picocss/pico/css/pico.jade.min.css";
  import type { Map } from "maplibre-gl";
  import {
    GeoJSON,
    MapLibre,
    LineLayer,
    hoverStateFilter,
  } from "svelte-maplibre";
  import { Layout } from "svelte-utils/two_column_layout";
  import { Popup, emptyGeojson, bbox } from "svelte-utils/map";
  import type { FeatureCollection } from "geojson";

  let sourceGj = emptyGeojson();
  let sourceColor = "red";
  let sourceOpacity = 0.8;

  let targetGj = emptyGeojson();
  let targetColor = "blue";
  let targetOpacity = 0.5;

  let map: Map | undefined;

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
    } catch (err) {
      window.alert(`Bad input file: ${err}`);
    }
  }

  async function loadFile(file: File): Promise<FeatureCollection> {
    let text = await file.text();
    let gj = JSON.parse(text);

    // Overwrite feature IDs
    let id = 1;
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
          eventsIfTopMost
          hoverCursor="pointer"
          paint={{
            "line-width": 8,
            "line-color": sourceColor,
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
            "line-color": targetColor,
            "line-opacity": hoverStateFilter(targetOpacity, 1.0),
          }}
        >
          <Popup openOn="click" let:props>
            <pre>{JSON.stringify(props)}</pre>
          </Popup>
        </LineLayer>
      </GeoJSON>
    </MapLibre>
  </div>
</Layout>
