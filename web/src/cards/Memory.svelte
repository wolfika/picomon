<script lang="ts">
  import Card from '../lib/Card.svelte';
  import Chart from '../lib/Chart.svelte';
  import Meter from '../lib/Meter.svelte';
  import { bytes, pct } from '../lib/format';
  import type { Meta, Point, Sample } from '../lib/types';

  let { meta, sample, history }: { meta: Meta; sample: Sample; history: Point[] } = $props();

  const m = $derived(sample.mem);
  const used = $derived(m.total ? (100 * m.used) / m.total : 0);
  const swap = $derived(m.swap_total ? (100 * m.swap_used) / m.swap_total : 0);
</script>

<Card title="Memory" aside="{bytes(m.total)} total">
  <div class="row">
    <div class="hero">{pct(used)}</div>
    <dl class="kv">
      <div><dt>Used</dt><dd>{bytes(m.used)}</dd></div>
      <div><dt>Available</dt><dd>{bytes(m.available)}</dd></div>
      <div><dt>Cache</dt><dd>{bytes(m.cached)}</dd></div>
    </dl>
  </div>
  <Meter value={used} label="Memory used" />
  <Chart
    label="Memory used"
    points={history}
    capacity={meta.history_len}
    series={[{ key: 'mem', label: 'Used', color: 'var(--accent)' }]}
    format={bytes}
    max={m.total}
  />
  {#if m.swap_total > 0}
    <div class="meter-row">
      <div class="meter-head">
        <span>Swap</span>
        <span class="muted">{bytes(m.swap_used)} / {bytes(m.swap_total)}</span>
      </div>
      <Meter value={swap} label="Swap used" />
    </div>
  {/if}
</Card>
