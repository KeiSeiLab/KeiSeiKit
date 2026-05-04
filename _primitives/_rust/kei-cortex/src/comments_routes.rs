//! Sovereign-comment HTTP surface — `/api/v1/cortex/comments/*`.
//!
//! Backed by sibling `kei-comments` primitive. Auth is supplied by the
//! existing cortex Bearer middleware (`routes_auth::require_bearer`)
//! when these handlers are registered into `build_api_router`.
//!
//! Constructor Pattern split:
//!   * `comments_routes.rs` — handlers (this file)
//!   * `comments_routes_init.rs` — store bootstrap + validators
//!
//! `CommentStore` wraps a rusqlite `Connection` which is `!Sync`, so the
//! handle uses `std::sync::Mutex` and every handler defers the SQLite
//! work to `tokio::task::spawn_blocking`.
//!
//! Store access is via the process-wide `global_store()` (lazy `OnceLock`)
//! — NOT via `axum::Extension`. An Extension layer was found to interact
//! badly with the parent's `route_layer(require_bearer)` stack and silently
//! drop the Authorization header on these specific routes; using the
//! global directly avoids that pitfall.

use crate::comments_routes_init::{
    global_store, lock_store, validate_author, validate_body, validate_emoji, validate_page_id,
};
use crate::error::AppError;
use axum::extract::Path;
use axum::Json;
use kei_comments::Comment;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
pub struct PostCommentBody {
    pub author: String,
    pub body: String,
    #[serde(default)]
    pub parent_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteCommentBody {
    pub author: String,
}

#[derive(Debug, Deserialize)]
pub struct ReactBody {
    pub author: String,
    pub emoji: String,
}

#[derive(Debug, Serialize)]
pub struct ListResponse {
    pub comments: Vec<Comment>,
}

pub async fn list_comments(
    Path(page_id): Path<String>,
) -> Result<Json<ListResponse>, AppError> {
    validate_page_id(&page_id)?;
    let comments = tokio::task::spawn_blocking(move || -> Result<Vec<Comment>, AppError> {
        let store = global_store();
        let guard = lock_store(&store)?;
        guard
            .list(&page_id)
            .map_err(|e| AppError::Internal(format!("list comments: {e}")))
    })
    .await
    .map_err(|e| AppError::Internal(format!("join: {e}")))??;
    Ok(Json(ListResponse { comments }))
}

pub async fn post_comment(
    Path(page_id): Path<String>,
    Json(req): Json<PostCommentBody>,
) -> Result<Json<Value>, AppError> {
    validate_page_id(&page_id)?;
    validate_author(&req.author)?;
    validate_body(&req.body)?;
    let comment = tokio::task::spawn_blocking(move || -> Result<Comment, AppError> {
        let store = global_store();
        let guard = lock_store(&store)?;
        let id = guard
            .post(&page_id, &req.author, &req.body, req.parent_id.as_deref())
            .map_err(|e| AppError::BadRequest(format!("post: {e}")))?;
        guard
            .get(&id)
            .map_err(|e| AppError::Internal(format!("readback: {e}")))?
            .ok_or_else(|| AppError::Internal("comment vanished after insert".into()))
    })
    .await
    .map_err(|e| AppError::Internal(format!("join: {e}")))??;
    Ok(Json(comment_to_response(&comment)))
}

pub async fn delete_comment(
    Path(id): Path<String>,
    Json(req): Json<DeleteCommentBody>,
) -> Result<Json<Value>, AppError> {
    validate_author(&req.author)?;
    let ok = tokio::task::spawn_blocking(move || -> Result<bool, AppError> {
        let store = global_store();
        let guard = lock_store(&store)?;
        guard
            .delete(&id, &req.author)
            .map_err(|e| AppError::Internal(format!("delete: {e}")))
    })
    .await
    .map_err(|e| AppError::Internal(format!("join: {e}")))??;
    Ok(Json(json!({ "ok": ok })))
}

pub async fn react_comment(
    Path(id): Path<String>,
    Json(req): Json<ReactBody>,
) -> Result<Json<Value>, AppError> {
    validate_author(&req.author)?;
    validate_emoji(&req.emoji)?;
    tokio::task::spawn_blocking(move || -> Result<(), AppError> {
        let store = global_store();
        let guard = lock_store(&store)?;
        guard
            .react(&id, &req.author, &req.emoji)
            .map_err(|e| AppError::Internal(format!("react: {e}")))
    })
    .await
    .map_err(|e| AppError::Internal(format!("join: {e}")))??;
    Ok(Json(json!({ "ok": true })))
}

fn comment_to_response(c: &Comment) -> Value {
    json!({
        "comment_id": c.id,
        "id": c.id,
        "page_id": c.page_id,
        "author": c.author,
        "body": c.body,
        "parent_id": c.parent_id,
        "created_at": c.created_at,
        "updated_at": c.updated_at,
        "deleted": c.deleted,
    })
}
