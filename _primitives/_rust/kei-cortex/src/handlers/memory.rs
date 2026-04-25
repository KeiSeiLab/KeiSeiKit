//! `GET /api/v1/cortex/memory/search` — substring scan over pet memory.
//!
//! Delegates to `kei_pet::memory::search` which implements a LIKE-scoped
//! query keyed by `(user_id, pet_name)`.

use crate::error::AppError;
use crate::state::AppState;
use crate::validate;
use axum::extract::{Query, State};
use axum::Json;
use kei_pet::memory::{ensure_schema, search, MemoryTag};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

/// Maximum allowed `limit`.
pub const MAX_LIMIT: usize = 200;

/// Default `limit` when absent.
pub const DEFAULT_LIMIT: usize = 20;

#[derive(Debug, Deserialize)]
pub struct MemoryQuery {
    pub user_id: String,
    pub pet_name: String,
    pub q: String,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct MemoryHit {
    pub id: i64,
    pub role: String,
    pub text: String,
    pub ts: i64,
}

#[derive(Debug, Serialize)]
pub struct MemoryResponse {
    pub hits: Vec<MemoryHit>,
}

/// Handler entry point.
pub async fn search_memory(
    State(state): State<AppState>,
    Query(q): Query<MemoryQuery>,
) -> Result<Json<MemoryResponse>, AppError> {
    validate_query(&q)?;
    let limit = clamp_limit(q.limit.unwrap_or(DEFAULT_LIMIT));
    let db_path = state.config().memory_db.clone();
    let hits = tokio::task::spawn_blocking(move || run_search(&db_path, &q, limit))
        .await
        .map_err(|e| AppError::Internal(format!("memory task join: {e}")))??;
    Ok(Json(MemoryResponse { hits }))
}

fn validate_query(q: &MemoryQuery) -> Result<(), AppError> {
    validate::user_id(&q.user_id)?;
    if q.pet_name.is_empty() {
        return Err(AppError::BadRequest("pet_name is empty".into()));
    }
    if q.q.is_empty() {
        return Err(AppError::BadRequest("q is empty".into()));
    }
    Ok(())
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

fn run_search(
    db_path: &std::path::Path,
    q: &MemoryQuery,
    limit: usize,
) -> Result<Vec<MemoryHit>, AppError> {
    if !db_path.exists() {
        return Ok(Vec::new());
    }
    let conn = Connection::open(db_path)?;
    ensure_schema(&conn).map_err(|e| AppError::Internal(format!("memory schema: {e}")))?;
    let tag = MemoryTag {
        user_id: q.user_id.clone(),
        pet_name: q.pet_name.clone(),
    };
    let rows = search(&conn, &tag, &q.q, limit)
        .map_err(|e| AppError::Internal(format!("memory search: {e}")))?;
    Ok(rows.into_iter().map(to_hit).collect())
}

fn to_hit(i: kei_pet::memory::Interaction) -> MemoryHit {
    MemoryHit {
        id: i.id,
        role: i.role,
        text: i.text,
        ts: i.ts,
    }
}
