//! clap CLI shapes — three subcommands.
//!
//! Constructor Pattern: this cube holds parser structs only. Dispatch
//! happens in `main.rs`; per-subcommand handlers live in this module
//! and call into the library.

use crate::profile::OsFamily;
use crate::{probe, recommend, render_markdown, render_plain, MockRunner, Runner, SystemRunner};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(
    name = "kei-machine-probe",
    version,
    about = "Wave 56 — Mac hardware/OS/tooling capability detector"
)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    /// Run all detectors, emit JSON Machine struct.
    Probe {
        #[arg(long)]
        mock_dir: Option<PathBuf>,
        #[arg(long)]
        no_tooling: bool,
    },
    /// Probe + recommend, emit JSON Recommendations struct.
    Capabilities {
        #[arg(long)]
        mock_dir: Option<PathBuf>,
    },
    /// Probe + recommend, emit human-readable summary.
    Report {
        #[arg(long)]
        mock_dir: Option<PathBuf>,
        #[arg(long)]
        markdown: bool,
    },
}

pub fn dispatch(cli: Cli) -> ExitCode {
    match cli.cmd {
        Cmd::Probe { mock_dir, no_tooling } => cmd_probe(mock_dir, no_tooling),
        Cmd::Capabilities { mock_dir } => cmd_capabilities(mock_dir),
        Cmd::Report { mock_dir, markdown } => cmd_report(mock_dir, markdown),
    }
}

fn cmd_probe(mock_dir: Option<PathBuf>, no_tooling: bool) -> ExitCode {
    let runner = build_runner(mock_dir);
    let machine = probe(runner.as_ref(), no_tooling);
    print_json(&machine, &machine.os.family)
}

fn cmd_capabilities(mock_dir: Option<PathBuf>) -> ExitCode {
    let runner = build_runner(mock_dir);
    let machine = probe(runner.as_ref(), false);
    let rec = recommend(&machine);
    print_json(&rec, &machine.os.family)
}

fn cmd_report(mock_dir: Option<PathBuf>, markdown: bool) -> ExitCode {
    let runner = build_runner(mock_dir);
    let machine = probe(runner.as_ref(), false);
    let rec = recommend(&machine);
    let body = if markdown {
        render_markdown(&machine, &rec)
    } else {
        render_plain(&machine, &rec)
    };
    println!("{body}");
    exit_for_family(&machine.os.family)
}

fn build_runner(mock_dir: Option<PathBuf>) -> Box<dyn Runner> {
    match mock_dir {
        Some(d) => Box::new(MockRunner::from_dir(d)),
        None => Box::new(SystemRunner),
    }
}

fn print_json<T: serde::Serialize>(value: &T, family: &OsFamily) -> ExitCode {
    match serde_json::to_string_pretty(value) {
        Ok(s) => {
            println!("{s}");
            exit_for_family(family)
        }
        Err(e) => {
            eprintln!("kei-machine-probe: serialize: {e}");
            ExitCode::from(1)
        }
    }
}

fn exit_for_family(family: &OsFamily) -> ExitCode {
    match family {
        OsFamily::Macos => ExitCode::SUCCESS,
        OsFamily::Linux | OsFamily::Other => ExitCode::from(2),
    }
}
