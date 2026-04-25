//! Milestone CRUD + task→milestone linking.

use crate::store::Store;
use crate::types::Milestone;
use anyhow::Result;
use chrono::Utc;
use rusqlite::params;

pub fn create_milestone(store: &Store, m: &Milestone) -> Result<i64> {
    let now = Utc::now().timestamp();
    let created = if m.created_at == 0 { now } else { m.created_at };
    let status = if m.status.is_empty() { "open" } else { &m.status };
    store.conn().execute(
        "INSERT INTO milestones (name, description, target_date, status, created_at)
         VALUES (?1,?2,?3,?4,?5)",
        params![m.name, m.description, m.target_date, status, created],
    )?;
    Ok(store.conn().last_insert_rowid())
}

pub fn link_task_to_milestone(store: &Store, task_id: i64, milestone_id: i64) -> Result<()> {
    store.conn().execute(
        "INSERT OR IGNORE INTO task_milestones (task_id, milestone_id) VALUES (?1,?2)",
        params![task_id, milestone_id],
    )?;
    Ok(())
}

pub fn tasks_in_milestone(store: &Store, milestone_id: i64) -> Result<Vec<i64>> {
    let mut stmt = store.conn().prepare(
        "SELECT task_id FROM task_milestones WHERE milestone_id=?1",
    )?;
    let rows = stmt.query_map(params![milestone_id], |r| r.get::<_, i64>(0))?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}
