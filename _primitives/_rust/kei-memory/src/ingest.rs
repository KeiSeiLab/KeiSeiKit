//! Ingest — read JSONL trace → insert events into DB.
//!
//! Constructor Pattern: one cube, single responsibility.
//! Trace-line shape lives in `trace_line.rs`; classification in
//! `classifier.rs`; tool_use/tool_result extraction in `extract.rs`.
//! This file owns the persistence + IO loop.
//!
//! Schema-mismatch fix: Wave A (2026-05-01). Pre-fix, ~50% of real
//! traces silently dropped via `Err(_) => continue` — root cause was
//! the old struct mapping `kind` to top-level `kind` field, which the
//! real format calls `type`, plus tool calls being nested objects.

pub use crate::trace_line::TraceLine;

use crate::classifier::classify;
use crate::coaccess::record_coaccess;
use crate::error::{KeiMemoryError, Result as KmResult};
use crate::extract::{extract_tool_result, extract_tool_uses, ToolUse};
use crate::injection_guard;
use chrono::Utc;
use rusqlite::{params, Connection, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

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

/// Read a JSONL transcript line by line and insert events.
///
/// Returns total event-row count inserted (one assistant line with N
/// tool_uses → N rows). Malformed JSON yields a stderr log line but
/// does not abort the file. Schema and IO errors propagate.
pub fn ingest_jsonl(conn: &Connection, session_id: &str, path: &Path) -> KmResult<usize> {
    ensure_session(conn, session_id)?;
    let file = File::open(path).map_err(KeiMemoryError::Io)?;
    let mut inserted = 0usize;
    for line in BufReader::new(file).lines().map_while(|l| l.ok()) {
        if let Some(parsed) = parse_one_line(&line) {
            inserted += process_line(conn, session_id, &parsed)?;
        }
    }
    finalize_session(conn, session_id)?;
    Ok(inserted)
}

/// Parse one JSONL line into a TraceLine, surfacing errors to stderr.
/// Returns None for blank / non-object / unparseable lines.
fn parse_one_line(line: &str) -> Option<TraceLine> {
    let trimmed = line.trim();
    if trimmed.is_empty() || !trimmed.starts_with('{') {
        return None;
    }
    match serde_json::from_str::<TraceLine>(trimmed) {
        Ok(p) => Some(p),
        Err(e) => {
            eprintln!("kei-memory: parse skip ({} chars): {e}", trimmed.len());
            None
        }
    }
}

/// Persist all event rows derivable from one parsed trace line.
///
/// Strategy (simpler model — no tool_use ↔ tool_result pairing):
///   * If message has nested `tool_use` blocks: emit one row per block
///     with `tool=name, file_path=input.file_path, is_error=false`.
///   * If message has a `tool_result` block: emit one row with
///     `is_error=<from JSON>` and the legacy `tool` if present.
///   * Otherwise: emit a single row driven by kind + legacy fields.
fn process_line(conn: &Connection, session_id: &str, e: &TraceLine) -> Result<usize> {
    let tool_uses: Vec<ToolUse> = e.message.as_ref().map(extract_tool_uses).unwrap_or_default();
    if !tool_uses.is_empty() {
        for u in &tool_uses {
            let fp = u.file_path.clone().or_else(|| e.file_path.clone());
            insert_one(conn, session_id, e, Some(&u.name), fp.as_deref(), false)?;
        }
        return Ok(tool_uses.len());
    }
    let is_err = e
        .message
        .as_ref()
        .and_then(extract_tool_result)
        .map(|r| r.is_error)
        .or(e.is_error)
        .unwrap_or(false);
    insert_one(conn, session_id, e, e.tool.as_deref(), e.file_path.as_deref(), is_err)?;
    Ok(1)
}

/// Insert a single event row directly (legacy entrypoint kept for tests).
///
/// P2.1.b — guards `message_text()` via `injection_guard::scan` BEFORE
/// persistence. A Block-tier hit logs to stderr and skips the row
/// (returns Ok so the surrounding ingest loop continues). This is a
/// real memory-write path: the message later flows into the system
/// prompt verbatim.
pub fn insert_event(conn: &Connection, session_id: &str, e: &TraceLine) -> Result<()> {
    insert_one(
        conn,
        session_id,
        e,
        e.tool.as_deref(),
        e.file_path.as_deref(),
        e.is_error.unwrap_or(false),
    )
}

/// Single insert path used by `process_line` AND `insert_event`.
/// Applies guard, classifier, persists row, records co-access.
fn insert_one(
    conn: &Connection,
    session_id: &str,
    e: &TraceLine,
    tool: Option<&str>,
    file_path: Option<&str>,
    is_err: bool,
) -> Result<()> {
    let msg_text = e.message_text();
    if message_is_blocked(session_id, msg_text.as_deref()) {
        return Ok(());
    }
    let ts = e.resolved_ts();
    let kind = e.kind.as_deref().unwrap_or("other");
    let class = e
        .event_class
        .clone()
        .unwrap_or_else(|| classify(Some(kind), tool, msg_text.as_deref(), is_err));
    conn.execute(
        "INSERT INTO events (session_id, ts, kind, tool, file_path, is_error, event_class, message, cwd)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![session_id, ts, kind, tool, file_path, is_err as i64, class, msg_text, e.cwd],
    )?;
    if let Some(fp) = file_path {
        record_coaccess(conn, session_id, fp, ts)?;
    }
    Ok(())
}

/// Returns true (and logs) when `message` carries a Block-tier finding.
fn message_is_blocked(session_id: &str, message: Option<&str>) -> bool {
    if let Some(msg) = message {
        if let Err(finding) = injection_guard::scan(msg) {
            eprintln!("kei-memory: insert_event rejected (session={session_id}): {finding}");
            return true;
        }
    }
    false
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
