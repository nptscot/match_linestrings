<script lang="ts">
  export let options: any;
  export let onChange: () => void;
</script>

<details open>
  <summary>Config</summary>

  <label>
    Expand the bounding box around each target by this many meters in all
    directions

    <input
      type="number"
      bind:value={options.buffer_meters}
      min="0"
      max="500"
      step="1"
      on:change={onChange}
    />
  </label>

  <label>
    How many degrees difference allowed between matches? (LineStrings pointing
    in opposite directions are ignored)

    <input
      type="number"
      bind:value={options.angle_diff_threshold}
      min="0"
      max="90"
      step="1"
      on:change={onChange}
    />
  </label>

  <label>
    How large may the ratio of lengths between the candidates be? (The ratio is
    always >= 1 (longer.length / shorter.length))

    <input
      type="number"
      bind:value={options.length_ratio_threshold}
      min="1"
      max="10"
      step="0.1"
      on:change={onChange}
    />
  </label>

  <label>
    How far away can the midpoints of the candidates be, in meters?

    <input
      type="number"
      bind:value={options.midpt_dist_threshold}
      min="1"
      max="50"
      step="1"
      on:change={onChange}
    />
  </label>
</details>
