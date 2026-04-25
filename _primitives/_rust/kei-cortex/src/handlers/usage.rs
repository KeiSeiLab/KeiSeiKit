//! `GET /api/v1/cortex/usage` — cost rollup over the kei-ledger SQLite.
//!
//! Read-only aggregation over today / week / month time windows plus
//! the top (provider, model) pair by total cost. The handler returns
//! 404 if the ledger file is missing or its `agents` table lacks a
//! `cost_cents` column — schema migration is a separate concern.
//!
//! F-MED-3 fix: today / week / month boundaries are CALENDAR DAYS in the
//! local timezone (Mon-anchored ISO weeks, 1st-of-month). The pre-fix
//! sliding-window rollup (`now - 24*3600`, etc.) drifted across midnight
//! and contradicted the UI labels. See `usage_calendar.rs` for the
//! boundary helpers.

use crate::error::AppError;
use crate::state::AppState;
use axum::extract::State;
use axum::Json;
use rusqlite::{Connection, OptionalExtension};
use serde::Serialize;
use std::path::Path;

mod calendar;
use calendar::CalendarBoundaries;

/// JSON body returned to the UI. Cents are unsigned i64 — the handler
/// clamps any unexpected NULL or negative aggregate to 0 before send.
#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct UsageReport {
    pub today_cents: i64,
    pub week_cents: i64,
    pub month_cents: i64,
    pub top_provider: String,
    pub top_model: String,
}

/// Handler entry point. Off-loads the SQLite read to a blocking task
/// so the async runtime stays responsive on slow disks.
pub async fn usage(State(state): State<AppState>) -> Result<Json<UsageReport>, AppError> {
    let cfg = state.config().clone();
    let report = tokio::task::spawn_blocking(move || load_usage(&cfg.ledger_path))
        .await
        .map_err(|e| AppError::Internal(format!("usage task join: {e}")))??;
    match report {
        Some(r) => Ok(Json(r)),
        None => Err(AppError::NotFound("ledger usage unavailable".into())),
    }
}

/// Top-level loader. Returns `None` (→ 404) when the ledger file is
/// missing, the `agents` table is absent, or any of the cost-tracking
/// columns (`cost_cents`, `provider`, `model`) has not yet been added.
fn load_usage(path: &Path) -> Result<Option<UsageReport>, AppError> {
    if !path.exists() {
        return Ok(None);
    }
    let conn = Connection::open(path)?;
    if !has_cost_columns(&conn)? {
        return Ok(None);
    }
    let bounds = CalendarBoundaries::for_now_local();
    let totals = sum_windows(&conn, &bounds)?;
    let top = top_provider_model(&conn)?;
    Ok(Some(UsageReport {
        today_cents: totals.0,
        week_cents: totals.1,
        month_cents: totals.2,
        top_provider: top.0,
        top_model: top.1,
    }))
}

/// Probe `pragma table_info(agents)` for the three cost-tracking columns
/// (`cost_cents`, `provider`, `model`). Returns false if the table is
/// absent or any column missing — partial migrations route to 404.
fn has_cost_columns(conn: &Connection) -> Result<bool, AppError> {
    let mut stmt = conn.prepare("PRAGMA table_info(agents)")?;
    let mut rows = stmt.query([])?;
    let mut seen_cost = false;
    let mut seen_provider = false;
    let mut seen_model = false;
    while let Some(row) = rows.next()? {
        let name: String = row.get(1)?;
        match name.as_str() {
            "cost_cents" => seen_cost = true,
            "provider" => seen_provider = true,
            "model" => seen_model = true,
            _ => {}
        }
    }
    Ok(seen_cost && seen_provider && seen_model)
}

/// Sum `cost_cents` since each calendar boundary (today / Monday / 1st of
/// the month, all in local time). `started_ts` is the unix-second anchor.
fn sum_windows(
    conn: &Connection,
    bounds: &CalendarBoundaries,
) -> Result<(i64, i64, i64), AppError> {
    let today = sum_since(conn, bounds.today_start_ts)?;
    let week = sum_since(conn, bounds.week_start_ts)?;
    let month = sum_since(conn, bounds.month_start_ts)?;
    Ok((today, week, month))
}

fn sum_since(conn: &Connection, since_ts: i64) -> Result<i64, AppError> {
    let total: Option<i64> = conn
        .query_row(
            "SELECT COALESCE(SUM(cost_cents), 0) FROM agents WHERE started_ts >= ?1",
            [since_ts],
            |r| r.get(0),
        )
        .optional()?;
    Ok(total.unwrap_or(0).max(0))
}

/// Top (provider, model) pair by total `cost_cents` across all rows.
/// Returns empty strings if the table is empty (no row matches).
fn top_provider_model(conn: &Connection) -> Result<(String, String), AppError> {
    let row: Option<(String, String)> = conn
        .query_row(
            "SELECT COALESCE(provider, '') AS p, COALESCE(model, '') AS m
             FROM agents
             WHERE cost_cents > 0
             GROUP BY p, m
             ORDER BY SUM(cost_cents) DESC
             LIMIT 1",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .optional()?;
    Ok(row.unwrap_or_default())
}

#[cfg(test)]
#[path = "usage_test.rs"]
mod tests;
