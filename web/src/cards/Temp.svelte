<script lang="ts">
  import Card from '../lib/Card.svelte';
  import Chart from '../lib/Chart.svelte';
  import Gauge from '../lib/Gauge.svelte';
  import { span } from '../lib/format';
  import type { Meta, Point } from '../lib/types';

  let { meta, temp, history }: { meta: Meta; temp: number; history: Point[] } = $props();

  const fmt = (c: number) => `${c.toFixed(1)} °C`;
  const status = $derived(temp >= 85 ? 'crit' : temp >= 70 ? 'warn' : 'ok');
  const statusText = { ok: 'Normal', warn: 'Warm', crit: 'Hot' };
  // The colour follows the status, which the card header also names in text.
  const color = $derived(`var(--${status})`);

  const range = $derived.by(() => {
    const t = history.map((p) => p.temp).filter((v): v is number => v !== null);
    return t.length ? `${Math.min(...t).toFixed(1)}–${Math.max(...t).toFixed(1)} °C` : undefined;
  });
</script>

<Card title="Temperature" aside={statusText[status]}>
  <div class="gauge-row">
    <Gauge value={temp} max={100} display={temp.toFixed(1)} unit="°C" {color} label="CPU temperature" />
    <Chart
      label="CPU temperature"
      points={history}
      capacity={meta.history_len}
      series={[{ key: 'temp', label: 'CPU', color }]}
      format={fmt}
      caption={span(meta)}
      note={range}
    />
  </div>
</Card>
