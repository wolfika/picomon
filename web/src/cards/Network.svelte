<script lang="ts">
  import Card from '../lib/Card.svelte';
  import Chart from '../lib/Chart.svelte';
  import { bytes, peak, rate, span } from '../lib/format';
  import Rates from '../lib/Rates.svelte';
  import type { Meta, Point, Sample } from '../lib/types';

  let {
    meta,
    sample,
    history,
    class: klass = '',
  }: { meta: Meta; sample: Sample; history: Point[]; class?: string } = $props();

  const ifaces = $derived(sample.net.ifaces);
  const top = $derived(Math.max(peak(history, 'rx'), peak(history, 'tx')));
</script>

<Card
  title="Network"
  aside="{ifaces.length} interface{ifaces.length === 1 ? '' : 's'}"
  class={klass}
>
  <Rates
    large
    items={[
      { arrow: '↓', label: 'download', value: sample.net.rx, color: 'var(--series-1)' },
      { arrow: '↑', label: 'upload', value: sample.net.tx, color: 'var(--series-2)' },
    ]}
  />
  <Chart
    label="Network throughput"
    points={history}
    capacity={meta.history_len}
    series={[
      { key: 'rx', label: 'Download', color: 'var(--series-1)' },
      { key: 'tx', label: 'Upload', color: 'var(--series-2)' },
    ]}
    format={rate}
    height={96}
    caption={span(meta)}
    note="peak {rate(top)}"
  />
  {#if ifaces.length > 0}
    <ul class="ifaces">
      {#each ifaces as i (i.name)}
        <li>
          <div>
            <div class="name">{i.name}</div>
            <div class="sub">{bytes(i.rx_total)} received · {bytes(i.tx_total)} sent</div>
          </div>
          <div class="now">
            <span><i style:color="var(--series-1)">↓</i> {rate(i.rx)}</span>
            <span><i style:color="var(--series-2)">↑</i> {rate(i.tx)}</span>
          </div>
        </li>
      {/each}
    </ul>
  {/if}
</Card>

<style>
  .ifaces {
    list-style: none;
    margin: 0;
    padding: 0;
  }
  li {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 16px;
    padding: 10px 0;
    border-top: 1px solid var(--grid);
  }
  .name {
    font-weight: 600;
  }
  .now {
    display: flex;
    flex-wrap: wrap;
    justify-content: flex-end;
    gap: 4px 18px;
    font-size: 13px;
    white-space: nowrap;
  }
  i {
    font-style: normal;
    font-weight: 700;
  }
</style>
