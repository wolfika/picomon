<script lang="ts">
  let {
    value,
    max = 100,
    display,
    unit,
    color,
    label,
    size = 108,
  }: {
    value: number;
    max?: number;
    /** Text shown in the centre. */
    display: string;
    unit: string;
    /** CSS colour for the arc. */
    color: string;
    label: string;
    size?: number;
  } = $props();

  // A 270° arc open at the bottom.
  const R = 44;
  const C = 2 * Math.PI * R;
  const ARC = C * 0.75;
  const frac = $derived(Math.min(1, Math.max(0, value / max)));
</script>

<div
  class="gauge"
  style:width="{size}px"
  style:height="{size}px"
  style:--c={color}
  role="meter"
  aria-label={label}
  aria-valuemin="0"
  aria-valuemax={max}
  aria-valuenow={Math.round(value)}
  aria-valuetext="{display}{unit}"
>
  <svg viewBox="0 0 100 100" aria-hidden="true">
    <g transform="rotate(135 50 50)">
      <circle class="track" cx="50" cy="50" r={R} stroke-dasharray="{ARC} {C}" />
      <circle class="value" cx="50" cy="50" r={R} stroke-dasharray="{ARC * frac} {C}" />
    </g>
  </svg>
  <div class="label">
    <span class="num">{display}</span><span class="unit">{unit}</span>
  </div>
</div>

<style>
  .gauge {
    position: relative;
    flex-shrink: 0;
  }
  svg {
    display: block;
    width: 100%;
    height: 100%;
    overflow: visible;
  }
  circle {
    fill: none;
    stroke-width: 8;
    stroke-linecap: round;
  }
  .track {
    stroke: color-mix(in srgb, var(--muted) 22%, transparent);
  }
  .value {
    stroke: var(--c);
    transition: stroke-dasharray 0.5s ease;
  }
  .label {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: baseline;
    justify-content: center;
    padding-top: 38%;
    line-height: 1;
  }
  .num {
    font-size: 26px;
    font-weight: 700;
    letter-spacing: -0.02em;
  }
  .unit {
    margin-left: 2px;
    font-size: 12px;
    font-weight: 500;
    color: var(--muted);
  }
</style>
