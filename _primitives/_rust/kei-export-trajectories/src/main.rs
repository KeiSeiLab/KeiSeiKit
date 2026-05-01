//! kei-export-trajectories CLI.
//!
//! Subcommands:
//!   export --from-ts <ISO> --output <path.jsonl>
//!   count  --from-ts <ISO>
//!   verify <path.jsonl>
//!
//! The `verify` command re-reads the JSONL we just wrote and confirms
//! the union-of-tool-stats invariant — it's how Phase 0.2 acceptance is
//! checked in CI without a separate Python reader.

use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use kei_export_trajectories::{
    normalize_keys, record_to_trajectory, write_jsonl, LedgerReader, Trajectory,
};
use std::collections::BTreeSet;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "kei-export-trajectories", version)]
struct Cli {
    /// Path to kei-ledger.sqlite. Defaults to
    /// `~/.claude/agents/ledger.sqlite`.
    #[arg(long, global = true)]
    ledger: Option<PathBuf>,
    /// Path to kei-memory.sqlite. Defaults to
    /// `~/.claude/memory/kei-memory.sqlite` if it exists.
    #[arg(long, global = true)]
    memory: Option<PathBuf>,
    /// Repo root for resolving `.claude/agents/<id>/chatlog.md`.
    #[arg(long, global = true)]
    repo_root: Option<PathBuf>,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Emit ShareGPT JSONL for every agent with started_ts >= --from-ts.
    Export {
        #[arg(long)]
        from_ts: String,
        #[arg(long)]
        output: PathBuf,
    },
    /// Count agents matching the same predicate; print to stdout.
    Count {
        #[arg(long)]
        from_ts: String,
    },
    /// Re-read a JSONL and confirm key-set invariants.
    Verify { path: PathBuf },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.cmd {
        Cmd::Export { from_ts, output } => run_export(&cli, from_ts, output),
        Cmd::Count { from_ts } => run_count(&cli, from_ts),
        Cmd::Verify { path } => run_verify(path),
    }
}

fn run_export(cli: &Cli, from_ts: &str, output: &PathBuf) -> Result<()> {
    let reader = build_reader(cli);
    let records = reader.read_since(parse_iso(from_ts)?)?;
    let mut trajs: Vec<Trajectory> = records
        .iter()
        .enumerate()
        .map(|(i, r)| record_to_trajectory(i as u64, r))
        .collect();
    normalize_keys(&mut trajs);
    write_jsonl(output, &trajs)?;
    println!("wrote {} trajectories to {}", trajs.len(), output.display());
    Ok(())
}

fn run_count(cli: &Cli, from_ts: &str) -> Result<()> {
    let n = build_reader(cli).count_since(parse_iso(from_ts)?)?;
    println!("{n}");
    Ok(())
}

fn run_verify(path: &PathBuf) -> Result<()> {
    let txt = std::fs::read_to_string(path)
        .with_context(|| format!("read {}", path.display()))?;
    let trajs: Vec<Trajectory> = txt
        .lines()
        .filter(|l| !l.is_empty())
        .map(serde_json::from_str)
        .collect::<Result<Vec<_>, _>>()
        .context("parse jsonl")?;
    let union: BTreeSet<&String> =
        trajs.iter().flat_map(|t| t.tool_stats.keys()).collect();
    for (i, t) in trajs.iter().enumerate() {
        let keys: BTreeSet<&String> = t.tool_stats.keys().collect();
        if keys != union {
            return Err(anyhow!("line {i}: tool_stats key set != union"));
        }
    }
    println!(
        "verified {} trajectories, {} tools in union",
        trajs.len(),
        union.len()
    );
    Ok(())
}

fn build_reader(cli: &Cli) -> LedgerReader {
    let ledger = cli.ledger.clone().unwrap_or_else(default_ledger_path);
    let mut r = LedgerReader::new(ledger);
    if let Some(m) = cli.memory.clone().or_else(default_memory_path) {
        r = r.with_memory(m);
    }
    if let Some(rr) = cli.repo_root.clone() {
        r = r.with_repo_root(rr);
    }
    r
}

fn default_ledger_path() -> PathBuf {
    home().join(".claude").join("agents").join("ledger.sqlite")
}

fn default_memory_path() -> Option<PathBuf> {
    let p = home().join(".claude").join("memory").join("kei-memory.sqlite");
    if p.is_file() { Some(p) } else { None }
}

fn home() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/"))
}

/// Parse ISO-8601 (date or full timestamp) into Unix epoch seconds.
/// Accepts either `2026-04-01` (UTC midnight) or full RFC3339
/// `2026-04-01T12:00:00Z`.
fn parse_iso(s: &str) -> Result<i64> {
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(s) {
        return Ok(dt.timestamp());
    }
    if let Ok(d) = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        if let Some(dt) = d.and_hms_opt(0, 0, 0) {
            return Ok(dt.and_utc().timestamp());
        }
    }
    Err(anyhow!(
        "unparseable --from-ts: {s} (want RFC3339 or YYYY-MM-DD)"
    ))
}
