<script lang="ts">
  import type { Reviewed, TargetGJ } from "./";

  export let clickedTarget: number;
  export let targetGj: TargetGJ;
  export let onConfirm: (value: Reviewed) => void;
  export let onCancel: () => void;

  $: currentValue = targetGj.features[clickedTarget].properties.reviewed;

  function onKeyDown(e: KeyboardEvent) {
    if ((e.target as HTMLElement).tagName == "INPUT") {
      return;
    }

    if (e.key == "1") {
      onConfirm("unreviewed");
    } else if (e.key == "2") {
      onConfirm("not sure");
    } else if (e.key == "3") {
      onConfirm("confirmed");
    } else if (e.key == "Escape") {
      onCancel();
    }
  }
</script>

<svelte:window on:keydown={onKeyDown} />

<h3>
  Target {clickedTarget}: {JSON.stringify(
    targetGj.features[clickedTarget].properties.matching_sources,
  )}
</h3>

<h6>Click sources to add or remove</h6>

<div style="display: flex; justify-content: space-between;">
  <button
    class="btn btn-danger"
    class:current={currentValue == "unreviewed"}
    on:click={() => onConfirm("unreviewed")}
  >
    <kbd>1</kbd>
    - Unreviewed
  </button>

  <button
    class="btn btn-warning"
    class:current={currentValue == "not sure"}
    on:click={() => onConfirm("not sure")}
  >
    <kbd>2</kbd>
    - Not sure
  </button>

  <button
    class="btn btn-success"
    class:current={currentValue == "confirmed"}
    on:click={() => onConfirm("confirmed")}
  >
    <kbd>3</kbd>
    - Confirm correct
  </button>
</div>

<style>
  .current {
    font-weight: bold;
    border: 3px solid black;
  }
</style>
