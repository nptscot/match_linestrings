<script lang="ts">
  import type { Reviewed, TargetGJ } from "./";

  export let clickedTarget: number;
  export let targetGj: TargetGJ;
  export let matchingSourceIndices: number[];
  export let onConfirm: (value: Reviewed) => void;

  let initialValue = targetGj.features[clickedTarget].properties.reviewed;

  let rawList = Array.isArray(initialValue)
    ? JSON.stringify(initialValue)
    : JSON.stringify(matchingSourceIndices);

  function parseMatches() {
    try {
      let list = JSON.parse(rawList);
      if (!Array.isArray(list)) {
        throw new Error("Not an array");
      }
      onConfirm(list);
    } catch (err) {
      window.alert("Not a list of source IDs");
    }
  }
</script>

<p>Target {clickedTarget}: {initialValue || "unreviewed"}</p>

<button on:click={() => onConfirm("")}>Mark unreviewed</button>

<button on:click={() => onConfirm("not sure")}>Not sure</button>

<button on:click={() => onConfirm([])}>No matches</button>

<input type="text" bind:value={rawList} />
<button on:click={parseMatches}>Matches these sources</button>
