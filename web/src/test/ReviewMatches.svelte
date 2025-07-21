<script lang="ts">
  import { onMount } from "svelte";
  import "@picocss/pico/css/pico.jade.min.css";
  import type { Map } from "maplibre-gl";
  import {
    GeoJSON,
    MapLibre,
    LineLayer,
    hoverStateFilter,
    Popup,
    type LayerClickInfo,
  } from "svelte-maplibre";
  import { Layout } from "svelte-utils/two_column_layout";
  import { emptyGeojson, bbox } from "svelte-utils/map";
  import { notNull, downloadGeneratedFile } from "svelte-utils";
  import type { Feature, FeatureCollection } from "geojson";
  import init, { matchLineStrings } from "backend";
  import Settings from "../Settings.svelte";

  let map: Map | undefined;

  let sourceGj = emptyGeojson();

  let targetGj = emptyGeojson();
  let hoveredTarget: Feature | null = null;

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

    // Quick local development
    if (true) {
      sourceGj = fixInput(await (await fetch("/sources.geojson")).json());
      targetGj = fixInput(await (await fetch("/targets.geojson")).json());
      zoomFit();
      hoveredTarget = null;
      recalculate();
    }
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

    // Modify targetGj, so we can style based on matches and mark results
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
      let input1 = await fileInput.files[0].text();
      let input2 = await fileInput.files[1].text();
      sourceGj = fixInput(JSON.parse(input1));
      targetGj = fixInput(JSON.parse(input2));
      zoomFit();
      hoveredTarget = null;
      recalculate();
    } catch (err) {
      window.alert(`Bad input file: ${err}`);
    }
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

  function fixInput(gj: FeatureCollection): FeatureCollection {
    // Overwrite feature IDs
    let id = 0;
    for (let f of gj.features) {
      f.id = id++;
      // Make sure properties aren't null
      f.properties ??= {};
    }
    return gj;
  }
</script>

<Layout>
  <div slot="left">
    <h1>Match LineStrings - review results to make test cases</h1>

    <label>
      Load two .geojson files
      <input bind:this={fileInput} on:change={loadFiles} type="file" multiple />
    </label>

    {#if sourceGj.features.length > 0}
      <div style="display: flex; justify-content: space-between;">
        <button class="secondary" on:click={swap}>Swap</button>
        <button class="secondary" on:click={zoomFit}>Zoom to fit</button>
      </div>

      <hr />

      <p>
        {sourceGj.features.length} sources and
        <span style="color: red">{targetGj.features.length} targets</span>
        , with {matches.filter((x) => x.matching_sources.length > 0).length} matching
        a source
      </p>

      <Settings bind:options onChange={recalculate} open={false} />
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
            "line-color": "red",
            "line-opacity": hoverStateFilter(
              ["case", ["get", "has_match"], 0.5, 0.2],
              1.0,
            ),
          }}
          bind:hovered={hoveredTarget}
        >
          <Popup openOn="hover" let:data>
            Target {notNull(data).id} matches {JSON.stringify(
              matchingSourceIndices,
            )}
          </Popup>
        </LineLayer>
      </GeoJSON>
    </MapLibre>
  </div>
</Layout>
