//! Pet endpoints — read a persona manifest + record an interaction.
//!
//! - `GET  /api/v1/cortex/pet/:user_id`
//! - `POST /api/v1/cortex/pet/:user_id/interaction`
//!
//! The manifest lives on disk at `<pet_root>/<user_id>.toml`. Interactions
//! are written to the kei-pet SQLite memory store.

use crate::error::AppError;
use crate::state::AppState;
use crate::validate;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use kei_pet::memory::{ensure_schema, record_interaction, MemoryTag};
use kei_pet::PetManifest;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::fs;

/// Response body for `GET /pet/:user_id`.
#[derive(Debug, Serialize)]
pub struct PetGetResponse {
    pub pet: PetManifest,
}

/// Request body for `POST /pet/:user_id/interaction`.
#[derive(Debug, Deserialize)]
pub struct InteractionRequest {
    pub role: String,
    pub text: String,
    pub ts: i64,
}

/// Response body for `POST /pet/:user_id/interaction`.
#[derive(Debug, Serialize)]
pub struct InteractionResponse {
    pub interaction_id: i64,
}

/// Handler — load `<pet_root>/<user_id>.toml` into a `PetManifest`.
pub async fn get_pet(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<Json<PetGetResponse>, AppError> {
    validate::user_id(&user_id)?;
    let path = state.config().pet_root.join(format!("{user_id}.toml"));
    if !path.exists() {
        return Err(AppError::NotFound(format!("pet {user_id}")));
    }
    let text = fs::read_to_string(&path)?;
    let pet = kei_pet::parse(&text)
        .map_err(|e| AppError::BadRequest(format!("parse pet.toml: {e}")))?;
    Ok(Json(PetGetResponse { pet }))
}

/// Handler — append a single interaction row to the kei-pet memory DB.
pub async fn post_interaction(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    Json(req): Json<InteractionRequest>,
) -> Result<(StatusCode, Json<InteractionResponse>), AppError> {
    validate::user_id(&user_id)?;
    validate_interaction(&req)?;
    let pet_name = pet_name_for(&state, &user_id).await?;
    let cfg = state.config().clone();
    let id = tokio::task::spawn_blocking(move || {
        write_interaction(&cfg.memory_db, &user_id, &pet_name, &req)
    })
    .await
    .map_err(|e| AppError::Internal(format!("interaction task join: {e}")))??;
    Ok((StatusCode::CREATED, Json(InteractionResponse { interaction_id: id })))
}

fn validate_interaction(req: &InteractionRequest) -> Result<(), AppError> {
    if req.role.is_empty() {
        return Err(AppError::BadRequest("role is empty".into()));
    }
    if req.text.is_empty() {
        return Err(AppError::BadRequest("text is empty".into()));
    }
    if req.ts <= 0 {
        return Err(AppError::BadRequest("ts must be positive".into()));
    }
    Ok(())
}

async fn pet_name_for(state: &AppState, user_id: &str) -> Result<String, AppError> {
    let path = state.config().pet_root.join(format!("{user_id}.toml"));
    if !path.exists() {
        return Err(AppError::NotFound(format!("pet {user_id}")));
    }
    let text = fs::read_to_string(&path)?;
    let pet = kei_pet::parse(&text)
        .map_err(|e| AppError::BadRequest(format!("parse pet.toml: {e}")))?;
    Ok(pet.identity.pet_name)
}

fn write_interaction(
    db_path: &std::path::Path,
    user_id: &str,
    pet_name: &str,
    req: &InteractionRequest,
) -> Result<i64, AppError> {
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let conn = Connection::open(db_path)?;
    ensure_schema(&conn).map_err(|e| AppError::Internal(format!("memory schema: {e}")))?;
    let tag = MemoryTag {
        user_id: user_id.to_string(),
        pet_name: pet_name.to_string(),
    };
    let id = record_interaction(&conn, &tag, &req.role, &req.text, req.ts)
        .map_err(|e| AppError::Internal(format!("record interaction: {e}")))?;
    Ok(id)
}
