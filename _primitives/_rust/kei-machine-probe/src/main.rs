//! kei-machine-probe — CLI entry.
//!
//! Constructor Pattern: `main` does parse + dispatch only. All subcommand
//! bodies live in `cli.rs`; all detection lives in the library. ≤30 LOC.

use clap::Parser;
use kei_machine_probe::cli::{dispatch, Cli};
use std::process::ExitCode;

fn main() -> ExitCode {
    dispatch(Cli::parse())
}
