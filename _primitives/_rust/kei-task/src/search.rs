//! FTS5 search over tasks (title + description).
//!
//! Thin shim over `kei_entity_store::verbs::search` preserved for
//! callers (integration tests, CLI `cmd_search`) that still want the
//! strongly-typed `Vec<Task>` surface.

use crate::schema::TASK_SCHEMA;
use crate::store::Store;
use crate::types::Task;
use anyhow::{anyhow, Result};
use kei_entity_store::verbs::search as v_search;
use serde_json::{json, Value};

pub fn search(store: &Store, query: &str, limit: i64) -> Result<Vec<Task>> {
    let lim = if limit <= 0 { 20 } else { limit };
    let v = v_search::run(store.conn(), &TASK_SCHEMA, json!({ "query": query, "limit": lim }))
        .map_err(|e| anyhow!("{e}"))?;
    let arr = v["results"].as_array().ok_or_else(|| anyhow!("results missing"))?;
    arr.iter().map(row_to_task).collect()
}

fn row_to_task(r: &Value) -> Result<Task> {
    Ok(Task {
        id: r["id"].as_i64().unwrap_or(0),
        title: r["title"].as_str().unwrap_or("").into(),
        description: r["description"].as_str().unwrap_or("").into(),
        status: r["status"].as_str().unwrap_or("").into(),
        priority: r["priority"].as_str().unwrap_or("").into(),
        task_type: r["task_type"].as_str().unwrap_or("").into(),
        parent_id: r["parent_id"].as_i64().unwrap_or(0),
        assigned_to: r["assigned_to"].as_str().unwrap_or("").into(),
        due_date: r["due_date"].as_i64().unwrap_or(0),
        completed_at: r["completed_at"].as_i64().unwrap_or(0),
        created_at: r["created_at"].as_i64().unwrap_or(0),
        updated_at: r["updated_at"].as_i64().unwrap_or(0),
    })
}
