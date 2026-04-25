//! `mark_installed` — flip the `installed` flag from 0 to 1.
//!
//! Metadata-only: does NOT fetch remote content. A future wave will
//! add a real fetch-and-verify path, but v0.30 ships the stub contract
//! so the CLI surface stabilises first.
//!
//! Uses a direct UPDATE rather than `kei_entity_store::verbs::update`
//! to keep the transaction small and return a typed `NotFound` when
//! the id does not exist.

use crate::error::DiscoverError;
use rusqlite::Connection;

const SQL: &str = "UPDATE discover_index SET installed = 1, updated_at = ?2 WHERE id = ?1";

pub fn mark_installed(conn: &Connection, id: i64) -> Result<(), DiscoverError> {
    let now = now_ts();
    let affected = conn.execute(SQL, rusqlite::params![id, now])?;
    if affected == 0 {
        return Err(DiscoverError::NotFound(id));
    }
    Ok(())
}

fn now_ts() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}
