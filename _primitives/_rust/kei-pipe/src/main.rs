//! `kei-pipe` CLI — `run <dag.toml>` and `validate <dag.toml>`.
//!
//! Exit codes:
//! - 0 — ok (run or validate)
//! - 1 — CLI / IO / parse failure (DAG couldn't be loaded)
//! - 2 — one or more steps failed during `run`

use clap::{Parser, Subcommand};
use kei_pipe::{run_dag, validate_dag};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "kei-pipe", version, about = "Atom DAG pipe runtime")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Run a DAG — execute every step in topo order, print final report.
    Run { path: PathBuf },
    /// Parse + topo-sort without executing; prints the resolved order.
    Validate { path: PathBuf },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Run { path } => cmd_run(&path),
        Cmd::Validate { path } => cmd_validate(&path),
    }
}

fn cmd_run(path: &std::path::Path) -> ExitCode {
    match run_dag(path) {
        Ok(report) => {
            match serde_json::to_string_pretty(&report) {
                Ok(s) => println!("{s}"),
                Err(e) => {
                    eprintln!("serialize report: {e}");
                    return ExitCode::from(1);
                }
            }
            if report.final_ok() {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(2)
            }
        }
        Err(e) => {
            eprintln!("{e}");
            ExitCode::from(1)
        }
    }
}

fn cmd_validate(path: &std::path::Path) -> ExitCode {
    match validate_dag(path) {
        Ok(order) => {
            for id in order {
                println!("{id}");
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{e}");
            ExitCode::from(1)
        }
    }
}
