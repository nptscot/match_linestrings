<script lang="ts">
  import type { TargetGJ } from "./";
  import type { FeatureCollection } from "geojson";
  import { matchLineStrings } from "backend";
  import Settings from "../Settings.svelte";

  export let sourceGj: FeatureCollection;
  export let targetGj: TargetGJ;
  export let setupDone: boolean;
  export let zoomFit: () => void;

  let options = {
    buffer_meters: 20.0,
    angle_diff_threshold: 10.0,
    length_ratio_threshold: 1.1,
    midpt_dist_threshold: 15.0,
  };

  function recalculate() {
    try {
      let matches = JSON.parse(
        matchLineStrings(
          JSON.stringify(sourceGj),
          JSON.stringify(targetGj),
          options,
        ),
      );

      for (let [idx, f] of targetGj.features.entries()) {
        f.properties.matching_sources = matches[idx].matching_sources;
        f.properties.reviewed ??= "unreviewed";
      }
      targetGj = targetGj;
    } catch (err) {
      window.alert(`Bug: ${err}`);
    }
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
      // @ts-expect-error
      targetGj = fixInput(JSON.parse(input2));
      zoomFit();
      recalculate();
    } catch (err) {
      window.alert(`Bad input file: ${err}`);
    }
  }

  function swap() {
    // @ts-expect-error TODO Actually do remove from sources
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

<label class="form-label">
  Load two .geojson files
  <input
    class="form-control"
    bind:this={fileInput}
    on:change={loadFiles}
    type="file"
    multiple
  />
</label>

{#if sourceGj.features.length > 0}
  <button class="btn btn-secondary" on:click={swap}>Swap</button>

  <p>
    {sourceGj.features.length} sources and
    <span style="color: red">{targetGj.features.length} targets</span>
    , with {targetGj.features.filter(
      (f) => f.properties.matching_sources.length > 0,
    ).length} matching a source
  </p>

  <button class="btn btn-primary mb-5" on:click={() => (setupDone = true)}>
    Start review
  </button>

  <Settings bind:options onChange={recalculate} />
{/if}
