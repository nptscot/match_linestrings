<script lang="ts">
  export let items: number;
  export let total: number;
  export let color: string;
</script>

<div class="row">
  <div
    style:position="absolute"
    style:width={`${(100 * items) / total}%`}
    style:height="100%"
    style:background-color={color}
  />
  <div style:position="relative"><slot /></div>
</div>

<style>
  .row {
    width: 100%;
    border: 1px solid black;
    position: relative;
  }
</style>
