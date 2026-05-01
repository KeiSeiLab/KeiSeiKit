//! Thin bin shim — defers to the library `cli::run` entry.

use std::process::ExitCode;

use clap::Parser;
use kei_token_tracker::cli::{run, Cli};

fn main() -> ExitCode {
    run(Cli::parse())
}
