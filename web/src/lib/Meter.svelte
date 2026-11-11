<script lang="ts">
  import { level } from './format';

  let {
    value,
    label,
    color = 'var(--accent)',
  }: {
    value: number;
    label: string;
    /** Fill colour while below the warning threshold. */
    color?: string;
  } = $props();
  const lvl = $derived(level(value));
</script>

<!-- biome-ignore lint/a11y/useSemanticElements: <meter> cannot be styled consistently across browsers; role="meter" exposes the same semantics -->
<div
  class="meter {lvl}"
  style:--base={color}
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
    --c: var(--base);
    height: 6px;
    border-radius: 999px;
    /* Track is a lighter step of the fill colour so state reads across the whole bar. */
    background: color-mix(in srgb, var(--c) 16%, transparent);
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
    min-width: 6px;
    border-radius: 999px;
    background: linear-gradient(90deg, var(--c), color-mix(in oklab, var(--c) 70%, white));
    transition: width 0.4s ease;
  }
</style>
