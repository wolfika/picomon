//! Metric collection from /proc and /sys.

mod cpu;
mod disk;
mod host;
mod mem;
mod net;
mod temp;

use std::path::PathBuf;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use serde::Serialize;

pub use host::HostInfo;

use crate::config::Config;

#[derive(Serialize)]
pub struct Sample {
    /// Increases by one per sample. Clients use it to drop samples they already
    /// have; `t` is wall-clock time and can jump backwards.
    pub seq: u64,
    /// Unix time in milliseconds.
    pub t: u64,
    pub uptime: f64,
    pub load: [f64; 3],
    pub cpu: CpuSample,
    pub mem: mem::Memory,
    pub disks: Vec<disk::Disk>,
    pub disk_io: Rate,
    pub net: NetSample,
}

#[derive(Serialize)]
pub struct CpuSample {
    /// Overall usage in percent.
    pub usage: f32,
    /// Per-core usage in percent.
    pub cores: Vec<f32>,
    /// Degrees Celsius.
    pub temp: Option<f32>,
}

#[derive(Serialize, Default, Clone, Copy)]
pub struct Rate {
    /// Bytes per second.
    pub read: f64,
    pub write: f64,
}

#[derive(Serialize)]
pub struct NetSample {
    /// Bytes per second, summed over shown interfaces.
    pub rx: f64,
    pub tx: f64,
    pub ifaces: Vec<net::Iface>,
}

/// Compact per-sample values kept in the history ring buffer.
#[derive(Serialize, Clone, Copy)]
pub struct Point {
    pub t: u64,
    pub cpu: f32,
    pub temp: Option<f32>,
    pub mem: u64,
    pub swap: u64,
    pub rx: f64,
    pub tx: f64,
    pub read: f64,
    pub write: f64,
}

impl Sample {
    pub fn point(&self) -> Point {
        Point {
            t: self.t,
            cpu: self.cpu.usage,
            temp: self.cpu.temp,
            mem: self.mem.used,
            swap: self.mem.swap_used,
            rx: self.net.rx,
            tx: self.net.tx,
            read: self.disk_io.read,
            write: self.disk_io.write,
        }
    }
}

/// Holds the previous counter values needed to turn cumulative counters into rates.
pub struct Collector {
    cpu: cpu::CpuState,
    net: net::NetState,
    disk_io: disk::IoState,
    temp_path: Option<PathBuf>,
    mounts: Vec<String>,
    last: Instant,
    seq: u64,
}

impl Collector {
    pub fn new(cfg: &Config) -> Self {
        let temp_path = cfg.cpu.temp_sensor.clone().or_else(temp::detect);
        Self {
            cpu: cpu::CpuState::new(),
            net: net::NetState::new(cfg.network.include.clone(), cfg.network.exclude.clone()),
            disk_io: disk::IoState::new(),
            temp_path,
            mounts: cfg.disks.mounts.clone(),
            last: Instant::now(),
            seq: 0,
        }
    }

    pub fn temp_path(&self) -> Option<&PathBuf> {
        self.temp_path.as_ref()
    }

    pub fn sample(&mut self) -> Sample {
        let now = Instant::now();
        let secs = now.duration_since(self.last).as_secs_f64();
        self.last = now;

        self.seq += 1;
        let (usage, cores) = self.cpu.sample();
        let (rx, tx, ifaces) = self.net.sample(secs);
        Sample {
            seq: self.seq,
            t: SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis() as u64),
            uptime: host::uptime(),
            load: host::loadavg(),
            cpu: CpuSample {
                usage,
                cores,
                temp: self.temp_path.as_deref().and_then(temp::read),
            },
            mem: mem::read(),
            disks: disk::usage(&self.mounts),
            disk_io: self.disk_io.sample(secs),
            net: NetSample { rx, tx, ifaces },
        }
    }
}

/// Reads a whole file, returning an empty string on error.
fn read(path: impl AsRef<std::path::Path>) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
}

/// Computes a whole per-second rate, treating counter resets as zero.
fn rate(now: u64, prev: u64, secs: f64) -> f64 {
    if secs <= 0.0 { 0.0 } else { (now.saturating_sub(prev) as f64 / secs).round() }
}

#[cfg(test)]
mod tests {
    #[test]
    fn collects_from_this_machine() {
        let mut c = super::Collector::new(&crate::config::Config::default());
        std::thread::sleep(std::time::Duration::from_millis(200));
        let s = c.sample();
        assert!(s.mem.total > 0);
        assert!(!s.cpu.cores.is_empty());
        assert!(serde_json::to_string(&s).is_ok());
        assert!(!super::HostInfo::read().kernel.is_empty());
    }
}
