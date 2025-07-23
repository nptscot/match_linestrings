<script lang="ts">
  import { NavigationControl, ScaleControl } from "svelte-maplibre";
  import type { Map } from "maplibre-gl";

  export let map: Map | undefined;

  $: if (map) {
    map.keyboard.disableRotation();
    map.dragRotate.disable();
    map.touchZoomRotate.disableRotation();
  }
</script>

<NavigationControl showCompass={false} />
<ScaleControl />
