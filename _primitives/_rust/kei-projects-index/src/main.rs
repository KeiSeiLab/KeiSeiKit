//! kei-projects-index — CLI dispatcher.
//!
//! Constructor Pattern: this binary holds clap shapes only. Every command
//! forwards to a function in the sibling `cli` / `query` cubes or to
//! `kei_projects_index::*` library APIs.

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

mod cli;

#[derive(Parser)]
#[command(name = "kei-projects-index", version, about = "SQLite index of ~/Projects/")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Create the DB file and apply the schema.
    Init {
        #[arg(long)]
        db: Option<PathBuf>,
    },
    /// Walk projects_root and refresh every row.
    Rebuild {
        #[arg(long)]
        db: Option<PathBuf>,
        #[arg(long)]
        root: Option<PathBuf>,
    },
    /// Print all rows as a JSON array to stdout.
    List {
        #[arg(long)]
        db: Option<PathBuf>,
    },
    /// Print one project row as JSON.
    Get {
        #[arg(long)]
        db: Option<PathBuf>,
        #[arg(long)]
        path: String,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Init { db } => cli::cmd_init(db),
        Cmd::Rebuild { db, root } => cli::cmd_rebuild(db, root),
        Cmd::List { db } => cli::cmd_list(db),
        Cmd::Get { db, path } => cli::cmd_get(db, path),
    }
}
