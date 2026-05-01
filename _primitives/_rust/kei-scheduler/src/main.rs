//! kei-scheduler CLI — schedule / cancel / list-due / mark-run / tick.
//!
//! Exit-code contract:
//! - 0 — success
//! - 1 — IO / storage / usage
//! - 2 — validation (bad trigger kind / spec / unknown id)

use chrono::Utc;
use clap::{Parser, Subcommand};
use kei_scheduler::{
    cancel, get_task, list_due, mark_run, schedule, Error, Store,
};
use std::path::PathBuf;
use std::process::ExitCode;

struct CliError {
    code: u8,
    msg: String,
}

impl CliError {
    fn io(msg: impl Into<String>) -> Self { Self { code: 1, msg: msg.into() } }
    fn validation(msg: impl Into<String>) -> Self { Self { code: 2, msg: msg.into() } }
}

impl From<anyhow::Error> for CliError {
    fn from(e: anyhow::Error) -> Self { Self::io(format!("{e:#}")) }
}

impl From<Error> for CliError {
    fn from(e: Error) -> Self {
        match &e {
            Error::Parse(_) | Error::NotFound(_) | Error::NameExists(_) =>
                Self::validation(format!("{e}")),
            _ => Self::io(format!("{e}")),
        }
    }
}

#[derive(Parser)]
#[command(name = "kei-scheduler", version, about = "Durable task scheduler (cron/at/interval)")]
struct Cli {
    #[arg(long)]
    db: Option<PathBuf>,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Insert a new scheduled task.
    Schedule {
        #[arg(long)] name: String,
        #[arg(long)] kind: String,
        #[arg(long)] spec: String,
        #[arg(long)] cmd: String,
    },
    /// Cancel a task by id.
    Cancel { #[arg(long)] id: i64 },
    /// Print due tasks as a JSON array (reads `now = Utc::now`).
    ListDue,
    /// Record a run's exit code and advance next_run_at.
    MarkRun {
        #[arg(long)] id: i64,
        #[arg(long)] exit: i64,
    },
    /// Convenience: `list-due` for the current wall clock.
    Tick,
    /// Print one task as JSON.
    Get { #[arg(long)] id: i64 },
}

fn db_path(cli_db: Option<PathBuf>) -> PathBuf {
    if let Some(p) = cli_db { return p; }
    if let Ok(e) = std::env::var("KEI_SCHEDULER_DB") { return PathBuf::from(e); }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".claude/scheduler/scheduler.sqlite")
}

fn run() -> Result<(), CliError> {
    let cli = Cli::parse();
    let store = Store::open(&db_path(cli.db))?;
    dispatch(&store, cli.cmd)
}

fn dispatch(store: &Store, cmd: Cmd) -> Result<(), CliError> {
    match cmd {
        Cmd::Schedule { name, kind, spec, cmd } =>
            cmd_schedule(store, &name, &kind, &spec, &cmd),
        Cmd::Cancel { id } => { cancel(store.conn(), id)?; println!("cancelled {id}"); Ok(()) }
        Cmd::ListDue | Cmd::Tick => cmd_list_due(store),
        Cmd::MarkRun { id, exit } => {
            mark_run(store.conn(), id, exit, Utc::now().timestamp())?;
            println!("marked run {id} exit={exit}");
            Ok(())
        }
        Cmd::Get { id } => cmd_get(store, id),
    }
}

fn cmd_schedule(
    store: &Store,
    name: &str,
    kind: &str,
    spec: &str,
    cmd: &str,
) -> Result<(), CliError> {
    let id = schedule(store.conn(), name, kind, spec, cmd)?;
    println!("{id}");
    Ok(())
}

fn cmd_list_due(store: &Store) -> Result<(), CliError> {
    let now = Utc::now().timestamp();
    let rows = list_due(store.conn(), now)?;
    let json = serde_json::to_string_pretty(&rows).map_err(|e| CliError::io(e.to_string()))?;
    println!("{json}");
    Ok(())
}

fn cmd_get(store: &Store, id: i64) -> Result<(), CliError> {
    match get_task(store.conn(), id)? {
        Some(t) => {
            let json = serde_json::to_string_pretty(&t)
                .map_err(|e| CliError::io(e.to_string()))?;
            println!("{json}");
            Ok(())
        }
        None => Err(CliError::validation(format!("task not found: id={id}"))),
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(CliError { code, msg }) => {
            eprintln!("{msg}");
            ExitCode::from(code)
        }
    }
}
