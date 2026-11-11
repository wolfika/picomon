<script lang="ts">
  import Card from '../lib/Card.svelte';
  import Chart from '../lib/Chart.svelte';
  import Gauge from '../lib/Gauge.svelte';
  import { level, pct, peak, span } from '../lib/format';
  import type { Meta, Point, Sample } from '../lib/types';

  let { meta, sample, history }: { meta: Meta; sample: Sample; history: Point[] } = $props();

  const cores = $derived(sample.cpu.cores);
  // Index labels only fit while each bar is wide enough for two digits.
  const labelled = $derived(cores.length <= 16);
</script>

<Card title="CPU" aside="{meta.host.cpus} threads">
  <div class="gauge-row">
    <Gauge
      value={sample.cpu.usage}
      display={sample.cpu.usage.toFixed(1)}
      unit="%"
      color="var(--cpu)"
      label="CPU usage"
    />
    <Chart
      label="CPU usage"
      points={history}
      capacity={meta.history_len}
      series={[{ key: 'cpu', label: 'Usage', color: 'var(--cpu)' }]}
      format={pct}
      max={100}
      caption={span(meta)}
      note="peak {pct(peak(history, 'cpu'))}"
    />
  </div>
  <div class="cores" class:labelled>
    {#each cores as c, i (i)}
      <div class="core" title="Thread {i}: {pct(c)}">
        <div class="bar {level(c)}"><div class="fill" style:height="{c}%"></div></div>
        {#if labelled}<span>{i}</span>{/if}
      </div>
    {/each}
  </div>
  <div class="split">
    <span class="muted">Load average</span>
    <span>{sample.load.map((l) => l.toFixed(2)).join(' · ')}</span>
  </div>
</Card>

<style>
  .cores {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(10px, 1fr));
    gap: 4px;
  }
  .cores.labelled {
    grid-template-columns: repeat(auto-fill, minmax(26px, 32px));
    gap: 8px;
  }
  .core {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    font-size: 10px;
    color: var(--faint);
  }
  .bar {
    --c: var(--cpu);
    position: relative;
    width: 100%;
    height: 32px;
    border-radius: 6px;
    background: color-mix(in srgb, var(--c) 14%, transparent);
    border: 1px solid color-mix(in srgb, var(--c) 18%, transparent);
    overflow: hidden;
  }
  .cores:not(.labelled) .bar {
    height: 28px;
    border-radius: 3px;
    border: 0;
  }
  .bar.warn {
    --c: var(--warn);
  }
  .bar.crit {
    --c: var(--crit);
  }
  .fill {
    position: absolute;
    inset: auto 0 0 0;
    background: var(--c);
    transition: height 0.4s ease;
  }
</style>
