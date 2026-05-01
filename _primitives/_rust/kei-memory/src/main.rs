//! kei-memory — offline session analyzer (binary entrypoint).
//!
//! Constructor Pattern: main.rs only dispatches; work lives in cubes.
//! Storage: `~/.claude/memory/kei-memory.sqlite` (or $KEI_MEMORY_DB).
//! RULE 0.14 — session self-audit, silent-first until 10 sessions ingested.

use kei_memory::{backlog, commands, schema};

use clap::{Parser, Subcommand};
use rusqlite::Connection;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "kei-memory", version, about = "Offline session retrospective (RULE 0.14)")]
struct Cli {
    /// Override DB path (default: $KEI_MEMORY_DB or ~/.claude/memory/kei-memory.sqlite)
    #[arg(long)]
    db: Option<PathBuf>,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Read a JSONL transcript and insert session + events.
    Ingest {
        #[arg(long)]
        session_id: String,
        #[arg(long)]
        transcript: PathBuf,
        #[arg(long)]
        prompt: Option<String>,
    },
    /// Print a retrospective for a session or the last N sessions.
    Analyze {
        #[arg(long)]
        session: Option<String>,
        #[arg(long, default_value_t = 1)]
        last: usize,
        #[arg(long)]
        summary: bool,
    },
    /// List recurring event-class patterns.
    Patterns {
        #[arg(long)]
        cross_session: bool,
        #[arg(long)]
        session: Option<String>,
    },
    /// Top-k past sessions by TF-IDF cosine similarity to the query text.
    Similar {
        prompt: String,
        #[arg(long, default_value_t = 5)]
        limit: usize,
    },
    /// Dump a session's events as markdown to stdout.
    Dump { session_id: String },
    /// N sessions, N events, top tools.
    Stats,
    /// Manage the silent-first audit backlog items.
    Backlog {
        #[arg(long)]
        add: Option<String>,
        #[arg(long)]
        list: bool,
        #[arg(long)]
        clear: bool,
    },
}

fn db_path(cli_db: Option<PathBuf>) -> PathBuf {
    if let Some(p) = cli_db {
        return p;
    }
    if let Ok(e) = std::env::var("KEI_MEMORY_DB") {
        return PathBuf::from(e);
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".claude/memory/kei-memory.sqlite")
}

fn open_db(path: &PathBuf) -> rusqlite::Result<Connection> {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let conn = Connection::open(path)?;
    schema::migrate(&conn)?;
    Ok(conn)
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let path = db_path(cli.db);
    let conn = match open_db(&path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("kei-memory: open {}: {e}", path.display());
            return ExitCode::from(1);
        }
    };
    match cli.cmd {
        Cmd::Ingest { session_id, transcript, prompt } => {
            commands::cmd_ingest(&conn, &session_id, &transcript, prompt)
        }
        Cmd::Analyze { session, last, summary } => {
            commands::cmd_analyze(&conn, session, last, summary)
        }
        Cmd::Patterns { cross_session, session } => {
            commands::cmd_patterns(&conn, cross_session, session)
        }
        Cmd::Similar { prompt, limit } => commands::cmd_similar(&conn, &prompt, limit),
        Cmd::Dump { session_id } => commands::cmd_dump(&conn, &session_id),
        Cmd::Stats => commands::cmd_stats(&conn),
        Cmd::Backlog { add, list, clear } => backlog::cmd_backlog(&conn, add, list, clear),
    }
}
