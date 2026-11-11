use serde::Serialize;

use super::read;

/// Static host facts, read once at startup.
#[derive(Serialize)]
pub struct HostInfo {
    pub hostname: String,
    pub os: String,
    pub kernel: String,
    pub arch: &'static str,
    pub cpu_model: String,
    /// Logical CPUs.
    pub cpus: usize,
}

impl HostInfo {
    pub fn read() -> Self {
        Self {
            hostname: read("/proc/sys/kernel/hostname").trim().to_string(),
            os: os_name(&read("/etc/os-release")),
            kernel: read("/proc/sys/kernel/osrelease").trim().to_string(),
            arch: std::env::consts::ARCH,
            cpu_model: cpu_model(&read("/proc/cpuinfo")),
            cpus: cpu_count(&read("/proc/stat")),
        }
    }
}

fn os_name(os_release: &str) -> String {
    let field = |key: &str| {
        os_release.lines().find_map(|l| {
            l.strip_prefix(key)?
                .strip_prefix('=')
                .map(|v| v.trim_matches('"').to_string())
        })
    };
    field("PRETTY_NAME")
        .or_else(|| field("NAME"))
        .unwrap_or_else(|| "Linux".into())
}

fn cpu_model(cpuinfo: &str) -> String {
    // x86 uses "model name"; many ARM boards only have "Hardware" or "Model".
    for key in ["model name", "Hardware", "Model", "cpu model"] {
        let found = cpuinfo.lines().find_map(|l| {
            let (k, v) = l.split_once(':')?;
            (k.trim() == key).then(|| v.trim().to_string())
        });
        if let Some(v) = found.filter(|v| !v.is_empty()) {
            return v;
        }
    }
    String::new()
}

/// Online CPUs as listed in /proc/stat, matching the per-core usage figures.
/// `available_parallelism` is not used because it shrinks under cgroup quotas
/// and affinity masks, while /proc/stat always lists every online CPU.
fn cpu_count(stat: &str) -> usize {
    stat.lines()
        .filter(|l| {
            l.strip_prefix("cpu")
                .is_some_and(|r| r.starts_with(|c: char| c.is_ascii_digit()))
        })
        .count()
        .max(1)
}

pub fn uptime() -> f64 {
    read("/proc/uptime")
        .split_whitespace()
        .next()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0.0)
}

pub fn loadavg() -> [f64; 3] {
    let text = read("/proc/loadavg");
    let mut it = text.split_whitespace().map(|v| v.parse().unwrap_or(0.0));
    [
        it.next().unwrap_or(0.0),
        it.next().unwrap_or(0.0),
        it.next().unwrap_or(0.0),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_os_release() {
        assert_eq!(
            os_name("NAME=Debian\nPRETTY_NAME=\"Debian GNU/Linux 12\"\n"),
            "Debian GNU/Linux 12"
        );
        assert_eq!(os_name("NAME=Alpine\n"), "Alpine");
    }

    #[test]
    fn parses_cpu_model() {
        assert_eq!(
            cpu_model("processor\t: 0\nmodel name\t: AMD Ryzen 7\n"),
            "AMD Ryzen 7"
        );
        assert_eq!(cpu_model("Hardware\t: BCM2835\n"), "BCM2835");
    }

    #[test]
    fn counts_cpus_from_stat() {
        assert_eq!(cpu_count("cpu  1 2 3\ncpu0 1 2 3\ncpu1 1 2 3\nintr 5\n"), 2);
        assert_eq!(cpu_count(""), 1);
    }
}
