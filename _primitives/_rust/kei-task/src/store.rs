//! Task store — thin shim over `kei_entity_store::Store`.
//!
//! Layer-A convergence pilot (2026-04-23): generic CRUD (create / get /
//! update) now runs through `kei_entity_store::verbs::*` using the
//! declarative `TASK_SCHEMA`. Public surface is preserved byte-for-byte
//! so existing integration tests and callers (`atoms::create`,
//! `milestones`, `deps`, `search`) compile unchanged.

use crate::schema::TASK_SCHEMA;
use crate::types::Task;
use anyhow::{anyhow, Result};
use kei_entity_store::verbs::{create as v_create, get as v_get, update as v_update};
use kei_entity_store::Store as EntityStore;
use rusqlite::Connection;
use serde_json::{json, Value};
use std::path::Path;

pub struct Store {
    inner: EntityStore,
}

impl Store {
    pub fn open(path: &Path) -> Result<Self> {
        let inner = EntityStore::open(path, &[&TASK_SCHEMA])?;
        Ok(Self { inner })
    }

    pub fn open_memory() -> Result<Self> {
        let inner = EntityStore::open_memory(&[&TASK_SCHEMA])?;
        Ok(Self { inner })
    }

    pub fn conn(&self) -> &Connection { self.inner.conn() }

    pub fn create_task(&self, t: &Task) -> Result<i64> {
        let status = if t.status.is_empty() { "pending" } else { &t.status };
        let priority = if t.priority.is_empty() { "medium" } else { &t.priority };
        let input = json!({
            "title": t.title,
            "description": t.description,
            "status": status,
            "priority": priority,
            "task_type": t.task_type,
            "parent_id": t.parent_id,
            "assigned_to": t.assigned_to,
            "due_date": t.due_date,
            "completed_at": t.completed_at,
            "created_at": t.created_at,
        });
        let v = v_create::run(self.inner.conn(), &TASK_SCHEMA, input)
            .map_err(|e| anyhow!("{e}"))?;
        v["id"].as_i64().ok_or_else(|| anyhow!("missing id in create response"))
    }

    pub fn get_task(&self, id: i64) -> Result<Option<Task>> {
        match v_get::run(self.inner.conn(), &TASK_SCHEMA, json!({ "id": id })) {
            Ok(v) => Ok(Some(task_from_json(v)?)),
            Err(e) if e.exit_code() == 2 => Ok(None),
            Err(e) => Err(anyhow!("{e}")),
        }
    }

    pub fn update_task(&self, t: &Task) -> Result<()> {
        let now = chrono::Utc::now().timestamp();
        let completed = if t.status == "completed" && t.completed_at == 0 { now }
                        else { t.completed_at };
        let input = json!({
            "id": t.id,
            "title": t.title,
            "description": t.description,
            "status": t.status,
            "priority": t.priority,
            "task_type": t.task_type,
            "parent_id": t.parent_id,
            "assigned_to": t.assigned_to,
            "due_date": t.due_date,
            "completed_at": completed,
        });
        v_update::run(self.inner.conn(), &TASK_SCHEMA, input)
            .map_err(|e| anyhow!("{e}"))?;
        Ok(())
    }
}

fn task_from_json(v: Value) -> Result<Task> {
    let obj = v.as_object().ok_or_else(|| anyhow!("expected object in get response"))?;
    Ok(Task {
        id: obj.get("id").and_then(|x| x.as_i64()).unwrap_or(0),
        title: obj.get("title").and_then(|x| x.as_str()).unwrap_or("").to_string(),
        description: obj.get("description").and_then(|x| x.as_str()).unwrap_or("").to_string(),
        status: obj.get("status").and_then(|x| x.as_str()).unwrap_or("").to_string(),
        priority: obj.get("priority").and_then(|x| x.as_str()).unwrap_or("").to_string(),
        task_type: obj.get("task_type").and_then(|x| x.as_str()).unwrap_or("").to_string(),
        parent_id: obj.get("parent_id").and_then(|x| x.as_i64()).unwrap_or(0),
        assigned_to: obj.get("assigned_to").and_then(|x| x.as_str()).unwrap_or("").to_string(),
        due_date: obj.get("due_date").and_then(|x| x.as_i64()).unwrap_or(0),
        completed_at: obj.get("completed_at").and_then(|x| x.as_i64()).unwrap_or(0),
        created_at: obj.get("created_at").and_then(|x| x.as_i64()).unwrap_or(0),
        updated_at: obj.get("updated_at").and_then(|x| x.as_i64()).unwrap_or(0),
    })
}
