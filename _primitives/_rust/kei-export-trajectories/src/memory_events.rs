//! Tool-event extraction from `kei-memory.sqlite`.
//!
//! Constructor Pattern: SQL-only cube. The ledger_reader hands us a
//! connection + an agent_id + a time window; we hand back a
//! `Vec<ToolEvent>`. Two queries: session-keyed first (cheap, indexed),
//! ts-windowed fallback for sessions that pre-date the agent_id ↔
//! session_id linkage.

use crate::tool_stats::ToolEvent;
use anyhow::{Context, Result};
use rusqlite::{params, Connection, OptionalExtension};

/// Pull tool events from `kei-memory.events` matching this agent. We try
/// `session_id == agent_id` first; if no rows exist, fall back to any
/// events whose `ts` falls in `[started_ts, finished_ts]`. Both are
/// best-effort — a trajectory with no events is valid (returns empty).
pub fn query_tool_events(
    conn: &Connection,
    agent_id: &str,
    started_ts: i64,
    finished_ts: Option<i64>,
) -> Result<Vec<ToolEvent>> {
    if let Some(direct) = query_events_by_session(conn, agent_id)? {
        if !direct.is_empty() {
            return Ok(direct);
        }
    }
    let end_ts = finished_ts.unwrap_or(i64::MAX);
    query_events_by_ts_window(conn, started_ts, end_ts)
}

fn query_events_by_session(conn: &Connection, sid: &str) -> Result<Option<Vec<ToolEvent>>> {
    let exists: Option<i64> = conn
        .query_row(
            "SELECT 1 FROM events WHERE session_id = ?1 LIMIT 1",
            params![sid],
            |r| r.get(0),
        )
        .optional()
        .context("probe events table")?;
    if exists.is_none() {
        return Ok(None);
    }
    let mut stmt = conn
        .prepare(
            "SELECT tool, is_error FROM events
             WHERE session_id = ?1 AND tool IS NOT NULL ORDER BY ts ASC",
        )
        .context("prepare events-by-session")?;
    let rows = stmt
        .query_map(params![sid], map_tool_event)
        .context("query events-by-session")?;
    let collected: rusqlite::Result<Vec<ToolEvent>> = rows.collect();
    Ok(Some(collected.context("collect events-by-session")?))
}

fn query_events_by_ts_window(conn: &Connection, start: i64, end: i64) -> Result<Vec<ToolEvent>> {
    let mut stmt = conn
        .prepare(
            "SELECT tool, is_error FROM events
             WHERE ts >= ?1 AND ts <= ?2 AND tool IS NOT NULL ORDER BY ts ASC",
        )
        .context("prepare events-by-window")?;
    let rows = stmt
        .query_map(params![start, end], map_tool_event)
        .context("query events-by-window")?;
    let collected: rusqlite::Result<Vec<ToolEvent>> = rows.collect();
    collected.context("collect events-by-window")
}

fn map_tool_event(r: &rusqlite::Row) -> rusqlite::Result<ToolEvent> {
    let tool: String = r.get(0)?;
    let is_error: i64 = r.get(1)?;
    Ok(ToolEvent {
        tool,
        success: is_error == 0,
    })
}
