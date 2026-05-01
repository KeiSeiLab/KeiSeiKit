//! kei-ledger — CLI dispatcher.
//!
//! Single responsibility: parse args, dispatch to ledger ops, format output.
//! Storage: `~/.claude/agents/ledger.sqlite` (or $KEI_LEDGER_DB override).
//!
//! Module tree: this binary depends on the `kei_ledger` library crate
//! (defined in `src/lib.rs`). The CLI dispatcher holds clap shapes and
//! glue only — every operation forwards to a library function.

mod dispatch;

use clap::{Parser, Subcommand};
use dispatch::{
    cmd_aggregate_skills, cmd_descendants, cmd_list, cmd_record_cost, cmd_tree, cmd_validate, err,
};
use kei_ledger::{ledger, schema};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "kei-ledger", version, about = "Agent fork/done/fail ledger")]
struct Cli {
    /// Override ledger path (default: $KEI_LEDGER_DB or ~/.claude/agents/ledger.sqlite)
    #[arg(long)]
    db: Option<PathBuf>,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Create the ledger file + schema if missing.
    Init,
    /// Log a new running agent.
    Fork {
        id: String,
        /// Branch name (<=256 chars).
        #[arg(value_parser = parse_branch)]
        branch: String,
        /// Parent branch (<=256 chars).
        #[arg(long, value_parser = parse_branch)]
        parent: Option<String>,
        #[arg(long)]
        spec_sha: String,
        #[arg(long)]
        worktree: Option<String>,
        /// Layer G DNA fingerprint (optional; kept blank for legacy callers).
        #[arg(long)]
        dna: Option<String>,
        /// DNA / human id of the agent that spawned this fork (v4 lineage).
        #[arg(long)]
        creator: Option<String>,
        /// DNA of the forked-from agent, if this is itself a fork (v4 lineage).
        #[arg(long = "fork-parent")]
        fork_parent: Option<String>,
    },
    /// Mark a running agent as done.
    Done {
        id: String,
        #[arg(long)]
        summary: String,
    },
    /// Mark a running agent as failed.
    Fail {
        id: String,
        #[arg(long)]
        reason: String,
    },
    /// Mark a done/failed agent as merged.
    Merged { id: String },
    /// List agents, optionally filtered by status.
    List {
        #[arg(long)]
        status: Option<String>,
    },
    /// Print parent -> children tree starting at a root agent id.
    Tree { id: String },
    /// Validate required artefact bundle for a given branch's agent.
    Validate {
        branch: String,
        #[arg(long, default_value = ".")]
        repo_root: PathBuf,
    },
    /// List agents whose fork_parent_id OR creator_id equals the given DNA.
    Descendants { dna: String },
    /// Aggregate skill_invocations for Phase D nightly decisions.
    AggregateSkills {
        /// Unix-second lower bound (default: now - 30 days).
        #[arg(long)]
        since: Option<i64>,
        /// Output format: json or markdown (default: markdown).
        #[arg(long, default_value = "markdown")]
        format: String,
    },
    /// Record cost-tracking metadata (v6+) for an existing agent row.
    /// Wave 44c: ADDITIVE by default — repeated calls accumulate. Pass
    /// `--replace` for legacy last-write-wins overwrite behavior.
    RecordCost {
        /// Agent id (matches `fork ... <id>`).
        agent_id: String,
        /// Cost in cents (integer ≥ 0). Capped at i64::MAX on extreme values.
        #[arg(long)]
        cents: u64,
        /// Provider name, e.g. "anthropic".
        #[arg(long)]
        provider: String,
        /// Model name, e.g. "claude-haiku-4-5-20251001".
        #[arg(long)]
        model: String,
        /// Overwrite previous cost (legacy semantics). Without this flag,
        /// the call accumulates with any prior recorded cost on the row.
        #[arg(long, default_value_t = false)]
        replace: bool,
    },
}

/// clap value_parser caps branch/parent length at MAX_BRANCH_LEN (audit L1).
fn parse_branch(s: &str) -> Result<String, String> {
    if s.len() > schema::MAX_BRANCH_LEN {
        return Err(format!(
            "branch length {} exceeds cap {}",
            s.len(),
            schema::MAX_BRANCH_LEN
        ));
    }
    Ok(s.to_string())
}

fn db_path(cli_db: Option<PathBuf>) -> PathBuf {
    if let Some(p) = cli_db {
        return p;
    }
    if let Ok(env) = std::env::var("KEI_LEDGER_DB") {
        return PathBuf::from(env);
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".claude/agents/ledger.sqlite")
}

#[allow(clippy::too_many_arguments)]
fn run_fork(
    conn: &rusqlite::Connection,
    id: String,
    branch: String,
    parent: Option<String>,
    spec_sha: String,
    worktree: Option<String>,
    dna: Option<String>,
    creator: Option<String>,
    fork_parent: Option<String>,
) -> ExitCode {
    match ledger::fork(
        conn,
        &id,
        &branch,
        parent.as_deref(),
        &spec_sha,
        worktree.as_deref(),
        dna.as_deref(),
        creator.as_deref(),
        fork_parent.as_deref(),
    ) {
        Ok(()) => {
            println!("forked {id} -> {branch}");
            ExitCode::SUCCESS
        }
        Err(e) => err(&format!("fork failed: {e}")),
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let path = db_path(cli.db);
    let conn = match ledger::open(&path) {
        Ok(c) => c,
        Err(e) => return err(&format!("open {}: {e}", path.display())),
    };
    match cli.cmd {
        Cmd::Init => {
            println!("initialised {}", path.display());
            ExitCode::SUCCESS
        }
        Cmd::Fork { id, branch, parent, spec_sha, worktree, dna, creator, fork_parent } => {
            run_fork(&conn, id, branch, parent, spec_sha, worktree, dna, creator, fork_parent)
        }
        Cmd::Done { id, summary } => match ledger::done(&conn, &id, &summary) {
            Ok(0) => err(&format!("no running agent with id {id}")),
            Ok(_) => ExitCode::SUCCESS,
            Err(e) => err(&format!("done failed: {e}")),
        },
        Cmd::Fail { id, reason } => match ledger::fail(&conn, &id, &reason) {
            Ok(0) => err(&format!("no running agent with id {id}")),
            Ok(_) => ExitCode::SUCCESS,
            Err(e) => err(&format!("fail update failed: {e}")),
        },
        Cmd::Merged { id } => match ledger::merged(&conn, &id) {
            Ok(0) => err(&format!("no done/failed agent with id {id}")),
            Ok(_) => ExitCode::SUCCESS,
            Err(e) => err(&format!("merged failed: {e}")),
        },
        Cmd::List { status } => cmd_list(&conn, status.as_deref()),
        Cmd::Tree { id } => cmd_tree(&conn, &id),
        Cmd::Validate { branch, repo_root } => cmd_validate(&branch, &repo_root),
        Cmd::Descendants { dna } => cmd_descendants(&conn, &dna),
        Cmd::AggregateSkills { since, format } => {
            cmd_aggregate_skills(&conn, since, &format)
        }
        Cmd::RecordCost { agent_id, cents, provider, model, replace } => {
            cmd_record_cost(&conn, &agent_id, cents, &provider, &model, replace)
        }
    }
}
