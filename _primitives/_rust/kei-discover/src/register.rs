//! `register` — insert one primitive announcement.
//!
//! Dispatches to `kei_entity_store::verbs::create` with a JSON
//! payload assembled from the typed arguments. `last_seen_ts` is stamped
//! to the current Unix timestamp; `installed` defaults to 0.
//!
//! Duplicate-slug detection: the schema emits a UNIQUE INDEX on `slug`
//! (see `schema.rs`). A duplicate INSERT surfaces as a SQLite constraint
//! error which we map back to `DiscoverError::DuplicateSlug(slug)` so
//! callers get a typed signal with the offending slug included.

use crate::error::DiscoverError;
use crate::schema::DISCOVER_SCHEMA;
use kei_entity_store::verbs::create as v_create;
use rusqlite::Connection;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn register(
    conn: &Connection,
    slug: &str,
    author: &str,
    url: &str,
    description: &str,
) -> Result<i64, DiscoverError> {
    validate(slug, author)?;
    let now = now_ts();
    let input = json!({
        "slug": slug,
        "author": author,
        "source_url": url,
        "description": description,
        "installed": 0,
        "last_seen_ts": now,
    });
    match v_create::run(conn, &DISCOVER_SCHEMA, input) {
        Ok(v) => v
            .get("id")
            .and_then(|x| x.as_i64())
            .ok_or_else(|| DiscoverError::Storage("register: missing id in response".into())),
        Err(e) => Err(classify(e, slug)),
    }
}

fn validate(slug: &str, author: &str) -> Result<(), DiscoverError> {
    if slug.trim().is_empty() {
        return Err(DiscoverError::InvalidInput("register: slug must be non-empty".into()));
    }
    if author.trim().is_empty() {
        return Err(DiscoverError::InvalidInput("register: author must be non-empty".into()));
    }
    Ok(())
}

/// Map a `VerbError` to `DiscoverError`, re-attaching the offending slug
/// when the failure is a UNIQUE-constraint violation on `slug`.
fn classify(e: kei_entity_store::VerbError, slug: &str) -> DiscoverError {
    let msg = format!("{e}");
    if msg.contains("UNIQUE constraint failed") && msg.contains("slug") {
        return DiscoverError::DuplicateSlug(slug.to_string());
    }
    DiscoverError::from(e)
}

fn now_ts() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}
