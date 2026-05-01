//! Dependency edges + cycle detection + dependency-chain traversal.

use crate::store::Store;
use crate::types::is_valid_dep;
use anyhow::{anyhow, Result};
use rusqlite::params;
use std::collections::HashSet;

/// Add a dependency. Refuses a cycle (taskId -> dependsOn -> ... -> taskId).
pub fn add_dependency(store: &Store, task_id: i64, depends_on: i64, dep_type: &str) -> Result<()> {
    if task_id == depends_on {
        return Err(anyhow!("self-dependency forbidden"));
    }
    let dt = if dep_type.is_empty() { "blocks" } else { dep_type };
    if !is_valid_dep(dt) {
        return Err(anyhow!("invalid dep type: {dt}"));
    }
    if creates_cycle(store, task_id, depends_on)? {
        return Err(anyhow!("cycle: {task_id} -> {depends_on} would close a loop"));
    }
    store.conn().execute(
        "INSERT OR IGNORE INTO task_deps (task_id, depends_on, dep_type) VALUES (?1,?2,?3)",
        params![task_id, depends_on, dt],
    )?;
    Ok(())
}

/// True if adding task_id -> depends_on would create a cycle.
fn creates_cycle(store: &Store, task_id: i64, depends_on: i64) -> Result<bool> {
    // If depends_on reaches task_id via existing deps, cycle would close.
    let mut stack = vec![depends_on];
    let mut seen: HashSet<i64> = HashSet::new();
    while let Some(cur) = stack.pop() {
        if cur == task_id {
            return Ok(true);
        }
        if !seen.insert(cur) {
            continue;
        }
        let mut stmt = store.conn().prepare("SELECT depends_on FROM task_deps WHERE task_id=?1")?;
        let rows = stmt.query_map(params![cur], |r| r.get::<_, i64>(0))?;
        for row in rows {
            stack.push(row?);
        }
    }
    Ok(false)
}

/// Full dependency chain (BFS transitive closure).
pub fn dependency_chain(store: &Store, task_id: i64) -> Result<Vec<i64>> {
    let mut seen: HashSet<i64> = HashSet::new();
    let mut frontier = vec![task_id];
    let mut chain: Vec<i64> = Vec::new();
    while let Some(cur) = frontier.pop() {
        let mut stmt = store.conn().prepare("SELECT depends_on FROM task_deps WHERE task_id=?1")?;
        let rows = stmt.query_map(params![cur], |r| r.get::<_, i64>(0))?;
        for row in rows {
            let id = row?;
            if seen.insert(id) {
                chain.push(id);
                frontier.push(id);
            }
        }
    }
    Ok(chain)
}
