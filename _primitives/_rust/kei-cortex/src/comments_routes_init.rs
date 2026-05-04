//! Init / store-handle / validators for `comments_routes`.
//!
//! Constructor Pattern: `comments_routes.rs` is the router + handlers,
//! this file owns the side-effecting bootstrap (open DB, migrate) and
//! the pure input validators. Splitting keeps each file <200 LOC.

use crate::error::AppError;
use kei_comments::{CommentStore, MAX_BODY_BYTES};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};

/// Shared, thread-safe handle injected via Extension into every handler.
pub type SharedStore = Arc<Mutex<CommentStore>>;

pub const MAX_AUTHOR_LEN: usize = 64;
pub const MAX_EMOJI_LEN: usize = 16;
pub const MAX_PAGE_ID_LEN: usize = 256;

/// Process-wide store handle. Opened lazily; on first-use failure we
/// fall back to an in-memory store so the daemon keeps serving.
pub fn global_store() -> SharedStore {
    static SLOT: OnceLock<SharedStore> = OnceLock::new();
    SLOT.get_or_init(|| Arc::new(Mutex::new(open_or_memory()))).clone()
}

fn open_or_memory() -> CommentStore {
    let path = comments_db_path();
    match CommentStore::open(&path).and_then(|s| {
        s.migrate()?;
        Ok(s)
    }) {
        Ok(s) => s,
        Err(e) => {
            eprintln!(
                "kei-cortex: comments DB open failed at {path:?}: {e}; using in-memory"
            );
            let mem = CommentStore::open_memory().expect("in-memory sqlite");
            let _ = mem.migrate();
            mem
        }
    }
}

fn comments_db_path() -> PathBuf {
    if let Ok(v) = std::env::var("KEI_COMMENTS_DB") {
        return PathBuf::from(v);
    }
    if let Ok(home) = std::env::var("HOME") {
        return PathBuf::from(home).join(".keisei").join("comments.sqlite");
    }
    PathBuf::from(".keisei/comments.sqlite")
}

pub fn validate_page_id(s: &str) -> Result<(), AppError> {
    if s.is_empty() || s.len() > MAX_PAGE_ID_LEN {
        return Err(AppError::BadRequest("invalid page_id".into()));
    }
    Ok(())
}

pub fn validate_author(s: &str) -> Result<(), AppError> {
    if s.is_empty() || s.len() > MAX_AUTHOR_LEN {
        return Err(AppError::BadRequest("invalid author".into()));
    }
    Ok(())
}

pub fn validate_emoji(s: &str) -> Result<(), AppError> {
    if s.is_empty() || s.len() > MAX_EMOJI_LEN {
        return Err(AppError::BadRequest("invalid emoji".into()));
    }
    Ok(())
}

pub fn validate_body(s: &str) -> Result<(), AppError> {
    if s.is_empty() {
        return Err(AppError::BadRequest("body must not be empty".into()));
    }
    if s.len() > MAX_BODY_BYTES {
        return Err(AppError::PayloadTooLarge(format!(
            "body length {} exceeds {}",
            s.len(),
            MAX_BODY_BYTES
        )));
    }
    Ok(())
}

pub fn lock_store(
    store: &SharedStore,
) -> Result<std::sync::MutexGuard<'_, CommentStore>, AppError> {
    store
        .lock()
        .map_err(|e| AppError::Internal(format!("comments lock poisoned: {e}")))
}
