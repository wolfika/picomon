//! CPU temperature sensor discovery.

use std::fs;
use std::path::{Path, PathBuf};

/// hwmon chip names that report CPU temperature, in order of preference.
const HWMON_CHIPS: &[&str] = &[
    "coretemp",
    "k10temp",
    "zenpower",
    "cpu_thermal",
    "cpu-thermal",
    "soc_thermal",
    "acpitz",
];

/// Preferred sensor labels within a chip. Tdie is the real die temperature on AMD;
/// Tctl may carry an offset.
const LABELS: &[&str] = &["Package id 0", "Tdie", "Tctl"];

/// thermal_zone types that report CPU temperature, in order of preference.
const ZONE_TYPES: &[&str] = &[
    "x86_pkg_temp",
    "cpu-thermal",
    "cpu_thermal",
    "soc-thermal",
    "acpitz",
];

fn read_trim(p: &Path) -> Option<String> {
    fs::read_to_string(p).ok().map(|s| s.trim().to_string())
}

fn hwmon() -> Option<PathBuf> {
    let chips: Vec<(String, PathBuf)> = fs::read_dir("/sys/class/hwmon")
        .ok()?
        .flatten()
        .filter_map(|e| Some((read_trim(&e.path().join("name"))?, e.path())))
        .collect();
    for want in HWMON_CHIPS {
        let Some((_, dir)) = chips.iter().find(|(name, _)| name == want) else {
            continue;
        };
        let mut inputs: Vec<PathBuf> = fs::read_dir(dir)
            .ok()?
            .flatten()
            .map(|e| e.path())
            .filter(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.starts_with("temp") && n.ends_with("_input"))
            })
            .collect();
        inputs.sort();
        for label in LABELS {
            for input in &inputs {
                let label_path =
                    input.with_file_name(input.file_name()?.to_str()?.replace("_input", "_label"));
                if read_trim(&label_path).as_deref() == Some(label) {
                    return Some(input.clone());
                }
            }
        }
        if let Some(first) = inputs.into_iter().next() {
            return Some(first);
        }
    }
    None
}

fn thermal_zone() -> Option<PathBuf> {
    let zones: Vec<(String, PathBuf)> = fs::read_dir("/sys/class/thermal")
        .ok()?
        .flatten()
        .filter(|e| e.file_name().to_string_lossy().starts_with("thermal_zone"))
        .filter_map(|e| Some((read_trim(&e.path().join("type"))?, e.path().join("temp"))))
        .collect();
    ZONE_TYPES.iter().find_map(|want| {
        zones
            .iter()
            .find(|(t, _)| t == want)
            .map(|(_, p)| p.clone())
    })
}

/// Finds the most likely CPU temperature input file.
pub fn detect() -> Option<PathBuf> {
    hwmon().or_else(thermal_zone)
}

/// Reads a sysfs temperature in millidegrees and returns degrees Celsius.
pub fn read(path: &Path) -> Option<f32> {
    let milli: i64 = read_trim(path)?.parse().ok()?;
    Some(milli as f32 / 1000.0)
}
