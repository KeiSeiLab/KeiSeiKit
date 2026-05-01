//! Clap definitions — 5 subcommands.
//!
//! Subcommand surface kept stable; main.rs dispatches to module entrypoints.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "kei-decompose",
    version,
    about = "UNIVERSAL decomposition layer — ANY MD output → kei-spawn task.toml + dispatch."
)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Cmd,
}

/// Forced-format flag value.
///
/// `auto` triggers detect-then-parse; the other variants short-circuit
/// detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
#[clap(rename_all = "kebab-case")]
pub enum FormatHint {
    Auto,
    Research,
    Audit,
    Sleep,
    Architecture,
    NewProject,
}

#[derive(Subcommand, Debug)]
pub enum Cmd {
    /// Auto-detect which parser claims this MD file.
    Detect {
        /// Path to the markdown file.
        md: PathBuf,
    },

    /// Parse using detected (or forced) format → JSON Action[].
    Parse {
        /// Path to the markdown file.
        md: PathBuf,
        /// Force a specific format. Default: auto-detect.
        #[arg(long, value_enum, default_value_t = FormatHint::Auto)]
        format: FormatHint,
    },

    /// Parse + emit one task.toml per Action under <out>.
    Emit {
        /// Path to the markdown file.
        md: PathBuf,
        /// Output directory for the emitted task.toml files.
        #[arg(long)]
        out: PathBuf,
        /// Force a specific format. Default: auto-detect.
        #[arg(long, value_enum, default_value_t = FormatHint::Auto)]
        format: FormatHint,
    },

    /// Full chain: parse → emit → kei-spawn each → kei-ledger pre-fork.
    Dispatch {
        /// Path to the markdown file.
        md: PathBuf,
        /// Skip kei-spawn invocation; only emit and report intent.
        #[arg(long)]
        dry_run: bool,
        /// Cap the number of actions dispatched.
        #[arg(long)]
        limit: Option<usize>,
        /// Force a specific format. Default: auto-detect.
        #[arg(long, value_enum, default_value_t = FormatHint::Auto)]
        format: FormatHint,
        /// Skip kei-ledger pre-fork registration.
        #[arg(long)]
        no_ledger: bool,
    },

    /// List registered parsers + their detection signatures.
    Formats,

    /// Walk rule files, split into sections, register each in kei-registry.
    ///
    /// Walks `--rules-dir/*.md`, `specialty/*.md`, and `projects/*.md` (depth
    /// ≤ 2). Skips files starting with `_` and `RULES.md` (the registry index,
    /// not a rule). Each H2 section becomes one `BlockType::Rule` entry in the
    /// SQLite registry.
    ///
    /// Fragment bodies are written to `--fragments-dir` (real filesystem paths)
    /// so that the `_assembler` can `fs::read_to_string` them directly.
    /// Default fragments dir: `~/.claude/registry-fragments`
    /// Env override: `KEI_FRAGMENTS_DIR`.
    DecomposeRules {
        /// Root directory containing rule `.md` files.
        /// Default: `~/.claude/rules`
        #[arg(long)]
        rules_dir: Option<PathBuf>,

        /// Path to the registry SQLite database.
        /// Default: `~/.claude/registry.sqlite`
        #[arg(long)]
        registry_db: Option<PathBuf>,

        /// Directory where fragment `.md` files are stored on disk.
        /// Default: `~/.claude/registry-fragments` (or `KEI_FRAGMENTS_DIR` env).
        #[arg(long)]
        fragments_dir: Option<PathBuf>,

        /// Print what would be registered/written without doing either.
        #[arg(long)]
        dry_run: bool,

        /// Re-extract ALL existing rule-type rows in the registry to the
        /// canonical fragments dir and update their `path` column.
        /// One-time migration for rows registered with the old `file::section`
        /// logical key format.
        #[arg(long)]
        rebuild_fragments: bool,
    },
}
