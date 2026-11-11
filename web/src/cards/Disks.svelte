<script lang="ts">
  import Card from '../lib/Card.svelte';
  import Chart from '../lib/Chart.svelte';
  import Meter from '../lib/Meter.svelte';
  import { bytes, pct, rate } from '../lib/format';
  import type { Disk, Meta, Point, Sample } from '../lib/types';

  let { meta, sample, history }: { meta: Meta; sample: Sample; history: Point[] } = $props();

  // Matches `df`: the root-reserved blocks count as neither used nor available.
  const usedPct = (d: Disk) => (d.used + d.avail ? (100 * d.used) / (d.used + d.avail) : 0);
</script>

<Card title="Storage">
  <div class="pair">
    <div class="stat">
      <span class="legend"><i style:background="var(--series-1)"></i>Read</span>
      <span class="value">{rate(sample.disk_io.read)}</span>
    </div>
    <div class="stat">
      <span class="legend"><i style:background="var(--series-2)"></i>Write</span>
      <span class="value">{rate(sample.disk_io.write)}</span>
    </div>
  </div>
  <Chart
    label="Disk throughput"
    points={history}
    capacity={meta.history_len}
    series={[
      { key: 'read', label: 'Read', color: 'var(--series-1)' },
      { key: 'write', label: 'Write', color: 'var(--series-2)' },
    ]}
    format={rate}
    binary
  />
  {#each sample.disks as d (d.mount)}
    {@const p = usedPct(d)}
    <div class="meter-row">
      <div class="meter-head">
        <span class="mount" title="{d.device} ({d.fs})">{d.mount}</span>
        <span class="muted">{bytes(d.used)} / {bytes(d.used + d.avail)} · <b>{pct(p)}</b></span>
      </div>
      <Meter value={p} label="{d.mount} used" />
    </div>
  {/each}
</Card>

<style>
  .mount {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  b {
    color: var(--text);
    font-weight: 600;
  }
</style>
