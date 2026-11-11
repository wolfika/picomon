<script lang="ts">
  import type { Point } from './types';
  import { clock } from './format';

  type Key = Exclude<keyof Point, 't'>;
  interface Series {
    key: Key;
    label: string;
    /** CSS color, normally a theme variable. */
    color: string;
  }

  let {
    points,
    series,
    capacity,
    format,
    max,
    height = 72,
    binary = false,
    label,
  }: {
    points: Point[];
    series: Series[];
    /** Number of slots on the x axis, so the chart fills from the right. */
    capacity: number;
    format: (n: number) => string;
    /** Fixed top of the scale. Defaults to the largest value shown. */
    max?: number;
    height?: number;
    /** Round the auto scale in powers of 1024, for byte values. */
    binary?: boolean;
    label: string;
  } = $props();

  let width = $state(0);
  let hover = $state<number | null>(null);

  const slots = $derived(Math.max(capacity, points.length, 2));
  const top = $derived.by(() => {
    if (max !== undefined) return max;
    let m = 0;
    for (const p of points) for (const s of series) m = Math.max(m, (p[s.key] as number | null) ?? 0);
    return niceCeil(m);
  });

  /** Rounds up to 1, 2 or 5 times a power of ten, within the display unit when `binary`. */
  function niceCeil(v: number): number {
    if (v <= 0) return 1;
    const unit = binary ? 1024 ** Math.max(0, Math.floor(Math.log(v) / Math.log(1024))) : 1;
    v /= unit;
    const p = 10 ** Math.floor(Math.log10(v)) * unit;
    v *= unit;
    for (const m of [1, 2, 5, 10]) if (v <= m * p) return m * p;
    return 10 * p;
  }

  // 2px inset keeps the stroke inside the box at 0 and at the top.
  const PAD = 2;
  const x = (i: number) => ((slots - points.length + i) / (slots - 1)) * width;
  const y = (v: number) => PAD + (1 - Math.min(v, top) / top) * (height - 2 * PAD);

  function line(key: Key): string {
    let d = '';
    let pen = false;
    points.forEach((p, i) => {
      const v = p[key] as number | null;
      if (v === null) {
        pen = false;
        return;
      }
      d += `${pen ? 'L' : 'M'}${x(i).toFixed(1)},${y(v).toFixed(1)}`;
      pen = true;
    });
    return d;
  }

  function area(key: Key): string {
    const l = line(key);
    if (!l || points.length < 2) return '';
    return `${l}L${x(points.length - 1).toFixed(1)},${height}L${x(0).toFixed(1)},${height}Z`;
  }

  function onMove(e: PointerEvent) {
    if (points.length === 0 || width === 0) return;
    const rect = (e.currentTarget as SVGElement).getBoundingClientRect();
    const slot = Math.round(((e.clientX - rect.left) / rect.width) * (slots - 1));
    const i = slot - (slots - points.length);
    hover = i < 0 ? null : Math.min(i, points.length - 1);
  }

  function onKey(e: KeyboardEvent) {
    if (points.length === 0) return;
    const cur = hover ?? points.length - 1;
    if (e.key === 'ArrowLeft') hover = Math.max(0, cur - 1);
    else if (e.key === 'ArrowRight') hover = Math.min(points.length - 1, cur + 1);
    else if (e.key === 'Escape') hover = null;
    else return;
    e.preventDefault();
  }

  const hp = $derived(hover === null ? null : points[hover]);
  const tipLeft = $derived(hover === null ? 0 : x(hover));
</script>

<div class="chart" bind:clientWidth={width} style:height="{height}px">
  {#if width > 0}
    <!-- Focusable so keyboard users can step through values with the arrow keys. -->
    <!-- svelte-ignore a11y_no_noninteractive_tabindex, a11y_no_noninteractive_element_interactions -->
    <svg
      {width}
      {height}
      role="img"
      aria-label="{label}, last {points.length} samples. Use arrow keys to inspect values."
      tabindex="0"
      onpointermove={onMove}
      onpointerleave={() => (hover = null)}
      onfocus={() => (hover = points.length - 1)}
      onblur={() => (hover = null)}
      onkeydown={onKey}
    >
      <line class="grid top" x1="0" x2={width} y1={PAD} y2={PAD} />
      <line class="grid" x1="0" x2={width} y1={height - 0.5} y2={height - 0.5} />
      {#if series.length === 1}
        <path d={area(series[0].key)} fill={series[0].color} fill-opacity="0.16" />
      {/if}
      {#each series as s (s.key)}
        <path d={line(s.key)} fill="none" stroke={s.color} stroke-width="2" stroke-linejoin="round" />
      {/each}
      {#if hp}
        <line class="cross" x1={tipLeft} x2={tipLeft} y1="0" y2={height} />
        {#each series as s (s.key)}
          {@const v = hp[s.key] as number | null}
          {#if v !== null}
            <circle cx={tipLeft} cy={y(v)} r="4" fill={s.color} class="dot" />
          {/if}
        {/each}
      {/if}
    </svg>
    <span class="scale">{format(top)}</span>
    {#if hp}
      <div class="tip" class:flip={tipLeft > width / 2} style:left="{tipLeft}px">
        <div class="tip-time">{clock(hp.t)}</div>
        {#each series as s (s.key)}
          {@const v = hp[s.key] as number | null}
          <div class="tip-row">
            <span class="swatch" style:background={s.color}></span>
            <span>{s.label}</span>
            <b>{v === null ? '–' : format(v)}</b>
          </div>
        {/each}
      </div>
    {/if}
  {/if}
</div>

<style>
  .chart {
    position: relative;
    width: 100%;
    /* Room for the scale label above the plot. */
    margin-top: 16px;
  }
  svg {
    display: block;
    overflow: visible;
    outline: none;
    cursor: crosshair;
    touch-action: pan-y;
  }
  svg:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 4px;
    border-radius: 4px;
  }
  .grid {
    stroke: var(--grid);
    stroke-width: 1;
  }
  .grid.top {
    stroke-dasharray: 2 4;
  }
  .cross {
    stroke: var(--muted);
    stroke-width: 1;
  }
  .dot {
    stroke: var(--surface);
    stroke-width: 2;
  }
  .scale {
    position: absolute;
    bottom: calc(100% + 1px);
    right: 0;
    line-height: 1;
    font-size: 11px;
    color: var(--muted);
    pointer-events: none;
  }
  .tip {
    position: absolute;
    bottom: calc(100% + 8px);
    transform: translateX(-12px);
    min-width: 140px;
    padding: 8px 10px;
    border-radius: 10px;
    background: var(--tooltip);
    border: 1px solid var(--border);
    box-shadow: var(--shadow);
    font-size: 12px;
    pointer-events: none;
    z-index: 5;
    white-space: nowrap;
  }
  .tip.flip {
    transform: translateX(calc(-100% + 12px));
  }
  .tip-time {
    color: var(--muted);
    margin-bottom: 4px;
  }
  .tip-row {
    display: flex;
    align-items: center;
    gap: 6px;
  }
  .tip-row b {
    margin-left: auto;
    padding-left: 12px;
    font-weight: 600;
  }
  .swatch {
    width: 8px;
    height: 8px;
    border-radius: 2px;
  }
</style>
