//! CLI shapes for `kei-decision`. Five subcommands, dispatched in `main.rs`.
//!
//! Exit codes (per spec):
//!   0 — success
//!   1 — file / IO error
//!   2 — no actions found / parse error
//!   3 — kei-spawn invocation failed

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "kei-decision",
    version,
    about = "Bridge research MASTER-REPORT.md to kei-spawn task.toml + kei-ledger pre-fork"
)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Subcommand, Debug)]
pub enum Cmd {
    /// Parse a master report and emit JSON of raw actions (with kind).
    Parse {
        /// Path to MASTER-REPORT.md.
        master: PathBuf,
    },
    /// Parse + classify + topo-sort + score-rank.
    Rank {
        #[arg(long)]
        master: PathBuf,
        /// Truncate to top N (default: all).
        #[arg(long)]
        limit: Option<usize>,
        /// Emit markdown table instead of JSON.
        #[arg(long, default_value_t = false)]
        markdown: bool,
    },
    /// Parse + rank + emit one task.toml per action under <out>/.
    Plan {
        #[arg(long)]
        master: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
    /// Full chain: parse + rank + emit + kei-spawn + (optional) kei-ledger.
    Execute {
        #[arg(long)]
        master: PathBuf,
        /// Skip kei-spawn invocation; only emit task files.
        #[arg(long, default_value_t = false)]
        dry_run: bool,
        /// Truncate to top N (default: all).
        #[arg(long)]
        limit: Option<usize>,
        /// Skip pre-fork ledger row.
        #[arg(long, default_value_t = false)]
        no_ledger: bool,
    },
    /// Scan a research dir tree, ingest each MASTER-REPORT.md into a cumulative
    /// graph. Optional --graph-out path.
    Link {
        /// Root directory to walk (e.g. ~/Projects/KnowledgeVault/research).
        research_dir: PathBuf,
        #[arg(long)]
        graph_out: Option<PathBuf>,
    },
}
