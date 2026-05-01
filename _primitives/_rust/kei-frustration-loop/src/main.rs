//! kei-frustration-loop — per-user frustration learning loop binary.
//!
//! Five subcommands: bootstrap / nightly-scan / feedback / auto-train /
//! personalize. All work happens in cubes; this file dispatches only.
//!
//! Constructor Pattern: main.rs only routes parsed args to `cli::dispatch`.

use clap::Parser;
use std::process::ExitCode;

use kei_frustration_loop::cli;

fn main() -> ExitCode {
    let parsed = cli::Cli::parse();
    match cli::dispatch(parsed) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("kei-frustration-loop: {e:#}");
            ExitCode::from(1)
        }
    }
}
