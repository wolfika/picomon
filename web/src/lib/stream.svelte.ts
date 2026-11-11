import type { Meta, Point, Sample } from './types';

function toPoint(s: Sample): Point {
  return {
    t: s.t,
    cpu: s.cpu.usage,
    temp: s.cpu.temp,
    mem: s.mem.used,
    swap: s.mem.swap_used,
    rx: s.net.rx,
    tx: s.net.tx,
    read: s.disk_io.read,
    write: s.disk_io.write,
  };
}

class Live {
  meta = $state.raw<Meta | null>(null);
  sample = $state.raw<Sample | null>(null);
  history = $state.raw<Point[]>([]);
  connected = $state(false);

  connect() {
    // EventSource reconnects on its own; each reconnect starts with a fresh init event.
    const es = new EventSource('/api/stream');
    es.onopen = () => (this.connected = true);
    es.onerror = () => (this.connected = false);

    es.addEventListener('init', (e) => {
      const init: { meta: Meta; history: Point[]; sample: Sample | null } = JSON.parse(e.data);
      this.meta = init.meta;
      this.history = init.history;
      this.sample = init.sample;
      this.connected = true;
    });

    es.addEventListener('sample', (e) => {
      const s: Sample = JSON.parse(e.data);
      // The server may send the sample that was already in init.
      if (this.sample && s.t <= this.sample.t) return;
      this.sample = s;
      const cap = this.meta?.history_len ?? 0;
      if (cap > 0) {
        const h = this.history.length >= cap ? this.history.slice(1 - cap) : this.history.slice();
        h.push(toPoint(s));
        this.history = h;
      }
    });
  }
}

export const live = new Live();
