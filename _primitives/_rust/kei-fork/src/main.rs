//! kei-fork — CLI dispatcher.
//!
//! Single responsibility: parse args, dispatch to lib ops, print JSON.
//! Default `kit_root = std::env::current_dir()`.

use clap::{Parser, Subcommand};
use kei_fork::{collect, create, gc, list, rescue, ForkStatus};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "kei-fork", version, about = "Managed git-worktree + ledger lifecycle")]
struct Cli {
    /// Override kit_root (default: current dir).
    #[arg(long)]
    kit_root: Option<PathBuf>,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Spawn a new managed fork.
    Create {
        #[arg(long)]
        agent_id: String,
        #[arg(long, default_value = "main")]
        base: String,
    },
    /// Collect a done fork: commit, merge --no-ff, archive.
    Collect {
        #[arg(long)]
        agent_id: String,
        #[arg(long)]
        msg: String,
    },
    /// List forks, optionally filtered by status.
    List {
        /// active | done | stale | merged | all
        #[arg(long, default_value = "all")]
        status: String,
    },
    /// Prune stale forks (no .DONE and age ≥ --older-than hours).
    Gc {
        #[arg(long, default_value_t = 24)]
        older_than: u32,
    },
    /// Copy a fork's files out of band.
    Rescue {
        #[arg(long)]
        agent_id: String,
        #[arg(long)]
        out: PathBuf,
    },
}

fn resolve_kit_root(arg: Option<PathBuf>) -> PathBuf {
    arg.unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
}

fn err(msg: &str) -> ExitCode {
    eprintln!("kei-fork: {msg}");
    ExitCode::from(1)
}

fn parse_status_filter(raw: &str) -> Result<Option<ForkStatus>, String> {
    if raw.eq_ignore_ascii_case("all") {
        return Ok(None);
    }
    ForkStatus::from_cli(raw)
        .map(Some)
        .ok_or_else(|| format!("unknown status '{raw}'"))
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let kit_root = resolve_kit_root(cli.kit_root);
    match cli.cmd {
        Cmd::Create { agent_id, base } => run_create(&agent_id, &base, &kit_root),
        Cmd::Collect { agent_id, msg } => run_collect(&agent_id, &msg, &kit_root),
        Cmd::List { status } => run_list(&status, &kit_root),
        Cmd::Gc { older_than } => run_gc(older_than, &kit_root),
        Cmd::Rescue { agent_id, out } => run_rescue(&agent_id, &kit_root, &out),
    }
}

fn run_create(agent_id: &str, base: &str, kit_root: &std::path::Path) -> ExitCode {
    match create(agent_id, base, kit_root) {
        Ok(h) => print_json(&h),
        Err(e) => err(&e.to_string()),
    }
}

fn run_collect(agent_id: &str, msg: &str, kit_root: &std::path::Path) -> ExitCode {
    match collect(agent_id, msg, kit_root) {
        Ok(r) => print_json(&r),
        Err(e) => err(&e.to_string()),
    }
}

fn run_list(status: &str, kit_root: &std::path::Path) -> ExitCode {
    let filter = match parse_status_filter(status) {
        Ok(f) => f,
        Err(e) => return err(&e),
    };
    match list(kit_root, filter) {
        Ok(rows) => print_json(&rows),
        Err(e) => err(&e.to_string()),
    }
}

fn run_gc(older_than: u32, kit_root: &std::path::Path) -> ExitCode {
    match gc(kit_root, older_than) {
        Ok(r) => print_json(&r),
        Err(e) => err(&e.to_string()),
    }
}

fn run_rescue(agent_id: &str, kit_root: &std::path::Path, out: &std::path::Path) -> ExitCode {
    match rescue(agent_id, kit_root, out) {
        Ok(n) => {
            println!("{n}");
            ExitCode::SUCCESS
        }
        Err(e) => err(&e.to_string()),
    }
}

fn print_json<T: serde::Serialize>(v: &T) -> ExitCode {
    match serde_json::to_string_pretty(v) {
        Ok(s) => {
            println!("{s}");
            ExitCode::SUCCESS
        }
        Err(e) => err(&format!("json encode failed: {e}")),
    }
}
