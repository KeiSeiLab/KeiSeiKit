//! kei-task::create atom — contract in atoms/create.md.
//!
//! Layer-A pilot: validates task-specific input (title non-empty,
//! priority enum) then delegates the INSERT + FTS reindex to
//! `kei_entity_store::verbs::create` through the crate-level
//! `TASK_SCHEMA`.

use crate::schema::TASK_SCHEMA;
use crate::store::Store;
use crate::types::is_valid_priority;
use kei_entity_store::verbs::create as v_create;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub priority: String,
    #[serde(default)]
    pub milestone_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    pub id: i64,
    pub created_at: i64,
}

#[derive(Debug)]
pub enum Error {
    InvalidTitle,
    InvalidPriority(String),
    StoreError(anyhow::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidTitle => write!(f, "InvalidTitle: title must be non-empty"),
            Error::InvalidPriority(p) => write!(f, "InvalidPriority: {p}"),
            Error::StoreError(e) => write!(f, "StoreError: {e:#}"),
        }
    }
}

impl std::error::Error for Error {}

pub fn run(store: &Store, input: Input) -> Result<Output, Error> {
    validate(&input)?;
    let priority = normalize_priority(&input.priority);
    let payload = json!({
        "title": input.title,
        "description": input.description,
        "priority": priority,
        "status": "pending",
    });
    let v = v_create::run(store.conn(), &TASK_SCHEMA, payload)
        .map_err(|e| Error::StoreError(anyhow::anyhow!("{e}")))?;
    let id = v["id"].as_i64()
        .ok_or_else(|| Error::StoreError(anyhow::anyhow!("missing id")))?;
    let created = v["created_at"].as_i64()
        .ok_or_else(|| Error::StoreError(anyhow::anyhow!("missing created_at")))?;
    Ok(Output { id, created_at: created })
}

fn validate(input: &Input) -> Result<(), Error> {
    if input.title.trim().is_empty() {
        return Err(Error::InvalidTitle);
    }
    if !input.priority.is_empty() && !is_valid_priority(&input.priority) {
        return Err(Error::InvalidPriority(input.priority.clone()));
    }
    Ok(())
}

fn normalize_priority(raw: &str) -> String {
    if raw.is_empty() { "medium".into() } else { raw.to_string() }
}
