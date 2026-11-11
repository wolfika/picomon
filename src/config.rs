use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::Deserialize;

/// Annotated default configuration, printed by `picomon --print-config`.
pub const EXAMPLE: &str = include_str!("../config.example.toml");

#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    pub server: Server,
    pub auth: Auth,
    pub collector: Collector,
    pub cpu: Cpu,
    pub disks: Disks,
    pub network: Network,
    pub theme: Theme,
}

#[derive(Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Server {
    pub listen: String,
    /// Page title. Defaults to the hostname.
    pub title: Option<String>,
}

impl Default for Server {
    fn default() -> Self {
        Self { listen: "0.0.0.0:7340".into(), title: None }
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Auth {
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Collector {
    /// Seconds between samples.
    pub interval: f64,
    /// Minutes of history kept in memory.
    pub history: f64,
}

impl Default for Collector {
    fn default() -> Self {
        Self { interval: 2.0, history: 10.0 }
    }
}

impl Collector {
    pub fn interval(&self) -> Duration {
        Duration::from_secs_f64(self.interval)
    }

    pub fn history_len(&self) -> usize {
        ((self.history * 60.0) / self.interval).ceil() as usize
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Cpu {
    /// Explicit temperature input file, e.g. /sys/class/hwmon/hwmon2/temp1_input.
    pub temp_sensor: Option<PathBuf>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Disks {
    /// Mount points to show. Empty means auto-detect.
    pub mounts: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Network {
    /// Interfaces to show. Empty means all except `exclude`.
    pub include: Vec<String>,
    /// Interface patterns to hide. A trailing `*` matches any suffix.
    pub exclude: Vec<String>,
}

impl Default for Network {
    fn default() -> Self {
        Self {
            include: Vec::new(),
            exclude: ["lo", "veth*", "docker*", "br-*", "virbr*", "cni*", "flannel*"]
                .map(String::from)
                .to_vec(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Theme {
    pub preset: String,
    /// CSS variable overrides, keyed without the leading `--`.
    pub colors: BTreeMap<String, String>,
}

impl Default for Theme {
    fn default() -> Self {
        Self { preset: "glass".into(), colors: BTreeMap::new() }
    }
}

pub const THEME_PRESETS: &[&str] = &["glass", "dark", "light", "nord"];

impl Config {
    pub fn load(path: Option<&Path>) -> Result<Self, String> {
        let cfg: Config = match path {
            Some(p) => {
                let text = std::fs::read_to_string(p)
                    .map_err(|e| format!("reading {}: {e}", p.display()))?;
                toml::from_str(&text).map_err(|e| format!("parsing {}: {e}", p.display()))?
            }
            None => Config::default(),
        };
        cfg.validate()?;
        Ok(cfg)
    }

    fn validate(&self) -> Result<(), String> {
        let c = &self.collector;
        if !(0.25..=3600.0).contains(&c.interval) {
            return Err("collector.interval must be between 0.25 and 3600 seconds".into());
        }
        if !(c.history >= 0.0 && c.history <= 1440.0) {
            return Err("collector.history must be between 0 and 1440 minutes".into());
        }
        if self.auth.username.is_some() != self.auth.password.is_some() {
            return Err("auth.username and auth.password must be set together".into());
        }
        if !THEME_PRESETS.contains(&self.theme.preset.as_str()) {
            return Err(format!(
                "theme.preset must be one of: {}",
                THEME_PRESETS.join(", ")
            ));
        }
        for (key, value) in &self.theme.colors {
            if key.is_empty() || !key.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '-') {
                return Err(format!("theme.colors: invalid variable name {key:?}"));
            }
            if value.chars().any(|ch| matches!(ch, ';' | '{' | '}' | '<' | '>' | '\\')) {
                return Err(format!("theme.colors.{key}: value contains a forbidden character"));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_config_parses_and_matches_defaults() {
        let cfg: Config = toml::from_str(EXAMPLE).unwrap();
        cfg.validate().unwrap();
        let def = Config::default();
        assert_eq!(cfg.server.listen, def.server.listen);
        assert_eq!(cfg.collector.interval, def.collector.interval);
        assert_eq!(cfg.collector.history, def.collector.history);
        assert_eq!(cfg.network.exclude, def.network.exclude);
        assert_eq!(cfg.theme.preset, def.theme.preset);
    }

    #[test]
    fn rejects_half_configured_auth() {
        let cfg: Config = toml::from_str("[auth]\nusername = \"a\"").unwrap();
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn rejects_css_injection() {
        let cfg: Config = toml::from_str("[theme.colors]\naccent = \"red;}</style>\"").unwrap();
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn history_len() {
        let c = Collector { interval: 2.0, history: 10.0 };
        assert_eq!(c.history_len(), 300);
    }
}
