# Changelog

## 0.1.0 (2026-11-11)

Initial release. Picomon is a self-hosted monitor for a single Linux server: one static binary that collects metrics from `/proc` and `/sys` and serves a web dashboard.

### Features

* Metrics: CPU usage per core, load average, CPU temperature, memory and swap, disk usage and throughput, network throughput per interface, and host details.
* Live dashboard updates over Server-Sent Events, with recent history kept in memory for charts (10 minutes by default).
* Themes: glass (default), dark, light, nord, and Catppuccin Latte, Frappé, Macchiato and Mocha. Set in the config, with colour overrides; viewers can switch theme per browser.
* TOML configuration. `picomon --print-config` prints the annotated defaults.
* Optional HTTP basic auth.
* Hardened systemd unit, printed by `picomon --print-service`.
* JSON API: `/api/sample` for the latest sample and `/api/stream` for the live event stream.
* Static binaries for x86_64, aarch64 and armv7.
