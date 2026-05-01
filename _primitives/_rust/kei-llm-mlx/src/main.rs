//! kei-llm-mlx — CLI entry.
//!
//! Constructor Pattern: `main` does parse + dispatch only. Subcommand
//! bodies live in `cli.rs`; library logic lives in sibling cubes.

use clap::Parser;
use kei_llm_mlx::cli::{dispatch, Cli};
use std::process::ExitCode;

fn main() -> ExitCode {
    dispatch(Cli::parse())
}
