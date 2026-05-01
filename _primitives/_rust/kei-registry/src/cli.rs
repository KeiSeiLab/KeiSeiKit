//! clap surface — 8 subcommands.
//!
//! Constructor Pattern: this cube owns argument parsing only. Dispatch
//! lives in `main.rs`; each handler delegates to `registry`, `related`,
//! `diff`, or `stats` modules. Default `--db` is resolved at run-time so
//! the path expansion can react to `$HOME`.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "kei-registry",
    version,
    about = "Universal kit-block identity layer (primitive / skill / rule / hook / atom)"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Initialise the SQLite store at `--db`. Idempotent.
    Init {
        #[arg(long)]
        db: Option<PathBuf>,
    },

    /// Walk kit + claude dirs, register all detected blocks.
    Scan {
        #[arg(long)]
        kit_root: Option<PathBuf>,
        #[arg(long)]
        rules_root: Option<PathBuf>,
        #[arg(long)]
        hooks_root: Option<PathBuf>,
        #[arg(long)]
        db: Option<PathBuf>,
        /// Comma-separated subset of {primitive,skill,rule,hook,atom}.
        #[arg(long)]
        types: Option<String>,
    },

    /// Manually register a single block.
    Register {
        block_type: String,
        path: PathBuf,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        caps: Option<String>,
        #[arg(long)]
        db: Option<PathBuf>,
    },

    /// List blocks (active by default).
    List {
        /// Restrict to one block_type.
        #[arg(long = "type", value_name = "TYPE")]
        block_type: Option<String>,
        #[arg(long)]
        db: Option<PathBuf>,
        #[arg(long, default_value_t = 1024)]
        limit: i64,
        #[arg(long)]
        include_superseded: bool,
    },

    /// Fetch a single block by integer id OR full DNA.
    Get {
        target: String,
        #[arg(long)]
        db: Option<PathBuf>,
    },

    /// Find blocks whose body references the target.
    Related {
        target: String,
        #[arg(long, default_value_t = 1)]
        depth: u32,
        #[arg(long)]
        db: Option<PathBuf>,
    },

    /// Facet-by-facet diff between two blocks.
    Diff {
        a: String,
        b: String,
        #[arg(long)]
        db: Option<PathBuf>,
    },

    /// Aggregate counts.
    Stats {
        #[arg(long)]
        db: Option<PathBuf>,
    },

    /// Generate a human-readable encyclopedia from registry rows.
    Encyclopedia {
        /// Path to the registry SQLite (default: $KEI_REGISTRY_DB or ~/.claude/registry.sqlite).
        #[arg(long = "registry-db")]
        registry_db: Option<PathBuf>,
        /// Write output to this path instead of stdout.
        #[arg(long)]
        output: Option<PathBuf>,
        /// Output format: markdown (default) or json.
        #[arg(long, default_value = "markdown")]
        format: String,
        /// Restrict to one block_type (primitive | skill | rule | hook | atom).
        #[arg(long = "type", value_name = "TYPE")]
        block_type: Option<String>,
    },

    /// Shorthand: register a single skill directory (must contain SKILL.md).
    RegisterSkill {
        /// Path to the skill directory (containing SKILL.md). Defaults to `.`.
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        db: Option<PathBuf>,
    },

    /// Shorthand: register a single .sh hook file.
    RegisterHook {
        path: PathBuf,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        db: Option<PathBuf>,
    },

    /// Walk all substrate dirs under KIT-ROOT and register each artefact.
    /// Idempotent: unchanged content is a no-op; changed content creates a
    /// supersede chain. Covers: primitives, skills, hooks, atoms, blocks,
    /// capabilities, roles. Manifests are skipped (not blocks).
    IndexSubstrate {
        /// Path to the kit root (defaults to current directory).
        #[arg(default_value = ".")]
        kit_root: PathBuf,
        #[arg(long)]
        db: Option<PathBuf>,
        /// Print counts without writing to the registry.
        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },
}
