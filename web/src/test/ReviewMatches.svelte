<script lang="ts">
  import { autosaveKey, type TargetGJ, type Reviewed } from "./";
  import { onMount } from "svelte";
  import "@picocss/pico/css/pico.jade.min.css";
  import type { Map, MapMouseEvent } from "maplibre-gl";
  import {
    GeoJSON,
    MapLibre,
    LineLayer,
    hoverStateFilter,
    MapEvents,
    Popup,
    type LayerClickInfo,
  } from "svelte-maplibre";
  import { Layout } from "svelte-utils/two_column_layout";
  import { emptyGeojson, bbox } from "svelte-utils/map";
  import { notNull, downloadGeneratedFile } from "svelte-utils";
  import init from "backend";
  import Form from "./Form.svelte";
  import SetupMode from "./SetupMode.svelte";

  let map: Map | undefined;

  let sourceGj = emptyGeojson();
  let targetGj: TargetGJ = emptyGeojson() as TargetGJ;
  let setupDone = false;

  let clickedTarget: number | null = null;
  $: matchingSourceIndices =
    clickedTarget == null
      ? []
      : targetGj.features[clickedTarget].properties.matching_sources;

  $: numReviewed = targetGj.features.filter(
    (f) => f.properties.reviewed != "unreviewed",
  ).length;

  onMount(async () => {
    await init();

    try {
      let restore = window.localStorage.getItem(autosaveKey);
      if (restore) {
        let json = JSON.parse(restore);
        [sourceGj, targetGj] = json;
        setupDone = true;
        console.log(`Restored data from local storage ${autosaveKey}`);
      }
    } catch (err) {
      console.log(`Couldn't restore data from local storage: ${err}`);
    }
  });

  $: if (setupDone) {
    console.log(`Autosaving with ${numReviewed} reviewed targets`);
    window.localStorage.setItem(
      autosaveKey,
      JSON.stringify([sourceGj, targetGj]),
    );
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

  function onConfirm(value: Reviewed) {
    if (clickedTarget == null) {
      throw new Error("impossible");
    }
    targetGj.features[clickedTarget].properties.reviewed = value;
    targetGj = targetGj;
    clickedTarget = null;
  }

  function onMapClick(e: CustomEvent<MapMouseEvent>) {
    clickedTarget = null;
  }

  function gotoNext() {
    for (let f of targetGj.features) {
      if (f.properties.reviewed == "unreviewed") {
        map?.fitBounds(bbox(f), {
          duration: 600,
          // TODO Can't get per-edge to work
          padding: 200,
        });
        clickedTarget = f.id as number;
        return;
      }
    }
  }

  function onKeyDown(e: KeyboardEvent) {
    if ((e.target as HTMLElement).tagName == "INPUT" || !setupDone) {
      return;
    }

    if (e.key == "n") {
      gotoNext();
    }
  }

  function downloadReviewed() {
    downloadGeneratedFile("reviewed_targets.geojson", JSON.stringify(targetGj));
  }

  function backToSetup() {
    if (!window.confirm("Do you want to discard this review and start over?")) {
      return;
    }

    sourceGj = emptyGeojson();
    targetGj = emptyGeojson() as TargetGJ;
    setupDone = false;
    clickedTarget = null;
  }

  function onClickTarget(e: CustomEvent<LayerClickInfo>) {
    if (!setupDone) {
      return;
    }
    clickedTarget = e.detail.features[0].id as number;
  }
</script>

<svelte:window on:keydown={onKeyDown} />

<Layout>
  <div slot="left">
    <h1>Match LineStrings - review results to make test cases</h1>

    {#if !setupDone}
      <SetupMode bind:sourceGj bind:targetGj bind:setupDone {zoomFit} />
    {:else}
      <div style="display: flex; justify-content: space-between;">
        <button class="secondary" on:click={backToSetup}>Start over</button>
        <button class="secondary" on:click={zoomFit}>Zoom to fit</button>
      </div>

      <br />

      <progress value={numReviewed} max={targetGj.features.length} />
      <p>{numReviewed} / {targetGj.features.length} targets reviewed</p>
      <button on:click={downloadReviewed}>Download reviews</button>
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
      <MapEvents on:click={onMapClick} />

      {#if setupDone}
        <div class="map-panel">
          {#if clickedTarget == null}
            <button on:click={gotoNext}>
              Goto <kbd>n</kbd>
              ext unreviewed
            </button>
          {:else}
            <Form
              {clickedTarget}
              {targetGj}
              {matchingSourceIndices}
              {onConfirm}
            />
          {/if}
        </div>
      {/if}

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
            "line-color": "black",
            "line-opacity": hoverStateFilter(0.5, 1.0),
          }}
        >
          <Popup openOn="hover" let:data>
            Source {notNull(data).id}
          </Popup>
        </LineLayer>
      </GeoJSON>

      <GeoJSON data={targetGj}>
        <LineLayer
          manageHoverState
          paint={{
            "line-width": 8,
            "line-color": [
              "match",
              ["get", "reviewed"],
              "unreviewed",
              "red",
              "blue",
            ],
            "line-opacity": [
              "case",
              ["==", ["id"], clickedTarget ?? -1],
              1.0,
              ["boolean", ["feature-state", "hover"], false],
              0.6,
              [">", ["length", ["get", "matching_sources"]], 0],
              0.5,
              0.2,
            ],
          }}
          hoverCursor="pointer"
          on:click={onClickTarget}
        />
      </GeoJSON>
    </MapLibre>
  </div>
</Layout>

<style>
  .map-panel {
    position: absolute;
    top: 10px;
    left: 10%;
    right: 10%;

    background: grey;
    padding: 8px;
  }
</style>
