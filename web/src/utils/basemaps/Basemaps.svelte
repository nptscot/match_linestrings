<script lang="ts">
  import type { StyleSpecification } from "maplibre-gl";

  import { basemapStyles } from "./";

  // Input
  export let choice: string;
  // Output
  export let style: string | StyleSpecification = basemapStyles[choice];

  $: style = basemapStyles[choice];
</script>

<div>
  <!-- svelte-ignore a11y-missing-content -->
  <h5 class="fa-solid fa-layer-group me-2 align-self-center"></h5>
  <select class="form-select" bind:value={choice}>
    {#each Object.keys(basemapStyles) as value}
      <option {value}>{value}</option>
    {/each}
  </select>
</div>

<style>
  div {
    position: absolute;
    bottom: 60px;
    left: 10px;

    background: white;
    padding: 4px;

    display: flex;
  }
</style>
