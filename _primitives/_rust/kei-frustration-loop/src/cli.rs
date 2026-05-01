//! CLI surface — clap parser + 5 loop subcommands + dispatch.
//!
//! Constructor Pattern: this cube owns the clap `Cli`/`Cmd` definitions
//! and routes each variant to a runner in `runners.rs`. Keep <200 LOC.

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::runners;

/// Top-level CLI surface for the per-user frustration learning loop.
#[derive(Parser)]
#[command(
    name = "kei-frustration-loop",
    version,
    about = "Per-user frustration learning loop — bootstrap / nightly-scan / feedback / auto-train / personalize"
)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Cmd,
}

/// Five loop subcommands. Order = the order they show in `--help`.
#[derive(Subcommand)]
pub enum Cmd {
    /// First-install bootstrap — train per-user firmware + queue initial hits.
    Bootstrap {
        #[arg(long, default_value = "default")]
        user: String,
        #[arg(long)]
        home: Option<PathBuf>,
    },
    /// Phase-0 nightly scan over chatlogs since `--since` (Unix seconds).
    NightlyScan {
        #[arg(long, default_value = "default")]
        user: String,
        #[arg(long)]
        since: Option<u64>,
        #[arg(long)]
        home: Option<PathBuf>,
    },
    /// Append one user-correction row to the per-user feedback log.
    Feedback {
        hit_id: String,
        #[arg(long)]
        label: String,
        #[arg(long, default_value = "default")]
        user: String,
        #[arg(long)]
        home: Option<PathBuf>,
        #[arg(long, default_value = "")]
        message: String,
        #[arg(long, default_value = "")]
        category: String,
    },
    /// Trigger per-user retrain when feedback log clears the threshold.
    AutoTrain {
        #[arg(long, default_value = "default")]
        user: String,
        #[arg(long)]
        threshold: Option<usize>,
        #[arg(long)]
        home: Option<PathBuf>,
        #[arg(long)]
        traces_dir: Option<PathBuf>,
    },
    /// Inspect which firmware will be used for `--user`.
    Personalize {
        #[arg(long, default_value = "default")]
        user: String,
        #[arg(long)]
        home: Option<PathBuf>,
    },
}

/// Dispatch the parsed CLI to the matching runner.
pub fn dispatch(cli: Cli) -> Result<()> {
    match cli.cmd {
        Cmd::Bootstrap { user, home } => runners::run_bootstrap(&user, home.as_deref()),
        Cmd::NightlyScan { user, since, home } => {
            runners::run_nightly(&user, since, home.as_deref())
        }
        Cmd::Feedback { hit_id, label, user, home, message, category } => {
            runners::run_feedback(&hit_id, &label, &user, home.as_deref(), &message, &category)
        }
        Cmd::AutoTrain { user, threshold, home, traces_dir } => {
            runners::run_auto_train(&user, threshold, home.as_deref(), traces_dir.as_deref())
        }
        Cmd::Personalize { user, home } => runners::run_personalize(&user, home.as_deref()),
    }
}
