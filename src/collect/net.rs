use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize)]
pub struct Iface {
    pub name: String,
    /// Bytes per second.
    pub rx: f64,
    pub tx: f64,
    /// Bytes since boot.
    pub rx_total: u64,
    pub tx_total: u64,
}

fn matches(pattern: &str, name: &str) -> bool {
    match pattern.strip_suffix('*') {
        Some(prefix) => name.starts_with(prefix),
        None => pattern == name,
    }
}

/// Parses /proc/net/dev into (name, rx bytes, tx bytes).
fn parse(text: &str) -> Vec<(String, u64, u64)> {
    text.lines()
        .skip(2)
        .filter_map(|line| {
            let (name, rest) = line.split_once(':')?;
            let f: Vec<u64> = rest.split_ascii_whitespace().map(|v| v.parse().unwrap_or(0)).collect();
            Some((name.trim().to_string(), *f.first()?, *f.get(8)?))
        })
        .collect()
}

pub struct NetState {
    include: Vec<String>,
    exclude: Vec<String>,
    prev: HashMap<String, (u64, u64)>,
}

impl NetState {
    pub fn new(include: Vec<String>, exclude: Vec<String>) -> Self {
        let mut s = Self { include, exclude, prev: HashMap::new() };
        s.prev = s.read().into_iter().map(|(n, rx, tx)| (n, (rx, tx))).collect();
        s
    }

    fn shown(&self, name: &str) -> bool {
        if self.include.is_empty() {
            !self.exclude.iter().any(|p| matches(p, name))
        } else {
            self.include.iter().any(|p| matches(p, name))
        }
    }

    fn read(&self) -> Vec<(String, u64, u64)> {
        let mut v: Vec<_> =
            parse(&super::read("/proc/net/dev")).into_iter().filter(|(n, ..)| self.shown(n)).collect();
        v.sort_by(|a, b| a.0.cmp(&b.0));
        v
    }

    /// Returns total rx/tx rates and per-interface figures since the previous call.
    pub fn sample(&mut self, secs: f64) -> (f64, f64, Vec<Iface>) {
        let now = self.read();
        let ifaces: Vec<Iface> = now
            .iter()
            .map(|(name, rx, tx)| {
                // A new interface has no baseline yet, so its first rate is zero.
                let (prx, ptx) = self.prev.get(name).copied().unwrap_or((*rx, *tx));
                Iface {
                    name: name.clone(),
                    rx: super::rate(*rx, prx, secs),
                    tx: super::rate(*tx, ptx, secs),
                    rx_total: *rx,
                    tx_total: *tx,
                }
            })
            .collect();
        self.prev = now.into_iter().map(|(n, rx, tx)| (n, (rx, tx))).collect();
        let rx = ifaces.iter().map(|i| i.rx).sum();
        let tx = ifaces.iter().map(|i| i.tx).sum();
        (rx, tx, ifaces)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_net_dev() {
        let text = "Inter-|   Receive\n face |bytes\n    lo: 10 1 0 0 0 0 0 0 20 2 0 0 0 0 0 0\n  \
                    eth0: 300 5 0 0 0 0 0 9 400 6 0 0 0 0 0 0\n";
        assert_eq!(parse(text), vec![("lo".into(), 10, 20), ("eth0".into(), 300, 400)]);
    }

    #[test]
    fn patterns() {
        assert!(matches("veth*", "veth12ab"));
        assert!(!matches("lo", "lo0"));
        let s = NetState { include: vec![], exclude: vec!["lo".into(), "docker*".into()], prev: HashMap::new() };
        assert!(s.shown("eth0"));
        assert!(!s.shown("docker0"));
    }
}
