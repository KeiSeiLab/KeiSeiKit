//! `GET /api/v1/cortex/summary` — aggregate counters over ledger + pets.
//!
//! The endpoint is intentionally cheap: a couple of indexed COUNTs + a
//! directory scan. It exists so the UI can render a landing page without
//! hitting four separate endpoints.

use crate::error::AppError;
use crate::state::AppState;
use axum::extract::State;
use axum::Json;
use rusqlite::Connection;
use serde::Serialize;
use std::fs;

/// JSON body returned by `/summary`.
#[derive(Debug, Serialize)]
pub struct SummaryResponse {
    pub total_dnas: i64,
    pub active_pets: Vec<String>,
    pub ledger_last_ts: Option<i64>,
    pub recent_sessions: i64,
}

/// Handler entry point.
pub async fn summary(State(state): State<AppState>) -> Result<Json<SummaryResponse>, AppError> {
    let cfg = state.config().clone();
    let body = tokio::task::spawn_blocking(move || build_summary(&cfg))
        .await
        .map_err(|e| AppError::Internal(format!("summary task join: {e}")))??;
    Ok(Json(body))
}

/// Blocking helper: opens the ledger DB, runs 3 queries, lists the pet dir.
fn build_summary(cfg: &crate::AppConfig) -> Result<SummaryResponse, AppError> {
    let total_dnas = count_ledger(&cfg.ledger_path, "SELECT COUNT(*) FROM agents")?;
    let ledger_last_ts = last_ledger_ts(&cfg.ledger_path)?;
    let recent_sessions = count_ledger(
        &cfg.ledger_path,
        "SELECT COUNT(*) FROM agents WHERE started_ts >= strftime('%s','now','-1 day')",
    )?;
    let active_pets = list_pet_user_ids(&cfg.pet_root)?;
    Ok(SummaryResponse {
        total_dnas,
        active_pets,
        ledger_last_ts,
        recent_sessions,
    })
}

/// Run a single scalar COUNT query against the ledger DB if present. Missing
/// file or missing `agents` table yield `0` so a first-boot daemon still
/// serves a useful response.
fn count_ledger(path: &std::path::Path, sql: &str) -> Result<i64, AppError> {
    if !path.exists() {
        return Ok(0);
    }
    let conn = Connection::open(path)?;
    if !has_agents_table(&conn)? {
        return Ok(0);
    }
    let count: i64 = conn.query_row(sql, [], |r| r.get(0)).unwrap_or(0);
    Ok(count)
}

/// Return max(started_ts) from the agents table, or `None` if table is empty.
fn last_ledger_ts(path: &std::path::Path) -> Result<Option<i64>, AppError> {
    if !path.exists() {
        return Ok(None);
    }
    let conn = Connection::open(path)?;
    if !has_agents_table(&conn)? {
        return Ok(None);
    }
    let ts: Option<i64> = conn
        .query_row("SELECT MAX(started_ts) FROM agents", [], |r| r.get(0))
        .unwrap_or(None);
    Ok(ts)
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

fn list_pet_user_ids(root: &std::path::Path) -> Result<Vec<String>, AppError> {
    if !root.exists() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                out.push(stem.to_string());
            }
        }
    }
    out.sort();
    Ok(out)
}
