//! Adjacency view — returns task graph as edge-list for visualisation.

use crate::store::Store;
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct TaskEdge {
    pub task_id: i64,
    pub depends_on: i64,
    pub dep_type: String,
}

pub fn list_edges(store: &Store) -> Result<Vec<TaskEdge>> {
    let mut stmt = store.conn().prepare(
        "SELECT task_id, depends_on, dep_type FROM task_deps"
    )?;
    let rows = stmt.query_map([], |r| {
        Ok(TaskEdge {
            task_id: r.get(0)?,
            depends_on: r.get(1)?,
            dep_type: r.get(2)?,
        })
    })?;
    let mut out = Vec::new();
    for r in rows { out.push(r?); }
    Ok(out)
}
