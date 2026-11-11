<script lang="ts">
  import Card from '../lib/Card.svelte';
  import Chart from '../lib/Chart.svelte';
  import { bytes, rate } from '../lib/format';
  import type { Meta, Point, Sample } from '../lib/types';

  let { meta, sample, history }: { meta: Meta; sample: Sample; history: Point[] } = $props();
</script>

<Card title="Network">
  <div class="pair">
    <div class="stat">
      <span class="legend"><i style:background="var(--series-1)"></i>Download</span>
      <span class="value">{rate(sample.net.rx)}</span>
    </div>
    <div class="stat">
      <span class="legend"><i style:background="var(--series-2)"></i>Upload</span>
      <span class="value">{rate(sample.net.tx)}</span>
    </div>
  </div>
  <Chart
    label="Network throughput"
    points={history}
    capacity={meta.history_len}
    series={[
      { key: 'rx', label: 'Download', color: 'var(--series-1)' },
      { key: 'tx', label: 'Upload', color: 'var(--series-2)' },
    ]}
    format={rate}
    binary
  />
  {#if sample.net.ifaces.length > 0}
    <table class="list">
      <thead>
        <tr><th>Interface</th><th>Down</th><th>Up</th><th>Total ↓/↑</th></tr>
      </thead>
      <tbody>
        {#each sample.net.ifaces as i (i.name)}
          <tr>
            <td>{i.name}</td>
            <td>{rate(i.rx)}</td>
            <td>{rate(i.tx)}</td>
            <td class="muted">{bytes(i.rx_total)} / {bytes(i.tx_total)}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</Card>
