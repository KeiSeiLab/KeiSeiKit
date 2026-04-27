//! Ingest — read JSONL trace → insert events into DB.
//!
//! Constructor Pattern: one cube, single responsibility.
//! Trace line shape (subset we care about):
//!   {"ts": 1700000000, "kind": "tool_use", "tool": "Bash",
//!    "file_path": "...", "is_error": false, "message": "..."}
//! Unknown/empty lines are skipped silently.

use crate::coaccess::record_coaccess;
use chrono::Utc;
use rusqlite::{params, Connection, Result};
use serde::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Deserialize, Default)]
pub struct TraceLine {
    #[serde(default)]
    pub ts: Option<i64>,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub tool: Option<String>,
    #[serde(default)]
    pub file_path: Option<String>,
    #[serde(default)]
    pub is_error: Option<bool>,
    #[serde(default)]
    pub event_class: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
}

/// Ensure the sessions row exists (idempotent). Returns started_ts.
pub fn ensure_session(conn: &Connection, session_id: &str) -> Result<i64> {
    let now = Utc::now().timestamp();
    conn.execute(
        "INSERT OR IGNORE INTO sessions (id, started_ts) VALUES (?1, ?2)",
        params![session_id, now],
    )?;
    let started: i64 = conn.query_row(
        "SELECT started_ts FROM sessions WHERE id = ?1",
        params![session_id],
        |r| r.get(0),
    )?;
    Ok(started)
}

/// Read a JSONL transcript line by line and insert one row per event.
/// Returns the number of events actually inserted (malformed lines skipped).
pub fn ingest_jsonl(conn: &Connection, session_id: &str, path: &Path) -> Result<usize> {
    ensure_session(conn, session_id)?;
    let file = File::open(path)
        .map_err(|e| rusqlite::Error::InvalidParameterName(format!("open {}: {e}", path.display())))?;
    let reader = BufReader::new(file);
    let mut inserted = 0usize;
    for line in reader.lines().map_while(|l| l.ok()) {
        let trimmed = line.trim();
        if trimmed.is_empty() || !trimmed.starts_with('{') {
            continue;
        }
        let parsed: TraceLine = match serde_json::from_str(trimmed) {
            Ok(p) => p,
            Err(_) => continue,
        };
        insert_event(conn, session_id, &parsed)?;
        inserted += 1;
    }
    finalize_session(conn, session_id)?;
    Ok(inserted)
}

/// Insert a single event row. Updates co-access if file_path present.
pub fn insert_event(conn: &Connection, session_id: &str, e: &TraceLine) -> Result<()> {
    let ts = e.ts.unwrap_or_else(|| Utc::now().timestamp());
    let kind = e.kind.clone().unwrap_or_else(|| "other".to_string());
    let is_err = e.is_error.unwrap_or(false) as i64;
    let class = e
        .event_class
        .clone()
        .unwrap_or_else(|| classify_default(&kind, e.tool.as_deref(), e.message.as_deref()));
    conn.execute(
        "INSERT INTO events (session_id, ts, kind, tool, file_path, is_error, event_class, message)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            session_id,
            ts,
            kind,
            e.tool,
            e.file_path,
            is_err,
            class,
            e.message
        ],
    )?;
    if let Some(fp) = &e.file_path {
        record_coaccess(conn, session_id, fp, ts)?;
    }
    Ok(())
}

/// Cheap heuristic classifier used when trace does not provide one.
fn classify_default(kind: &str, tool: Option<&str>, message: Option<&str>) -> String {
    if let Some(m) = message {
        let lm = m.to_lowercase();
        if lm.contains("permission denied") || lm.contains("denied") {
            return "permission_denied".to_string();
        }
        if lm.contains("worktree") && lm.contains("error") {
            return "worktree_error".to_string();
        }
        if lm.contains("cargo") && lm.contains("workspace") {
            return "cargo_workspace".to_string();
        }
    }
    match (kind, tool) {
        ("tool_use", Some(t)) => format!("tool_use:{t}"),
        _ => kind.to_string(),
    }
}

/// Update aggregate counters on the sessions row.
pub fn finalize_session(conn: &Connection, session_id: &str) -> Result<()> {
    let now = Utc::now().timestamp();
    conn.execute(
        "UPDATE sessions SET
            ended_ts = ?1,
            tool_call_count = (SELECT COUNT(*) FROM events WHERE session_id = ?2),
            error_count = (SELECT COALESCE(SUM(is_error),0) FROM events WHERE session_id = ?2)
         WHERE id = ?2",
        params![now, session_id],
    )?;
    Ok(())
}
