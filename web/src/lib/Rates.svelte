<script lang="ts">
  import { rate } from './format';

  interface Item {
    arrow: '↓' | '↑';
    label: string;
    value: number;
    /** Identity colour of the matching chart series. */
    color: string;
  }

  let { items, large = false }: { items: Item[]; large?: boolean } = $props();
</script>

<div class="rates" class:large>
  {#each items as it (it.label)}
    <div class="rate">
      <span class="arrow" style:color={it.color} aria-hidden="true">{it.arrow}</span>
      <div>
        <div class="value">{rate(it.value)}</div>
        <div class="label">{it.label}</div>
      </div>
    </div>
  {/each}
</div>

<style>
  .rates {
    display: flex;
    flex-wrap: wrap;
    gap: 12px 36px;
  }
  .rate {
    display: flex;
    align-items: baseline;
    gap: 8px;
  }
  .arrow {
    font-size: 18px;
    font-weight: 700;
  }
  .value {
    font-size: 20px;
    font-weight: 700;
    letter-spacing: -0.01em;
    line-height: 1.15;
  }
  .large .arrow {
    font-size: 22px;
  }
  .large .value {
    font-size: 28px;
  }
  .label {
    font-size: 12px;
    color: var(--faint);
  }
</style>
