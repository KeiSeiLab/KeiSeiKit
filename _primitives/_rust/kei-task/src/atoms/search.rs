//! kei-task::search atom — contract in atoms/search.md.
//!
//! Layer-A pilot: input validation stays here, FTS5 JOIN + row
//! assembly run through `kei_entity_store::verbs::search` using
//! `TASK_SCHEMA`.

use crate::schema::TASK_SCHEMA;
use crate::store::Store;
use kei_entity_store::verbs::search as v_search;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fmt;

const DEFAULT_LIMIT: i64 = 20;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    pub query: String,
    #[serde(default)]
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHit {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub task_type: String,
    pub parent_id: i64,
    pub assigned_to: String,
    pub due_date: i64,
    pub completed_at: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    pub results: Vec<SearchHit>,
}

#[derive(Debug)]
pub enum Error {
    InvalidQuery,
    StoreError(anyhow::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidQuery => write!(f, "InvalidQuery: query must be non-empty"),
            Error::StoreError(e) => write!(f, "StoreError: {e:#}"),
        }
    }
}

impl std::error::Error for Error {}

pub fn run(store: &Store, input: Input) -> Result<Output, Error> {
    if input.query.trim().is_empty() {
        return Err(Error::InvalidQuery);
    }
    let limit = normalize_limit(input.limit);
    let payload = json!({ "query": input.query, "limit": limit });
    let v = v_search::run(store.conn(), &TASK_SCHEMA, payload)
        .map_err(|e| Error::StoreError(anyhow::anyhow!("{e}")))?;
    let results = hits_from_value(&v).map_err(Error::StoreError)?;
    Ok(Output { results })
}

fn normalize_limit(raw: Option<i64>) -> i64 {
    match raw {
        Some(n) if n > 0 => n,
        _ => DEFAULT_LIMIT,
    }
}

fn hits_from_value(v: &Value) -> Result<Vec<SearchHit>, anyhow::Error> {
    let arr = v["results"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("search: results not an array"))?;
    arr.iter().map(hit_from_row).collect()
}

fn hit_from_row(row: &Value) -> Result<SearchHit, anyhow::Error> {
    Ok(SearchHit {
        id: row["id"].as_i64().unwrap_or(0),
        title: row["title"].as_str().unwrap_or("").to_string(),
        description: row["description"].as_str().unwrap_or("").to_string(),
        status: row["status"].as_str().unwrap_or("").to_string(),
        priority: row["priority"].as_str().unwrap_or("").to_string(),
        task_type: row["task_type"].as_str().unwrap_or("").to_string(),
        parent_id: row["parent_id"].as_i64().unwrap_or(0),
        assigned_to: row["assigned_to"].as_str().unwrap_or("").to_string(),
        due_date: row["due_date"].as_i64().unwrap_or(0),
        completed_at: row["completed_at"].as_i64().unwrap_or(0),
        created_at: row["created_at"].as_i64().unwrap_or(0),
        updated_at: row["updated_at"].as_i64().unwrap_or(0),
    })
}
