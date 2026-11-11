use std::collections::HashMap;
use std::ffi::CString;
use std::path::Path;

use serde::Serialize;

use super::Rate;

/// Filesystems shown when no mounts are configured.
const PHYSICAL_FS: &[&str] = &[
    "ext2", "ext3", "ext4", "xfs", "btrfs", "zfs", "f2fs", "bcachefs", "jfs", "reiserfs",
    "nilfs2", "vfat", "exfat", "ntfs", "ntfs3", "fuseblk",
];

#[derive(Serialize)]
pub struct Disk {
    pub mount: String,
    pub device: String,
    pub fs: String,
    /// Bytes.
    pub total: u64,
    pub used: u64,
    /// Bytes available to unprivileged users.
    pub avail: u64,
}

struct Mount {
    device: String,
    mount: String,
    fs: String,
}

/// Decodes the octal escapes (`\040` for space) used in /proc/self/mounts.
fn unescape(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'\\' && i + 3 < b.len() && b[i + 1..i + 4].iter().all(|c| (b'0'..=b'7').contains(c)) {
            let v = (b[i + 1] - b'0') * 64 + (b[i + 2] - b'0') * 8 + (b[i + 3] - b'0');
            out.push(v);
            i += 4;
        } else {
            out.push(b[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn mounts() -> Vec<Mount> {
    super::read("/proc/self/mounts")
        .lines()
        .filter_map(|line| {
            let mut f = line.split(' ');
            Some(Mount {
                device: unescape(f.next()?),
                mount: unescape(f.next()?),
                fs: f.next()?.to_string(),
            })
        })
        .collect()
}

/// Picks physical filesystems, keeping one mount point per device (the shortest).
fn auto_select(all: Vec<Mount>) -> Vec<Mount> {
    let mut picked: Vec<Mount> = Vec::new();
    for m in all.into_iter().filter(|m| PHYSICAL_FS.contains(&m.fs.as_str())) {
        match picked.iter_mut().find(|p| p.device == m.device) {
            Some(p) if m.mount.len() < p.mount.len() => *p = m,
            Some(_) => {}
            None => picked.push(m),
        }
    }
    picked.sort_by(|a, b| a.mount.cmp(&b.mount));
    picked
}

fn statvfs(path: &str) -> Option<(u64, u64, u64)> {
    let c = CString::new(path).ok()?;
    let mut s: libc::statvfs = unsafe { std::mem::zeroed() };
    // SAFETY: `c` is a valid NUL-terminated string and `s` is a valid out-pointer.
    if unsafe { libc::statvfs(c.as_ptr(), &mut s) } != 0 {
        return None;
    }
    let bs = s.f_frsize as u64;
    let total = s.f_blocks as u64 * bs;
    let used = (s.f_blocks as u64).saturating_sub(s.f_bfree as u64) * bs;
    let avail = s.f_bavail as u64 * bs;
    Some((total, used, avail))
}

/// Usage for configured mounts, or for auto-detected physical filesystems.
pub fn usage(configured: &[String]) -> Vec<Disk> {
    let all = mounts();
    let selected: Vec<Mount> = if configured.is_empty() {
        auto_select(all)
    } else {
        configured
            .iter()
            .map(|path| {
                // The last matching entry is the one currently visible at that path.
                all.iter().rev().find(|m| &m.mount == path).map_or_else(
                    || Mount { device: String::new(), mount: path.clone(), fs: String::new() },
                    |m| Mount { device: m.device.clone(), mount: m.mount.clone(), fs: m.fs.clone() },
                )
            })
            .collect()
    };
    selected
        .into_iter()
        .filter_map(|m| {
            let (total, used, avail) = statvfs(&m.mount)?;
            (total > 0).then_some(Disk { mount: m.mount, device: m.device, fs: m.fs, total, used, avail })
        })
        .collect()
}

/// Cumulative bytes (read, written) per device name.
type IoCounters = HashMap<String, (u64, u64)>;

/// Parses /proc/diskstats, keeping devices accepted by `physical`.
fn parse_diskstats(text: &str, physical: impl Fn(&str) -> bool) -> IoCounters {
    text.lines()
        .filter_map(|line| {
            let f: Vec<&str> = line.split_ascii_whitespace().collect();
            if f.len() < 10 || !physical(f[2]) {
                return None;
            }
            // Sectors are always 512 bytes in /proc/diskstats.
            let read = f[5].parse::<u64>().unwrap_or(0) * 512;
            let write = f[9].parse::<u64>().unwrap_or(0) * 512;
            Some((f[2].to_string(), (read, write)))
        })
        .collect()
}

fn io_counters() -> IoCounters {
    // Whole disks backed by hardware have /sys/block/<name>/device; this skips
    // partitions, loop, ram, zram, dm and md devices, which would double count.
    parse_diskstats(&super::read("/proc/diskstats"), |name| {
        Path::new("/sys/block").join(name).join("device").exists()
    })
}

/// Sums per-device byte deltas. A device without a previous reading (just
/// plugged in) contributes nothing, rather than its whole lifetime total.
fn io_delta(prev: &IoCounters, now: &IoCounters) -> (u64, u64) {
    now.iter().fold((0, 0), |(r, w), (name, &(nr, nw))| match prev.get(name) {
        Some(&(pr, pw)) => (r + nr.saturating_sub(pr), w + nw.saturating_sub(pw)),
        None => (r, w),
    })
}

pub struct IoState {
    prev: IoCounters,
}

impl IoState {
    pub fn new() -> Self {
        Self { prev: io_counters() }
    }

    pub fn sample(&mut self, secs: f64) -> Rate {
        let now = io_counters();
        let (read, write) = io_delta(&self.prev, &now);
        self.prev = now;
        Rate { read: super::rate(read, 0, secs), write: super::rate(write, 0, secs) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn m(device: &str, mount: &str, fs: &str) -> Mount {
        Mount { device: device.into(), mount: mount.into(), fs: fs.into() }
    }

    #[test]
    fn unescapes_spaces() {
        assert_eq!(unescape(r"/mnt/my\040disk"), "/mnt/my disk");
        assert_eq!(unescape(r"/plain"), "/plain");
    }

    #[test]
    fn auto_select_dedupes_and_filters() {
        let got = auto_select(vec![
            m("/dev/sda1", "/mnt/wslg/distro", "ext4"),
            m("/dev/sda1", "/", "ext4"),
            m("tmpfs", "/run", "tmpfs"),
            m("/dev/sdb1", "/data", "xfs"),
        ]);
        let mounts: Vec<&str> = got.iter().map(|m| m.mount.as_str()).collect();
        assert_eq!(mounts, ["/", "/data"]);
    }

    #[test]
    fn new_device_does_not_spike_io() {
        let stats = |sda: u64, sdb: Option<u64>| {
            let mut t = format!("   8 0 sda 0 0 {sda} 0 0 0 {sda} 0 0 0 0\n");
            if let Some(b) = sdb {
                t += &format!("   8 16 sdb 0 0 {b} 0 0 0 {b} 0 0 0 0\n");
            }
            t += "   8 1 sda1 0 0 999 0 0 0 999 0 0 0 0\n";
            parse_diskstats(&t, |n| n.starts_with("sd") && n.len() == 3)
        };
        let before = stats(100, None);
        // sdb appears with a large lifetime total; only sda's 10 new sectors count.
        let after = stats(110, Some(1_000_000));
        assert_eq!(io_delta(&before, &after), (10 * 512, 10 * 512));
        // From then on sdb is counted normally.
        let later = stats(110, Some(1_000_002));
        assert_eq!(io_delta(&after, &later), (2 * 512, 2 * 512));
    }
}
