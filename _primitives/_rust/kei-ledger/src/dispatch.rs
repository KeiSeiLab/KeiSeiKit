//! CLI dispatch helpers — one function per subcommand.
//!
//! Constructor Pattern: extracted from `main.rs` to keep the entry point
//! under the 200-LOC cap. Each fn returns `ExitCode` directly so `main`
//! stays a flat match.
//!
//! Module owner: the binary crate. Pulls library functions from the
//! `kei_ledger` crate (defined in `src/lib.rs`).

use kei_ledger::{cost, descendants, ledger, skill_aggregator_cli};
use rusqlite::Connection;
use serde_json::json;
use std::path::Path;
use std::process::ExitCode;

pub fn err(msg: &str) -> ExitCode {
    eprintln!("kei-ledger: {msg}");
    ExitCode::from(1)
}

pub fn cmd_list(conn: &Connection, status: Option<&str>) -> ExitCode {
    match ledger::list(conn, status) {
        Ok(rows) => {
            if rows.is_empty() {
                println!("(no agents)");
            }
            for r in &rows {
                println!(
                    "{}\t{}\t{}\t{}\tparent={}\tspec={}",
                    r.id,
                    r.status,
                    r.branch,
                    r.started_ts,
                    r.parent_branch.as_deref().unwrap_or("-"),
                    &r.spec_sha[..r.spec_sha.len().min(12)]
                );
            }
            ExitCode::SUCCESS
        }
        Err(e) => err(&format!("list failed: {e}")),
    }
}

pub fn cmd_tree(conn: &Connection, id: &str) -> ExitCode {
    match ledger::tree(conn, id) {
        Ok(rows) if rows.is_empty() => err(&format!("no agent with id {id}")),
        Ok(rows) => {
            for r in &rows {
                let indent = if r.id == id { "" } else { "  " };
                println!("{}{} [{}] branch={}", indent, r.id, r.status, r.branch);
            }
            ExitCode::SUCCESS
        }
        Err(e) => err(&format!("tree failed: {e}")),
    }
}

pub fn cmd_validate(branch: &str, repo_root: &Path) -> ExitCode {
    // branch naming convention: agent/<kind>-<ts>  OR  inline-<ts>
    // ledger artefact dir uses the raw agent id, which the caller passes as branch.
    let agent_id = branch.strip_prefix("agent/").unwrap_or(branch);
    let missing = ledger::validate(repo_root, agent_id);
    if missing.is_empty() {
        println!("OK: all 6 artefacts present for {agent_id}");
        ExitCode::SUCCESS
    } else {
        eprintln!("MISSING for {agent_id}:");
        for m in &missing {
            eprintln!("  - {m}");
        }
        ExitCode::from(2)
    }
}

pub fn cmd_descendants(conn: &Connection, dna: &str) -> ExitCode {
    match descendants::descendants(conn, dna) {
        Ok(rows) => {
            if rows.is_empty() {
                println!("(no descendants for {dna})");
            }
            for r in &rows {
                let relation = if r.fork_parent_id.as_deref() == Some(dna) {
                    "fork"
                } else {
                    "spawn"
                };
                println!("{}\t{}\t{}\t{}", r.id, relation, r.status, r.branch);
            }
            ExitCode::SUCCESS
        }
        Err(e) => err(&format!("descendants failed: {e}")),
    }
}

/// Record cost metadata for an existing agent. Emits JSON to stdout so
/// callers (cortex, scripts) can pipe through `jq`. Exit code 1 if the
/// agent does not exist (zero rows updated), 0 otherwise. Schema must be
/// at v6+ — `kei-ledger init` migrates legacy ledgers automatically on
/// open before this dispatcher runs.
///
/// Wave 44c: ADDITIVE semantics — repeated calls accumulate cost_cents
/// for the same agent. Use `--replace` for the legacy overwrite
/// behavior (typically only retry / amend flows).
pub fn cmd_record_cost(
    conn: &Connection,
    agent_id: &str,
    cents: u64,
    provider: &str,
    model: &str,
    replace: bool,
) -> ExitCode {
    let result = if replace {
        cost::replace_cost(conn, agent_id, cents, provider, model)
    } else {
        cost::record_cost(conn, agent_id, cents, provider, model)
    };
    match result {
        Ok(0) => err(&format!("no agent with id {agent_id}")),
        Ok(_) => emit_record_cost_json(conn, agent_id),
        Err(e) => err(&format!("record-cost failed: {e}")),
    }
}

/// Thin pass-through so `main.rs` keeps all cmd_* in one import namespace.
pub fn cmd_aggregate_skills(
    conn: &Connection,
    since: Option<i64>,
    format: &str,
) -> ExitCode {
    skill_aggregator_cli::cmd_aggregate_skills(conn, since, format)
}

/// Emit the post-write JSON line. Split out to keep `cmd_record_cost`
/// flat and ≤30 LOC after the `--replace` branch was added.
fn emit_record_cost_json(conn: &Connection, agent_id: &str) -> ExitCode {
    match cost::read_cost(conn, agent_id) {
        Ok(Some((total, _, _))) => {
            let body = json!({
                "ok": true,
                "agent_id": agent_id,
                "total_cost_cents": total,
            });
            println!("{body}");
            ExitCode::SUCCESS
        }
        Ok(None) => err(&format!("agent {agent_id} disappeared mid-write")),
        Err(e) => err(&format!("read-back failed: {e}")),
    }
}
