//! kei-prune CLI dispatcher.
//!
//! Constructor Pattern: one cube = clap wiring + three verb handlers.
//! Each handler is <15 LOC and delegates immediately to the library.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use kei_prune::{candidates, ensure_schema, mark_retired, stats, PruneError};
use rusqlite::Connection;

/// Default ledger location — mirrors RULE 0.12 SSoT path.
const DEFAULT_DB: &str = "~/.claude/agents/ledger.sqlite";

#[derive(Parser, Debug)]
#[command(
    name = "kei-prune",
    version,
    about = "Mark unused kei-ledger agents as retired (sidecar-only, non-destructive)"
)]
struct Cli {
    /// SQLite database path. Defaults to the RULE 0.12 ledger SSoT.
    #[arg(long, global = true, default_value = DEFAULT_DB)]
    db: String,

    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// List agents eligible for retirement (JSON array).
    List {
        /// Minimum idle days since `started_ts`.
        #[arg(long, default_value_t = 90)]
        idle_days: u32,
    },
    /// Mark a specific agent id as retired (idempotent).
    Mark {
        /// Ledger `agents.id` to retire.
        #[arg(long)]
        id: String,
    },
    /// Emit bucket counts (total / active / idle / retired) as JSON.
    Stats,
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("kei-prune error: {e}");
            ExitCode::from(1)
        }
    }
}

/// Dispatch to the per-verb handler after opening the DB + schema.
fn run(cli: Cli) -> Result<(), PruneError> {
    let db_path = expand_tilde(&cli.db);
    let conn = Connection::open(&db_path)?;
    ensure_schema(&conn)?;
    match cli.cmd {
        Cmd::List { idle_days } => verb_list(&conn, idle_days),
        Cmd::Mark { id } => verb_mark(&conn, &id),
        Cmd::Stats => verb_stats(&conn),
    }
}

/// `list` verb — emit JSON array of candidates to stdout.
fn verb_list(conn: &Connection, idle_days: u32) -> Result<(), PruneError> {
    let now = now_seconds();
    let rows = candidates(conn, now, idle_days)?;
    println!("{}", serde_json::to_string_pretty(&rows)?);
    Ok(())
}

/// `mark` verb — record retirement, echo one-line JSON confirmation.
fn verb_mark(conn: &Connection, id: &str) -> Result<(), PruneError> {
    let now = now_seconds();
    mark_retired(conn, id, now)?;
    let msg = serde_json::json!({ "retired": id, "retired_ts": now });
    println!("{}", msg);
    Ok(())
}

/// `stats` verb — emit JSON object of fleet counts.
fn verb_stats(conn: &Connection) -> Result<(), PruneError> {
    let s = stats(conn)?;
    println!("{}", serde_json::to_string_pretty(&s)?);
    Ok(())
}

/// Expand a leading `~/` to `$HOME/`. No-op otherwise.
fn expand_tilde(path: &str) -> PathBuf {
    if let Some(stripped) = path.strip_prefix("~/") {
        if let Some(home) = std::env::var_os("HOME") {
            let mut pb = PathBuf::from(home);
            pb.push(stripped);
            return pb;
        }
    }
    PathBuf::from(path)
}

/// Current unix time in whole seconds. Isolated for test override and
/// to avoid pulling chrono for a single now() call.
fn now_seconds() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}
