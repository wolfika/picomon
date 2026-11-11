<script lang="ts">
  import Card from '../lib/Card.svelte';
  import Chart from '../lib/Chart.svelte';
  import Meter from '../lib/Meter.svelte';
  import Rates from '../lib/Rates.svelte';
  import { bytes, pct, peak, rate, span } from '../lib/format';
  import type { Disk, Meta, Point, Sample } from '../lib/types';

  let { meta, sample, history }: { meta: Meta; sample: Sample; history: Point[] } = $props();

  // Matches `df`: the root-reserved blocks count as neither used nor available.
  const usedPct = (d: Disk) => (d.used + d.avail ? (100 * d.used) / (d.used + d.avail) : 0);
  const top = $derived(Math.max(peak(history, 'read'), peak(history, 'write')));
</script>

<Card title="Storage" aside="{sample.disks.length} mounted">
  <Rates
    items={[
      { arrow: '↓', label: 'read', value: sample.disk_io.read, color: 'var(--series-1)' },
      { arrow: '↑', label: 'write', value: sample.disk_io.write, color: 'var(--series-2)' },
    ]}
  />
  <Chart
    label="Disk throughput"
    points={history}
    capacity={meta.history_len}
    series={[
      { key: 'read', label: 'Read', color: 'var(--series-1)' },
      { key: 'write', label: 'Write', color: 'var(--series-2)' },
    ]}
    format={rate}
    height={48}
    caption={span(meta)}
    note="peak {rate(top)}"
  />
  <div class="mounts">
    {#each sample.disks as d (d.mount)}
      {@const p = usedPct(d)}
      <div class="mount">
        <div class="split">
          <span class="path" title={d.device}>
            <b>{d.mount}</b>
            {#if d.fs}<span class="fs">{d.fs}</span>{/if}
          </span>
          <span class="pct">{pct(p)}</span>
        </div>
        <Meter value={p} label="{d.mount} used" color="var(--series-1)" />
        <div class="split sub">
          <span>{bytes(d.used)} of {bytes(d.used + d.avail)}</span>
          <span>{bytes(d.avail)} free</span>
        </div>
      </div>
    {/each}
  </div>
</Card>

<style>
  .mounts {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }
  .mount {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .path {
    display: flex;
    align-items: center;
    gap: 8px;
    min-width: 0;
  }
  .path b {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-weight: 600;
  }
  .fs {
    flex-shrink: 0;
    padding: 0 7px;
    border: 1px solid var(--border);
    border-radius: 999px;
    font-size: 11px;
    line-height: 18px;
    color: var(--muted);
  }
  .pct {
    font-weight: 600;
  }
</style>
