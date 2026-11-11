// Mirrors the JSON emitted by src/collect and src/server.rs.

export interface HostInfo {
  hostname: string;
  os: string;
  kernel: string;
  arch: string;
  cpu_model: string;
  cpus: number;
}

export interface Meta {
  title: string;
  host: HostInfo;
  /** Seconds between samples. */
  interval: number;
  history_len: number;
}

export interface Disk {
  mount: string;
  device: string;
  fs: string;
  total: number;
  used: number;
  avail: number;
}

export interface Iface {
  name: string;
  rx: number;
  tx: number;
  rx_total: number;
  tx_total: number;
}

export interface Sample {
  /** Increases by one per sample; used for de-duplication instead of `t`. */
  seq: number;
  t: number;
  uptime: number;
  load: [number, number, number];
  cpu: { usage: number; cores: number[]; temp: number | null };
  mem: {
    total: number;
    used: number;
    available: number;
    cached: number;
    swap_total: number;
    swap_used: number;
  };
  disks: Disk[];
  disk_io: { read: number; write: number };
  net: { rx: number; tx: number; ifaces: Iface[] };
}

export interface Point {
  t: number;
  cpu: number;
  temp: number | null;
  mem: number;
  swap: number;
  rx: number;
  tx: number;
  read: number;
  write: number;
}
