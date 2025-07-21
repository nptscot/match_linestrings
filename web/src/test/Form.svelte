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

  function onKeyDown(e: KeyboardEvent) {
    if ((e.target as HTMLElement).tagName == "INPUT") {
      return;
    }

    if (e.key == "1") {
      onConfirm("unreviewed");
    } else if (e.key == "2") {
      onConfirm("not sure");
    } else if (e.key == "3") {
      onConfirm([]);
    } else if (e.key == "4") {
      parseMatches();
    }
  }
</script>

<svelte:window on:keydown={onKeyDown} />

<h3>Target {clickedTarget}: {initialValue}</h3>

<div style="display: flex; justify-content: space-between;">
  <button on:click={() => onConfirm("unreviewed")}>
    <kbd>1</kbd>
    - Unreviewed
  </button>

  <button on:click={() => onConfirm("not sure")}>
    <kbd>2</kbd>
    - Not sure
  </button>

  <button on:click={() => onConfirm([])}>
    <kbd>3</kbd>
    - No matches
  </button>

  <div>
    <div>
      <button on:click={parseMatches}>
        <kbd>4</kbd>
        - These sources
      </button>
    </div>
    <input type="text" bind:value={rawList} style:width="50%" />
  </div>
</div>
