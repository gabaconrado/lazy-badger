//! Lazy Badger CLI

use std::process::ExitCode;

use clap::Parser;

// ******** Exit codes ******** //

/// Script not found
const EXIT_SCRIPT_NOT_FOUND: u8 = 101;
/// Error getting script path
const EXIT_GET_SCRIPT_PATH: u8 = 102;
/// Error running script
const EXIT_RUN_SCRIPT: u8 = 103;
/// Unknown exit code returned by script
const EXIT_UNKNOWN_CODE: u8 = 199;

/// Definitions of the [`clap`] CLI
mod cli;

/// Main process
fn main() -> ExitCode {
    let cli = cli::Cli::parse();

    let script_path = match lazy_badger::find_script(&cli.scripts_root, cli.name.clone()) {
        Ok(Some(p)) => p,
        Ok(None) => {
            eprintln!("Script '{}' was not found", cli.name);
            return ExitCode::from(EXIT_SCRIPT_NOT_FOUND);
        }
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(EXIT_GET_SCRIPT_PATH);
        }
    };

    match lazy_badger::run_script(&script_path, &cli.args) {
        Ok(status) if status.success() => ExitCode::SUCCESS,
        Ok(status) => {
            let code = match status.code() {
                Some(c) => u8::try_from(c).unwrap_or(EXIT_UNKNOWN_CODE),
                None => EXIT_UNKNOWN_CODE,
            };
            ExitCode::from(code)
        }
        Err(err) => {
            eprintln!("Error running script: {err}");
            ExitCode::from(EXIT_RUN_SCRIPT)
        }
    }
}
