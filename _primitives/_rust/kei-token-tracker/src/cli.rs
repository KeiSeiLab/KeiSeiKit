//! Clap dispatch for the `kei-token-tracker` CLI binary.

use std::path::PathBuf;
use std::process::ExitCode;

use chrono::{TimeZone, Utc};
use clap::{Parser, Subcommand};

use crate::aggregate::format_usd;
use crate::sleep_report;
use crate::store::Store;

#[derive(Parser)]
#[command(name = "kei-token-tracker", version, about = "Per-LLM-call token + cost store")]
pub struct Cli {
    #[arg(long, global = true)]
    pub db: Option<PathBuf>,
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    /// Print total event count.
    Count,
    /// List most recent events (default 20).
    List {
        #[arg(long, default_value_t = 20)]
        limit: u32,
    },
    /// Aggregate by model since N days ago.
    Aggregate {
        #[arg(long, default_value_t = 1)]
        since_days: u32,
    },
    /// Render the Phase D nightly markdown report.
    SleepReport {
        #[arg(long, default_value_t = 1)]
        since_days: u32,
        /// Optional output path; default = stdout.
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

/// Entry point; returns a process exit code so the bin can exit cleanly.
pub fn run(cli: Cli) -> ExitCode {
    let store = match open_store(cli.db.as_deref()) {
        Ok(s) => s,
        Err(e) => return err(&format!("open: {e}")),
    };
    match cli.cmd {
        Cmd::Count => cmd_count(&store),
        Cmd::List { limit } => cmd_list(&store, limit),
        Cmd::Aggregate { since_days } => cmd_aggregate(&store, since_days),
        Cmd::SleepReport { since_days, out } => cmd_sleep_report(&store, since_days, out),
    }
}

fn open_store(db: Option<&std::path::Path>) -> Result<Store, crate::error::Error> {
    match db {
        Some(p) => Store::open(p),
        None => Store::open_in_memory(),
    }
}

fn cmd_count(store: &Store) -> ExitCode {
    match store.count() {
        Ok(n) => {
            println!("{n}");
            ExitCode::SUCCESS
        }
        Err(e) => err(&format!("count: {e}")),
    }
}

fn cmd_list(store: &Store, limit: u32) -> ExitCode {
    match store.list_recent(limit) {
        Ok(rows) => {
            for r in &rows {
                let line = serde_json::to_string(r).unwrap_or_else(|_| "{}".into());
                println!("{line}");
            }
            ExitCode::SUCCESS
        }
        Err(e) => err(&format!("list: {e}")),
    }
}

fn cmd_aggregate(store: &Store, since_days: u32) -> ExitCode {
    let since = since_unix(since_days);
    match store.aggregate_by_model(since) {
        Ok(rows) => {
            for r in &rows {
                println!(
                    "{}\t{}\t{}\t{}\t{}",
                    r.model,
                    r.events,
                    r.input_tokens,
                    r.output_tokens,
                    format_usd(r.micro_cents),
                );
            }
            ExitCode::SUCCESS
        }
        Err(e) => err(&format!("aggregate: {e}")),
    }
}

fn cmd_sleep_report(store: &Store, since_days: u32, out: Option<PathBuf>) -> ExitCode {
    let since = since_unix(since_days);
    let rows = match store.aggregate_by_model(since) {
        Ok(r) => r,
        Err(e) => return err(&format!("aggregate: {e}")),
    };
    let date = Utc
        .timestamp_opt(since, 0)
        .single()
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "unknown".into());
    let md = sleep_report::render(&date, &rows);
    match out {
        Some(p) => match std::fs::write(&p, md) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => err(&format!("write {}: {e}", p.display())),
        },
        None => {
            print!("{md}");
            ExitCode::SUCCESS
        }
    }
}

fn since_unix(days: u32) -> i64 {
    let now = Utc::now().timestamp();
    let secs = (days as i64).saturating_mul(86_400);
    now.saturating_sub(secs)
}

fn err(msg: &str) -> ExitCode {
    eprintln!("kei-token-tracker: {msg}");
    ExitCode::FAILURE
}
