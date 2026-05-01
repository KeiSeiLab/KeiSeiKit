//! Relationship graph — who interacted with whom, grouped by channel.

use crate::store::Store;
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Pair {
    pub person_id: i64,
    pub target_id: i64,
    pub channel: String,
    pub count: i64,
}

pub fn relationship_graph(store: &Store) -> Result<Vec<Pair>> {
    let mut stmt = store.conn().prepare(
        "SELECT person_id, target_id, channel, COUNT(*) FROM interactions
         WHERE target_id > 0 GROUP BY person_id, target_id, channel",
    )?;
    let rows = stmt.query_map([], |r| {
        Ok(Pair {
            person_id: r.get(0)?,
            target_id: r.get(1)?,
            channel: r.get(2)?,
            count: r.get(3)?,
        })
    })?;
    let mut out = Vec::new();
    for r in rows { out.push(r?); }
    Ok(out)
}
