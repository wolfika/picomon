# Picomon

Lightweight self-hosted monitor for a single Linux server, with a web dashboard. One binary, about 1.3 MB, using about 3 MB of RAM. Low resource footprint, ideal for personal servers, such as a Raspberry Pi.

Metrics:

- CPU usage, per-core usage and load average
- CPU temperature
- Memory and swap
- Disk usage per filesystem and disk throughput
- Network throughput per interface
- Host details: OS, kernel, architecture, CPU model, uptime

Metrics come straight from `/proc` and `/sys`. The last few minutes are kept in memory for charts. Nothing is written to disk.

## Install

Build requirements: stable Rust and [Bun](https://bun.sh).

```sh
make build                 # binary at target/release/picomon
sudo make install          # binary, systemd unit, /etc/picomon/config.toml
sudo systemctl enable --now picomon
```

Run `make build` as the user that has Rust and Bun installed. `make install` only copies the built files and does not need either toolchain.

The dashboard is served at `http://<host>:7340`.

`make install` copies the example config only if `/etc/picomon/config.toml` does not already exist.

## Update

```sh
git pull
make build
sudo make install
sudo systemctl daemon-reload       # picks up changes to the unit file
sudo systemctl restart picomon
```

If the service fails to start, `journalctl -u picomon -n 20` shows why. A config key that the new version no longer accepts is named there. Compare with `picomon --print-config`.

## Configuration

Picomon reads its config from, in order:

1. `--config <path>`
2. `$PICOMON_CONFIG`
3. `/etc/picomon/config.toml`, if it exists

With no config file, defaults are used. `picomon --print-config` prints the annotated defaults. Unknown keys are rejected at startup.

### Authentication

Setting `auth.username` and `auth.password` enables HTTP basic auth on every route. Basic auth sends credentials in clear text. Put Picomon behind a reverse proxy with TLS when it is reachable from untrusted networks. Alternatively, set `listen = "127.0.0.1:7340"` and let the proxy handle both TLS and auth.

The systemd unit reads the config as a root-only credential, so the file can stay at mode 600.

### CPU temperature

The sensor is detected from hwmon (`coretemp`, `k10temp`, `zenpower`, `cpu_thermal`, …), with thermal zones as the fallback. The detected path is logged at startup. Set `cpu.temp_sensor` if the wrong sensor is picked. The temperature card is hidden when no sensor exists, which is typical in VMs and containers.

### Themes

`theme.preset` selects a built-in theme. `glass` is the default. `[theme.colors]` overrides any theme variable; keys are CSS variable names without the leading `--`:

| Variable                                        | Used for                                                          |
| ----------------------------------------------- | ----------------------------------------------------------------- |
| `bg`, `bg-image`                                | Page background                                                   |
| `surface`, `border`, `shadow`, `blur`, `radius` | Cards                                                             |
| `hero`                                          | Header card background, layered over `surface`                    |
| `text`, `muted`, `faint`                        | Text, from strongest to weakest                                   |
| `grid`, `tooltip`                               | Chart gridlines, tooltip and menu background                      |
| `accent`                                        | Brand mark and focus rings                                        |
| `cpu`, `mem`                                    | CPU and memory gauges and charts                                  |
| `series-1`, `series-2`                          | Download/upload and read/write; `series-1` also fills disk meters |
| `ok`, `warn`, `crit`                            | Status colours, including the temperature gauge                   |

Viewers can switch theme from the menu button in the page header. The choice is stored in their browser.

## API

| Endpoint          | Description                                                                                        |
| ----------------- | -------------------------------------------------------------------------------------------------- |
| `GET /api/sample` | Latest sample as JSON                                                                              |
| `GET /api/stream` | Server-Sent Events. An `init` event with host info and history, then a `sample` event per interval |

## Development

```sh
make dev     # backend on :7340, Vite with hot reload on :5173
make test    # Rust tests and Svelte type check
```

Layout:

- `src/collect/`: metric readers, one file per subsystem
- `src/hub.rs`: collector thread, history ring buffer, broadcast
- `src/server.rs`: HTTP routes, SSE, auth, embedded frontend
- `web/`: Svelte 5 frontend, built into `web/dist` and embedded at compile time

## License

MIT. See [LICENSE](LICENSE).
