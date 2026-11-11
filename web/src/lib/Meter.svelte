<script lang="ts">
  import { level } from './format';

  let { value, label }: { value: number; label: string } = $props();
  const lvl = $derived(level(value));
</script>

<div
  class="meter {lvl}"
  role="meter"
  aria-label={label}
  aria-valuemin="0"
  aria-valuemax="100"
  aria-valuenow={Math.round(value)}
>
  <div class="fill" style:width="{Math.min(100, Math.max(0, value))}%"></div>
</div>

<style>
  .meter {
    --c: var(--accent);
    height: 8px;
    border-radius: 999px;
    /* Track is a lighter step of the fill colour so state reads across the whole bar. */
    background: color-mix(in srgb, var(--c) 18%, transparent);
    overflow: hidden;
  }
  .meter.warn {
    --c: var(--warn);
  }
  .meter.crit {
    --c: var(--crit);
  }
  .fill {
    height: 100%;
    border-radius: 999px;
    background: var(--c);
    transition: width 0.4s ease;
  }
</style>
