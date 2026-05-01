//! kei-projects-watcher — CLI binary.
//!
//! Expected install path (referenced by the launchd plist template
//! `kei-projects-watcher.plist.tmpl` shipped by the orchestrator):
//!     ${KIT}/_rust/target/release/kei-projects-watcher run
//!
//! Subcommands:
//!   run     — daemon: watch ~/Projects and re-index on the fly
//!   status  — print last-indexed-ts of every project as JSON

use std::path::PathBuf;
use std::time::Duration;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use kei_projects_watcher::{cmd_run, cmd_status};

#[derive(Parser)]
#[command(name = "kei-projects-watcher", version)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Run the watcher daemon until SIGINT / SIGTERM.
    Run {
        #[arg(long)]
        db: Option<PathBuf>,
        #[arg(long)]
        root: Option<PathBuf>,
        #[arg(long, default_value_t = 2000)]
        debounce_ms: u64,
    },
    /// Print last-indexed-ts of each project as JSON.
    Status {
        #[arg(long)]
        db: Option<PathBuf>,
    },
}

fn home_join(rel: &str) -> Result<PathBuf> {
    Ok(dirs::home_dir().context("$HOME not set")?.join(rel))
}

fn default_db(opt: Option<PathBuf>) -> Result<PathBuf> {
    opt.map(Ok)
        .unwrap_or_else(|| home_join(".claude/agents/projects-index.sqlite"))
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    match Cli::parse().cmd {
        Cmd::Run { db, root, debounce_ms } => {
            let db = default_db(db)?;
            let root = root.map(Ok).unwrap_or_else(|| home_join("Projects"))?;
            cmd_run(db, root, Duration::from_millis(debounce_ms)).await
        }
        Cmd::Status { db } => cmd_status(default_db(db)?),
    }
}
