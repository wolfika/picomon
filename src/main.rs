mod collect;
mod config;
mod hub;
mod server;

use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;

use config::Config;

const DEFAULT_CONFIG_PATH: &str = "/etc/picomon/config.toml";

const USAGE: &str = "\
Usage: picomon [OPTIONS]

Options:
  -c, --config <PATH>  Config file (default: $PICOMON_CONFIG, then /etc/picomon/config.toml if present)
      --print-config   Print the default config and exit
  -h, --help           Print help
  -V, --version        Print version
";

enum Cli {
    Run(Option<PathBuf>),
    Exit(ExitCode),
}

fn parse_args() -> Cli {
    let mut args = std::env::args().skip(1);
    let mut config = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-c" | "--config" => match args.next() {
                Some(p) => config = Some(PathBuf::from(p)),
                None => {
                    eprintln!("error: {arg} requires a path\n\n{USAGE}");
                    return Cli::Exit(ExitCode::from(2));
                }
            },
            "--print-config" => {
                print!("{}", config::EXAMPLE);
                return Cli::Exit(ExitCode::SUCCESS);
            }
            "-h" | "--help" => {
                print!("{USAGE}");
                return Cli::Exit(ExitCode::SUCCESS);
            }
            "-V" | "--version" => {
                println!("picomon {}", env!("CARGO_PKG_VERSION"));
                return Cli::Exit(ExitCode::SUCCESS);
            }
            _ => {
                eprintln!("error: unexpected argument {arg:?}\n\n{USAGE}");
                return Cli::Exit(ExitCode::from(2));
            }
        }
    }
    let config = config
        .or_else(|| std::env::var_os("PICOMON_CONFIG").map(PathBuf::from))
        .or_else(|| {
            let p = PathBuf::from(DEFAULT_CONFIG_PATH);
            p.exists().then_some(p)
        });
    Cli::Run(config)
}

fn main() -> ExitCode {
    let config_path = match parse_args() {
        Cli::Run(p) => p,
        Cli::Exit(code) => return code,
    };
    let cfg = match Config::load(config_path.as_deref()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("picomon: {e}");
            return ExitCode::FAILURE;
        }
    };
    match &config_path {
        Some(p) => eprintln!("picomon: loaded config from {}", p.display()),
        None => eprintln!("picomon: no config file, using defaults"),
    }

    let host = collect::HostInfo::read();
    let collector = collect::Collector::new(&cfg);
    match collector.temp_path() {
        Some(p) => eprintln!("picomon: CPU temperature from {}", p.display()),
        None => eprintln!("picomon: no CPU temperature sensor found"),
    }
    let hub = Arc::new(hub::Hub::new(cfg.collector.history_len()));
    hub::spawn_collector(hub.clone(), collector, cfg.collector.interval());
    let app = server::router(&cfg, hub, &host);

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("building tokio runtime");
    rt.block_on(async {
        let listener = match tokio::net::TcpListener::bind(&cfg.server.listen).await {
            Ok(l) => l,
            Err(e) => {
                eprintln!("picomon: binding {}: {e}", cfg.server.listen);
                return ExitCode::FAILURE;
            }
        };
        eprintln!("picomon: listening on http://{}", cfg.server.listen);
        // SSE streams never end on their own, so exit directly on a signal
        // instead of waiting for connections to drain. There is no state to flush.
        tokio::select! {
            r = axum::serve(listener, app) => {
                if let Err(e) = r {
                    eprintln!("picomon: server error: {e}");
                    return ExitCode::FAILURE;
                }
            }
            _ = shutdown() => eprintln!("picomon: shutting down"),
        }
        ExitCode::SUCCESS
    })
}

async fn shutdown() {
    use tokio::signal::unix::{SignalKind, signal};
    let mut term = signal(SignalKind::terminate()).expect("installing SIGTERM handler");
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {}
        _ = term.recv() => {}
    }
}
