//! FTS over messages.
//!
//! Layer-A convergence (2026-04-23): delegates to
//! `kei_entity_store::verbs::search` using `MESSAGES_SCHEMA`. The engine
//! handles FTS5 JOIN + rank ordering; this module maps the generic
//! JSON result back to typed `ChatMessage` rows for legacy callers.
//! Per-message `cost` is persisted (engine `RealDefault` field);
//! `row_to_message` reads it back as f64.

use crate::schema::MESSAGES_SCHEMA;
use crate::sessions::ChatMessage;
use crate::store::Store;
use anyhow::{anyhow, Result};
use kei_entity_store::verbs::search as v_search;
use serde_json::{json, Value};

pub fn search(store: &Store, query: &str, limit: i64) -> Result<Vec<ChatMessage>> {
    let lim = if limit <= 0 { 20 } else { limit };
    let v = v_search::run(store.conn(), &MESSAGES_SCHEMA, json!({ "query": query, "limit": lim }))
        .map_err(|e| anyhow!("{e}"))?;
    let arr = v["results"]
        .as_array()
        .ok_or_else(|| anyhow!("search: results missing"))?;
    arr.iter().map(row_to_message).collect()
}

fn row_to_message(r: &Value) -> Result<ChatMessage> {
    Ok(ChatMessage {
        id: r["id"].as_i64().unwrap_or(0),
        session_id: r["session_id"].as_str().unwrap_or("").into(),
        role: r["role"].as_str().unwrap_or("").into(),
        content: r["content"].as_str().unwrap_or("").into(),
        tokens_in: r["tokens_in"].as_i64().unwrap_or(0),
        tokens_out: r["tokens_out"].as_i64().unwrap_or(0),
        cost: r["cost"].as_f64().unwrap_or(0.0),
        created_at: r["created_at"].as_i64().unwrap_or(0),
    })
}
