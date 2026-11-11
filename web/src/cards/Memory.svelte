<script lang="ts">
  import Card from '../lib/Card.svelte';
  import Chart from '../lib/Chart.svelte';
  import Gauge from '../lib/Gauge.svelte';
  import Meter from '../lib/Meter.svelte';
  import { bytes, span } from '../lib/format';
  import type { Meta, Point, Sample } from '../lib/types';

  let { meta, sample, history }: { meta: Meta; sample: Sample; history: Point[] } = $props();

  const m = $derived(sample.mem);
  const used = $derived(m.total ? (100 * m.used) / m.total : 0);
  const swap = $derived(m.swap_total ? (100 * m.swap_used) / m.swap_total : 0);
</script>

<Card title="Memory" aside="{bytes(m.used)} / {bytes(m.total)}">
  <div class="gauge-row">
    <Gauge value={used} display={used.toFixed(0)} unit="%" color="var(--mem)" label="Memory used" />
    <Chart
      label="Memory used"
      points={history}
      capacity={meta.history_len}
      series={[{ key: 'mem', label: 'Used', color: 'var(--mem)' }]}
      format={bytes}
      max={m.total}
      caption={span(meta)}
      note="{bytes(m.available)} available"
    />
  </div>
  {#if m.swap_total > 0}
    <div class="swap">
      <div class="split">
        <span class="muted">Swap</span>
        <span>{bytes(m.swap_used)} / {bytes(m.swap_total)}</span>
      </div>
      <Meter value={swap} label="Swap used" color="var(--mem)" />
    </div>
  {/if}
  <div class="split">
    <span class="muted">Cache</span>
    <span>{bytes(m.cached)}</span>
  </div>
</Card>

<style>
  .swap {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
</style>
