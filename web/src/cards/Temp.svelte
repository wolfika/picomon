<script lang="ts">
  import Card from '../lib/Card.svelte';
  import Chart from '../lib/Chart.svelte';
  import type { Meta, Point } from '../lib/types';

  let { meta, temp, history }: { meta: Meta; temp: number; history: Point[] } = $props();

  const fmt = (c: number) => `${c.toFixed(0)}°C`;
  const status = $derived(temp >= 85 ? 'crit' : temp >= 70 ? 'warn' : 'ok');
  const statusText = { ok: 'Normal', warn: 'Warm', crit: 'Hot' };
</script>

<Card title="Temperature">
  <div class="row">
    <div class="hero">{temp.toFixed(1)}<span class="unit">°C</span></div>
    <span class="badge {status}">{statusText[status]}</span>
  </div>
  <Chart
    label="CPU temperature"
    points={history}
    capacity={meta.history_len}
    series={[{ key: 'temp', label: 'CPU', color: 'var(--accent)' }]}
    format={fmt}
    max={Math.max(100, ...history.map((p) => p.temp ?? 0))}
  />
</Card>
