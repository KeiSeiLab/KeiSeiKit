//! executor — build per-phase execution plan; optionally pre-register in kei-ledger.
//!
//! Constructor Pattern: one responsibility, ≤200 LOC, ≤30 LOC per fn.
//! Does NOT spawn agents — callers feed each prompt to Agent({...}) externally.

use crate::phase_prompt::{build_phase_prompt, PhasePrompt};
use crate::plan_parser::ParsedPlan;
use anyhow::{Context, Result};
use std::path::Path;

// ─────────────────────────── public types ──────────────────────────────────

/// Lifecycle status of a phase in the execution plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionStatus {
    Queued,
    Pending,
    Running,
    Done,
    Failed,
}

/// Tracking record for one phase in the execution plan.
#[derive(Debug, Clone)]
pub struct ExecutionRecord {
    pub phase_id: String,
    pub agent_id: Option<String>,
    pub status: ExecutionStatus,
    pub started_at: i64,
    pub finished_at: Option<i64>,
    pub error: Option<String>,
}

/// Output of `build_executor_plan`: parallel lists (records[i] ↔ prompts[i]).
pub struct ExecutorPlan {
    pub records: Vec<ExecutionRecord>,
    pub prompts: Vec<PhasePrompt>,
}

// ─────────────────────────── public API ────────────────────────────────────

/// Build a per-phase execution plan without invoking agents.
pub fn build_executor_plan(
    parsed: &ParsedPlan,
    _ledger_db: Option<&Path>,
) -> Result<ExecutorPlan> {
    let now = unix_now();
    let mut records = Vec::with_capacity(parsed.phases.len());
    let mut prompts = Vec::with_capacity(parsed.phases.len());

    for phase in &parsed.phases {
        let prompt = build_phase_prompt(phase);
        records.push(ExecutionRecord {
            phase_id: phase.id.clone(),
            agent_id: None,
            status: ExecutionStatus::Queued,
            started_at: now,
            finished_at: None,
            error: None,
        });
        prompts.push(prompt);
    }

    Ok(ExecutorPlan { records, prompts })
}

/// Pre-register each phase as a 'queued' row in kei-ledger.
/// Idempotent: rows already present (same phase_id) are skipped.
pub fn prereg_phases(plan: &ExecutorPlan, ledger_db: &Path) -> Result<()> {
    let conn = open_ledger(ledger_db)?;
    for (rec, prompt) in plan.records.iter().zip(plan.prompts.iter()) {
        let id = make_agent_id(&rec.phase_id);
        let branch = make_branch(&rec.phase_id);
        let spec_sha = sha_of(&prompt.prompt_text);
        if row_exists(&conn, &id) {
            continue;
        }
        insert_queued_row(&conn, &id, &branch, &spec_sha)
            .with_context(|| format!("inserting ledger row for {}", rec.phase_id))?;
    }
    Ok(())
}

// ─────────────────────────── helpers ───────────────────────────────────────

pub(crate) fn open_ledger(path: &Path) -> Result<rusqlite::Connection> {
    if let Some(p) = path.parent() {
        let _ = std::fs::create_dir_all(p);
    }
    let conn = rusqlite::Connection::open(path)
        .with_context(|| format!("opening ledger at {}", path.display()))?;
    conn.execute_batch(AGENTS_DDL).context("creating agents table")?;
    Ok(conn)
}

const AGENTS_DDL: &str = "CREATE TABLE IF NOT EXISTS agents (
    id TEXT PRIMARY KEY,
    branch TEXT NOT NULL,
    parent_branch TEXT,
    spec_sha TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'queued',
    started_ts INTEGER NOT NULL,
    worktree_path TEXT,
    dna TEXT,
    creator_id TEXT,
    fork_parent_id TEXT,
    finished_ts INTEGER,
    summary TEXT
);";

fn row_exists(conn: &rusqlite::Connection, id: &str) -> bool {
    conn.query_row("SELECT 1 FROM agents WHERE id = ?1", rusqlite::params![id], |_| Ok(()))
        .is_ok()
}

fn insert_queued_row(
    conn: &rusqlite::Connection,
    id: &str,
    branch: &str,
    spec_sha: &str,
) -> rusqlite::Result<()> {
    let now = unix_now();
    conn.execute(
        "INSERT INTO agents (id, branch, spec_sha, status, started_ts)
         VALUES (?1, ?2, ?3, 'queued', ?4)",
        rusqlite::params![id, branch, spec_sha, now],
    )?;
    Ok(())
}

fn make_agent_id(phase_id: &str) -> String {
    format!("import-phase-{}", phase_id.to_lowercase().replace('.', "-"))
}

fn make_branch(phase_id: &str) -> String {
    format!("import/{}", phase_id.to_lowercase().replace('.', "-"))
}

fn sha_of(text: &str) -> String {
    use sha2::{Digest, Sha256};
    let hash = Sha256::digest(text.as_bytes());
    format!("{hash:x}")[..16].to_owned()
}

fn unix_now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}
