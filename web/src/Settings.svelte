<script lang="ts">
  interface Props {
    options: {
      buffer_meters: number;
      angle_diff_threshold: number;
      length_ratio_threshold: number;
      midpt_dist_threshold: number;
    };
    // TODO Reconsider if necessary
    onChange: () => void;
  }

  let { options = $bindable(), onChange }: Props = $props();
</script>

<div class="card">
  <div class="card-body">
    <h3 class="card-title">Settings</h3>

    <form>
      <label class="form-label">
        Expand the bounding box around each target by this many meters in all
        directions

        <input
          class="form-control"
          type="number"
          bind:value={options.buffer_meters}
          min="0"
          max="500"
          step="1"
          onchange={onChange}
        />
      </label>

      <label class="form-label">
        How many degrees difference allowed between matches? (LineStrings
        pointing in opposite directions are ignored)

        <input
          class="form-control"
          type="number"
          bind:value={options.angle_diff_threshold}
          min="0"
          max="90"
          step="1"
          onchange={onChange}
        />
      </label>

      <label class="form-label">
        How large may the ratio of lengths between the candidates be? (The ratio
        is always >= 1 (longer.length / shorter.length))

        <input
          class="form-control"
          type="number"
          bind:value={options.length_ratio_threshold}
          min="1"
          max="10"
          step="0.1"
          onchange={onChange}
        />
      </label>

      <label class="form-label">
        How far away can the midpoints of the candidates be, in meters?

        <input
          class="form-control"
          type="number"
          bind:value={options.midpt_dist_threshold}
          min="1"
          max="50"
          step="1"
          onchange={onChange}
        />
      </label>
    </form>
  </div>
</div>
