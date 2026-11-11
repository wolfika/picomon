use serde::Serialize;

/// Memory figures in bytes.
#[derive(Serialize, Default)]
pub struct Memory {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    /// Page cache and buffers, reclaimable.
    pub cached: u64,
    pub swap_total: u64,
    pub swap_used: u64,
}

pub fn read() -> Memory {
    parse(&super::read("/proc/meminfo"))
}

fn parse(text: &str) -> Memory {
    let mut m = Memory::default();
    let (mut free, mut buffers, mut cached, mut sreclaimable, mut swap_free) = (0, 0, 0, 0, 0);
    let mut available = None;
    for line in text.lines() {
        let Some((key, rest)) = line.split_once(':') else {
            continue;
        };
        let kb: u64 = rest.trim().trim_end_matches(" kB").parse().unwrap_or(0);
        let bytes = kb * 1024;
        match key {
            "MemTotal" => m.total = bytes,
            "MemFree" => free = bytes,
            "MemAvailable" => available = Some(bytes),
            "Buffers" => buffers = bytes,
            "Cached" => cached = bytes,
            "SReclaimable" => sreclaimable = bytes,
            "SwapTotal" => m.swap_total = bytes,
            "SwapFree" => swap_free = bytes,
            _ => {}
        }
    }
    m.cached = buffers + cached + sreclaimable;
    // MemAvailable is missing on kernels older than 3.14.
    m.available = available.unwrap_or(free + m.cached).min(m.total);
    m.used = m.total - m.available;
    m.swap_used = m.swap_total.saturating_sub(swap_free);
    m
}

#[cfg(test)]
mod tests {
    #[test]
    fn parses_meminfo() {
        let m = super::parse(
            "MemTotal: 1000 kB\nMemFree: 200 kB\nMemAvailable: 600 kB\nBuffers: 50 kB\n\
             Cached: 300 kB\nSwapTotal: 100 kB\nSwapFree: 40 kB\n",
        );
        assert_eq!(m.total, 1000 * 1024);
        assert_eq!(m.used, 400 * 1024);
        assert_eq!(m.swap_used, 60 * 1024);
    }
}
