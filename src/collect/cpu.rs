#[derive(Clone, Copy, Default)]
struct Times {
    idle: u64,
    total: u64,
}

impl Times {
    fn usage_since(self, prev: Times) -> f32 {
        let total = self.total.saturating_sub(prev.total);
        let idle = self.idle.saturating_sub(prev.idle);
        if total == 0 {
            0.0
        } else {
            // One decimal is enough for display and keeps the JSON small.
            ((1000.0 * (total - idle.min(total)) as f64 / total as f64).round() / 10.0) as f32
        }
    }
}

/// Parses `/proc/stat` into the aggregate line followed by one entry per core.
fn parse(stat: &str) -> Vec<Times> {
    stat.lines()
        .take_while(|l| l.starts_with("cpu"))
        .map(|line| {
            // user nice system idle iowait irq softirq steal guest guest_nice
            let f: Vec<u64> = line
                .split_ascii_whitespace()
                .skip(1)
                .map(|v| v.parse().unwrap_or(0))
                .collect();
            let get = |i: usize| f.get(i).copied().unwrap_or(0);
            // guest time is already included in user/nice.
            let total = (0..8).map(get).sum();
            Times { idle: get(3) + get(4), total }
        })
        .collect()
}

pub struct CpuState {
    prev: Vec<Times>,
}

impl CpuState {
    pub fn new() -> Self {
        Self { prev: parse(&super::read("/proc/stat")) }
    }

    /// Returns overall usage and per-core usage in percent since the previous call.
    pub fn sample(&mut self) -> (f32, Vec<f32>) {
        let now = parse(&super::read("/proc/stat"));
        let usage: Vec<f32> = now
            .iter()
            .enumerate()
            .map(|(i, t)| t.usage_since(self.prev.get(i).copied().unwrap_or_default()))
            .collect();
        self.prev = now;
        match usage.split_first() {
            Some((total, cores)) => (*total, cores.to_vec()),
            None => (0.0, Vec::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usage_from_two_snapshots() {
        let a = parse("cpu  100 0 100 800 0 0 0 0 0 0\ncpu0 100 0 100 800 0 0 0 0 0 0\nintr 1");
        let b = parse("cpu  150 0 150 900 0 0 0 0 0 0\ncpu0 150 0 150 900 0 0 0 0 0 0\nintr 1");
        assert_eq!(a.len(), 2);
        assert_eq!(b[0].usage_since(a[0]), 50.0);
    }
}
