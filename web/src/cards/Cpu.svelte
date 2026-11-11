<script lang="ts">
  import Card from '../lib/Card.svelte';
  import Chart from '../lib/Chart.svelte';
  import { level, pct } from '../lib/format';
  import type { Meta, Point, Sample } from '../lib/types';

  let { meta, sample, history }: { meta: Meta; sample: Sample; history: Point[] } = $props();
</script>

<Card title="CPU" aside={meta.host.cpu_model}>
  <div class="row">
    <div class="hero">{pct(sample.cpu.usage)}</div>
    <dl class="kv">
      <div><dt>Load</dt><dd>{sample.load.map((l) => l.toFixed(2)).join(' · ')}</dd></div>
      <div><dt>Threads</dt><dd>{meta.host.cpus}</dd></div>
    </dl>
  </div>
  <Chart
    label="CPU usage"
    points={history}
    capacity={meta.history_len}
    series={[{ key: 'cpu', label: 'Usage', color: 'var(--accent)' }]}
    format={pct}
    max={100}
  />
  <div class="cores" style:--n={sample.cpu.cores.length}>
    {#each sample.cpu.cores as c, i (i)}
      <div class="core {level(c)}" title="Core {i}: {pct(c)}">
        <div class="core-fill" style:height="{c}%"></div>
      </div>
    {/each}
  </div>
</Card>

<style>
  .cores {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(10px, 1fr));
    gap: 3px;
  }
  .core {
    --c: var(--accent);
    position: relative;
    height: 28px;
    border-radius: 3px;
    background: color-mix(in srgb, var(--c) 16%, transparent);
    overflow: hidden;
  }
  .core.warn {
    --c: var(--warn);
  }
  .core.crit {
    --c: var(--crit);
  }
  .core-fill {
    position: absolute;
    inset: auto 0 0 0;
    background: var(--c);
    border-radius: 3px 3px 0 0;
    transition: height 0.4s ease;
  }
</style>
