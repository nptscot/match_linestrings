<script lang="ts">
  import type { Reviewed, TargetGJ } from "./";

  export let clickedTarget: number;
  export let targetGj: TargetGJ;
  export let onConfirm: (value: Reviewed) => void;
  export let onCancel: () => void;

  $: props = targetGj.features[clickedTarget].properties;

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

<div style="display: flex; justify-content: space-between;">
  <h3>
    Target {clickedTarget}:
    {#if props.matching_sources.length > 0}
      {JSON.stringify(props.matching_sources)}
    {:else}
      no matches (off-road)
    {/if}
  </h3>
  <button class="btn btn-secondary" on:click={onCancel}>
    <kbd>Escape</kbd>
    - Back
  </button>
</div>

<h6>Click sources to add or remove</h6>

<div style="display: flex; justify-content: space-between;">
  <button
    class="btn btn-danger"
    class:current={props.reviewed == "unreviewed"}
    on:click={() => onConfirm("unreviewed")}
  >
    <kbd>1</kbd>
    - Unreviewed
  </button>

  <button
    class="btn btn-warning"
    class:current={props.reviewed == "not sure"}
    on:click={() => onConfirm("not sure")}
  >
    <kbd>2</kbd>
    - Not sure
  </button>

  <button
    class="btn btn-success"
    class:current={props.reviewed == "confirmed"}
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
