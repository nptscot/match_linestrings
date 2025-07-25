<script lang="ts">
  import type { TargetGJ } from "./";
  import type { FeatureCollection } from "geojson";
  import * as backend from "../../../backend/pkg";
  import Settings from "../Settings.svelte";

  interface Props {
    sourceGj: FeatureCollection;
    targetGj: TargetGJ;
    setupDone: boolean;
    zoomFit: () => void;
  }

  let {
    sourceGj = $bindable(),
    targetGj = $bindable(),
    setupDone = $bindable(),
    zoomFit,
  }: Props = $props();

  let alreadyHasMatching = $state(false);

  let options = $state({
    buffer_meters: 20.0,
    angle_diff_threshold: 10.0,
    length_ratio_threshold: 1.1,
    midpt_dist_threshold: 15.0,
  });

  function recalculate() {
    if (alreadyHasMatching) {
      return;
    }
    try {
      let matches = JSON.parse(
        backend.matchLineStrings(
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

  let fileInput: HTMLInputElement | undefined = $state();
  async function loadFiles(e: Event) {
    if (!fileInput?.files) {
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
      if ("matching_sources" in targetGj.features[0].properties) {
        alreadyHasMatching = true;
      } else if ("matching_sources" in sourceGj.features[0].properties!) {
        alreadyHasMatching = true;
        // @ts-expect-error
        [sourceGj, targetGj] = [targetGj, sourceGj];
      }

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
    onchange={loadFiles}
    type="file"
    multiple
  />
</label>

{#if sourceGj.features.length > 0}
  <button
    class="btn btn-secondary"
    onclick={swap}
    disabled={alreadyHasMatching}
  >
    Swap
  </button>

  <p>
    {sourceGj.features.length} sources and
    {targetGj.features.length} targets , with {targetGj.features.filter(
      (f) => f.properties.matching_sources.length > 0,
    ).length} matching a source
  </p>

  <button class="btn btn-primary mb-5" onclick={() => (setupDone = true)}>
    Start review
  </button>

  {#if !alreadyHasMatching}
    <Settings bind:options onChange={recalculate} />
  {/if}
{/if}
