//! `GET /api/v1/cortex/ledger/recent?limit=N` — most-recent agent rows.
//!
//! Reads the kei-ledger SQLite database directly. The daemon only needs the
//! columns the UI renders, so we project a compact `LedgerRow` rather than
//! the full kei-ledger struct.

use crate::error::AppError;
use crate::state::AppState;
use axum::extract::{Query, State};
use axum::Json;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

/// Hard upper bound on `limit` to keep responses small.
pub const MAX_LIMIT: usize = 200;

/// Default limit when the query string is omitted.
pub const DEFAULT_LIMIT: usize = 20;

#[derive(Debug, Deserialize)]
pub struct LedgerQuery {
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct LedgerRow {
    pub id: String,
    pub branch: String,
    pub parent_branch: Option<String>,
    pub status: String,
    pub started_ts: i64,
    pub finished_ts: Option<i64>,
    pub summary: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LedgerResponse {
    pub rows: Vec<LedgerRow>,
}

/// Handler entry point.
pub async fn recent(
    State(state): State<AppState>,
    Query(q): Query<LedgerQuery>,
) -> Result<Json<LedgerResponse>, AppError> {
    let limit = clamp_limit(q.limit.unwrap_or(DEFAULT_LIMIT));
    let cfg = state.config().clone();
    let rows = tokio::task::spawn_blocking(move || load_recent(&cfg.ledger_path, limit))
        .await
        .map_err(|e| AppError::Internal(format!("ledger task join: {e}")))??;
    Ok(Json(LedgerResponse { rows }))
}

fn clamp_limit(requested: usize) -> usize {
    if requested == 0 {
        DEFAULT_LIMIT
    } else if requested > MAX_LIMIT {
        MAX_LIMIT
    } else {
        requested
    }
}

fn load_recent(path: &std::path::Path, limit: usize) -> Result<Vec<LedgerRow>, AppError> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let conn = Connection::open(path)?;
    if !has_agents_table(&conn)? {
        return Ok(Vec::new());
    }
    query_rows(&conn, limit)
}

fn has_agents_table(conn: &Connection) -> Result<bool, AppError> {
    let exists: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='agents'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    Ok(exists > 0)
}

fn query_rows(conn: &Connection, limit: usize) -> Result<Vec<LedgerRow>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, branch, parent_branch, status, started_ts, finished_ts, summary
         FROM agents
         ORDER BY started_ts DESC, id DESC
         LIMIT ?1",
    )?;
    let iter = stmt.query_map([limit as i64], row_to_ledger)?;
    let mut out = Vec::with_capacity(limit);
    for row in iter {
        out.push(row?);
    }
    Ok(out)
}

fn row_to_ledger(row: &rusqlite::Row<'_>) -> rusqlite::Result<LedgerRow> {
    Ok(LedgerRow {
        id: row.get(0)?,
        branch: row.get(1)?,
        parent_branch: row.get(2)?,
        status: row.get(3)?,
        started_ts: row.get(4)?,
        finished_ts: row.get(5)?,
        summary: row.get(6)?,
    })
}
