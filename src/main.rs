mod config;

use std::path::PathBuf;
use std::process::ExitCode;

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
    let _ = cfg;
    ExitCode::SUCCESS
}
