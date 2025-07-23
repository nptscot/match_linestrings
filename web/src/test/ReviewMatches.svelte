<script lang="ts">
  import "bootstrap/dist/css/bootstrap.min.css";
  import "@fortawesome/fontawesome-free/css/all.min.css";
  import { autosaveKey, type TargetGJ, type Reviewed } from "./";
  import { onMount } from "svelte";
  import type { Map } from "maplibre-gl";
  import {
    GeoJSON,
    MapLibre,
    LineLayer,
    hoverStateFilter,
    SymbolLayer,
    Popup,
    type LayerClickInfo,
  } from "svelte-maplibre";
  import { Layout } from "svelte-utils/two_column_layout";
  import { emptyGeojson, bbox } from "svelte-utils/map";
  import {
    notNull,
    downloadGeneratedFile,
    QualitativeLegend,
  } from "svelte-utils";
  import * as backend from "../../../backend/pkg";
  import Form from "./Form.svelte";
  import SetupMode from "./SetupMode.svelte";

  let map: Map | undefined;

  let sourceGj = emptyGeojson();
  let targetGj: TargetGJ = emptyGeojson() as TargetGJ;
  let setupDone = false;
  let showLabels = false;

  let clickedTarget: number | null = null;
  $: matchingSourceIndices =
    clickedTarget == null
      ? []
      : targetGj.features[clickedTarget].properties.matching_sources;

  $: numReviewed = targetGj.features.filter(
    (f) => f.properties.reviewed != "unreviewed",
  ).length;

  onMount(async () => {
    await backend.default();

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
    showLabels = false;
    window.localStorage.removeItem(autosaveKey);
  }

  function onClickTarget(e: CustomEvent<LayerClickInfo>) {
    if (!setupDone) {
      return;
    }
    clickedTarget = e.detail.features[0].id as number;
  }

  function onClickSource(e: CustomEvent<LayerClickInfo>) {
    if (!setupDone || clickedTarget == null) {
      return;
    }
    let id = e.detail.features[0].id as number;
    let props = targetGj.features[clickedTarget].properties;
    if (props.matching_sources.includes(id)) {
      props.matching_sources = props.matching_sources.filter((x) => x != id);
    } else {
      props.matching_sources.push(id);
    }
    targetGj = targetGj;
  }
</script>

<svelte:window on:keydown={onKeyDown} />

<Layout>
  <div slot="left">
    <h1>Manually specify LineString matchings for test cases</h1>

    <div>
      <a
        class="icon-link mb-3"
        href="https://github.com/nptscot/match_linestrings/blob/main/tests/README.md"
        target="_blank"
      >
        About <i class="fa-solid fa-arrow-up-right-from-square" />
      </a>
    </div>

    {#if !setupDone}
      <SetupMode bind:sourceGj bind:targetGj bind:setupDone {zoomFit} />
    {:else}
      <div style="display: flex; justify-content: space-between;">
        <button class="btn btn-outline-danger" on:click={backToSetup}>
          Start over
        </button>
        <button class="btn btn-outline-secondary" on:click={zoomFit}>
          Zoom to fit
        </button>
      </div>

      <br />

      <div
        class="progress"
        role="progressbar"
        aria-valuenow={numReviewed}
        aria-valuemin="0"
        aria-valuemax={targetGj.features.length}
      >
        <div
          class="progress-bar"
          style:width={`${(100 * numReviewed) / targetGj.features.length}%`}
        />
      </div>
      <p>{numReviewed} / {targetGj.features.length} targets reviewed</p>

      <button class="btn btn-outline-success" on:click={downloadReviewed}>
        Download reviews
      </button>

      <label class="form-check-label mt-5">
        <input
          class="form-check-input"
          type="checkbox"
          bind:checked={showLabels}
        />
        Show labels for matches
      </label>
      <div class="card">
        <div class="card-body">
          <h5 class="card-title">Legend</h5>
          <QualitativeLegend
            itemsPerRow={1}
            labelColors={{
              Confirmed: "green",
              "Not sure": "orange",
              Unreviewed: "red",
              Source: "grey",
            }}
          />
          <p>Thinner lines have a match, thicker are off-road</p>
        </div>
      </div>
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
      {#if setupDone}
        <div class="map-panel">
          {#if clickedTarget == null}
            <button class="btn btn-primary" on:click={gotoNext}>
              Goto <kbd>n</kbd>
              ext unreviewed
            </button>
          {:else}
            <Form
              {clickedTarget}
              {targetGj}
              {onConfirm}
              onCancel={() => (clickedTarget = null)}
            />
          {/if}
        </div>
      {/if}

      <GeoJSON data={sourceGj}>
        <LineLayer
          manageHoverState
          paint={{
            "line-color": [
              "case",
              ["in", ["id"], ["literal", matchingSourceIndices]],
              "blue",
              "black",
            ],
            "line-width": 8,
            "line-opacity": hoverStateFilter(0.5, 1.0),
          }}
          hoverCursor={clickedTarget == null ? undefined : "pointer"}
          on:click={onClickSource}
        >
          {#if clickedTarget != null}
            <Popup openOn="hover" let:data>
              Source {notNull(data).id} -- click to {matchingSourceIndices.includes(
                notNull(data).id,
              )
                ? "remove"
                : "add"}
            </Popup>
          {/if}
        </LineLayer>

        <SymbolLayer
          paint={{
            "text-color": "white",
            "text-halo-color": "grey",
            "text-halo-width": 4,
          }}
          layout={{
            "text-field": ["to-string", ["id"]],
            "text-size": 16,
            "symbol-placement": "line",
            visibility: showLabels ? "visible" : "none",
          }}
        />
      </GeoJSON>

      <GeoJSON data={targetGj}>
        <LineLayer
          manageHoverState
          paint={{
            "line-width": [
              "case",
              [">", ["length", ["get", "matching_sources"]], 0],
              8,
              16,
            ],
            "line-color": [
              "match",
              ["get", "reviewed"],
              "unreviewed",
              "red",
              "not sure",
              "orange",
              "green",
            ],
            "line-opacity": [
              "case",
              ["==", ["id"], clickedTarget ?? -1],
              1.0,
              ["boolean", ["feature-state", "hover"], false],
              0.6,
              0.3,
            ],
          }}
          hoverCursor="pointer"
          on:click={onClickTarget}
        />

        <SymbolLayer
          paint={{
            "text-color": "white",
            "text-halo-color": [
              "match",
              ["get", "reviewed"],
              "unreviewed",
              "red",
              "not sure",
              "orange",
              "green",
            ],
            "text-halo-width": 4,
          }}
          layout={{
            "text-field": [
              "case",
              [">", ["length", ["get", "matching_sources"]], 0],
              ["to-string", ["get", "matching_sources"]],
              "off-road",
            ],
            "text-size": 16,
            "symbol-placement": "line",
            visibility: showLabels ? "visible" : "none",
          }}
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

    background: white;
    border: 1px solid black;
    padding: 16px;
  }
</style>
