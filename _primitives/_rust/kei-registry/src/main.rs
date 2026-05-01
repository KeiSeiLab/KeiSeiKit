//! kei-registry binary entry point.
//!
//! Constructor Pattern: this file does ONE thing — parse CLI args and
//! dispatch to `handlers::dispatch`. All policy lives in the library.
//! Exit codes per spec: 0 success, 1 IO error, 2 not-found, 3 schema mismatch.

use clap::Parser;
use kei_registry::cli::Cli;
use kei_registry::handlers::{dispatch, Outcome};

fn main() {
    let cli = Cli::parse();
    let exit_code = match dispatch(cli.command) {
        Ok(Outcome::Ok) => 0,
        Ok(Outcome::NotFound(target)) => {
            eprintln!("kei-registry: not found: {target}");
            2
        }
        Err(e) => {
            eprintln!("kei-registry: error: {e:#}");
            1
        }
    };
    std::process::exit(exit_code);
}
