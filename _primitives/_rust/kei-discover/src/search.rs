//! `search` — FTS5 match over `slug` + `description`.
//!
//! Thin wrapper around `kei_entity_store::verbs::search` that decodes
//! the JSON `results` array into typed `Entry` values. An empty query
//! is rejected with `InvalidInput` before dispatch (the engine enforces
//! the same rule but we surface the typed variant eagerly).

use crate::entry::Entry;
use crate::error::DiscoverError;
use crate::schema::DISCOVER_SCHEMA;
use kei_entity_store::verbs::search as v_search;
use rusqlite::Connection;
use serde_json::json;

pub fn search(conn: &Connection, query: &str) -> Result<Vec<Entry>, DiscoverError> {
    if query.trim().is_empty() {
        return Err(DiscoverError::InvalidInput("search: query must be non-empty".into()));
    }
    let v = v_search::run(conn, &DISCOVER_SCHEMA, json!({ "query": query }))?;
    let rows = v
        .get("results")
        .and_then(|x| x.as_array())
        .ok_or_else(|| DiscoverError::Storage("search: missing results array".into()))?;
    Ok(rows.iter().map(Entry::from_json).collect())
}
