<script lang="ts">
  import type { Reviewed, TargetGJ } from "./";

  interface Props {
    clickedTarget: number;
    targetGj: TargetGJ;
    onConfirm: (value: Reviewed) => void;
    onCancel: () => void;
  }

  let { clickedTarget, targetGj, onConfirm, onCancel }: Props = $props();

  let targetProps = $derived(targetGj.features[clickedTarget].properties);

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

<svelte:window onkeydown={onKeyDown} />

<div style="display: flex; justify-content: space-between;">
  <h3>
    Target {clickedTarget}:
    {#if targetProps.matching_sources.length > 0}
      {JSON.stringify(targetProps.matching_sources)}
    {:else}
      no matches (off-road)
    {/if}
  </h3>
  <button class="btn btn-secondary" onclick={onCancel}>
    <kbd>Escape</kbd>
    - Back
  </button>
</div>

<h6>Click sources to add or remove</h6>

<div style="display: flex; justify-content: space-between;">
  <button
    class="btn btn-danger"
    class:current={targetProps.reviewed == "unreviewed"}
    onclick={() => onConfirm("unreviewed")}
  >
    <kbd>1</kbd>
    - Unreviewed
  </button>

  <button
    class="btn btn-warning"
    class:current={targetProps.reviewed == "not sure"}
    onclick={() => onConfirm("not sure")}
  >
    <kbd>2</kbd>
    - Not sure
  </button>

  <button
    class="btn btn-success"
    class:current={targetProps.reviewed == "confirmed"}
    onclick={() => onConfirm("confirmed")}
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
