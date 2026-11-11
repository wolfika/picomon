import type { Meta, Point } from './types';

const UNITS = ['B', 'KiB', 'MiB', 'GiB', 'TiB', 'PiB'];

export function bytes(n: number): string {
  let i = 0;
  while (n >= 1024 && i < UNITS.length - 1) {
    n /= 1024;
    i++;
  }
  const digits = i === 0 || n >= 100 ? 0 : 1;
  return `${n.toFixed(digits)} ${UNITS[i]}`;
}

export function rate(n: number): string {
  return `${bytes(n)}/s`;
}

export function pct(n: number): string {
  return `${n.toFixed(n < 10 ? 1 : 0)}%`;
}

export function duration(secs: number): string {
  const d = Math.floor(secs / 86400);
  const h = Math.floor((secs % 86400) / 3600);
  const m = Math.floor((secs % 3600) / 60);
  if (d > 0) return `${d}d ${h}h ${m}m`;
  if (h > 0) return `${h}h ${m}m`;
  return `${m}m`;
}

export function clock(t: number): string {
  return new Date(t).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' });
}

/** Severity level for a 0–100 value, used by meters. */
export function level(value: number, warn = 75, crit = 90): 'ok' | 'warn' | 'crit' {
  return value >= crit ? 'crit' : value >= warn ? 'warn' : 'ok';
}

/** Caption for the time span a chart covers, e.g. "Last 10 min". */
export function span(meta: Meta): string {
  const mins = (meta.history_len * meta.interval) / 60;
  return mins >= 120 ? `Last ${Math.round(mins / 60)} h` : `Last ${Math.round(mins)} min`;
}

/** Highest value of a series over the history. */
export function peak(points: Point[], key: Exclude<keyof Point, 't'>): number {
  let m = 0;
  for (const p of points) m = Math.max(m, (p[key] as number | null) ?? 0);
  return m;
}
