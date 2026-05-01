//! Interactions — append-only per-person event log.
//!
//! Stays bespoke (not promoted to engine) because:
//! - `FOREIGN KEY(person_id) REFERENCES people(id) ON DELETE CASCADE`
//!   is not expressible via `EntitySchema` fields.
//! - `interactions_for(person_id)` is a filter query by FK column,
//!   not a generic `list` with offset/limit.
//! - `graph.rs::relationship_graph` runs `GROUP BY person_id,
//!   target_id, channel` which is out of scope for engine verbs.
//! Table DDL still lives in `SOCIAL_SCHEMA::custom_migrations`.

use crate::store::Store;
use anyhow::Result;
use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Interaction {
    pub id: i64,
    pub person_id: i64,
    pub target_id: i64,
    pub interaction_type: String,
    pub channel: String,
    pub content: String,
    pub timestamp: i64,
}

pub fn log_interaction(store: &Store, i: &Interaction) -> Result<i64> {
    let now = Utc::now().timestamp();
    let ts = if i.timestamp == 0 { now } else { i.timestamp };
    let channel = if i.channel.is_empty() { "manual" } else { &i.channel };
    store.conn().execute(
        "INSERT INTO interactions (person_id, target_id, interaction_type,
                                   channel, content, timestamp, created_at)
         VALUES (?1,?2,?3,?4,?5,?6,?7)",
        params![i.person_id, i.target_id, i.interaction_type,
            channel, i.content, ts, now],
    )?;
    Ok(store.conn().last_insert_rowid())
}

pub fn interactions_for(store: &Store, person_id: i64) -> Result<Vec<Interaction>> {
    let mut stmt = store.conn().prepare(
        "SELECT id, person_id, target_id, interaction_type, channel, content, timestamp
         FROM interactions WHERE person_id=?1 ORDER BY timestamp DESC",
    )?;
    let rows = stmt.query_map(params![person_id], |r| {
        Ok(Interaction {
            id: r.get(0)?, person_id: r.get(1)?, target_id: r.get(2)?,
            interaction_type: r.get(3)?, channel: r.get(4)?,
            content: r.get(5)?, timestamp: r.get(6)?,
        })
    })?;
    let mut out = Vec::new();
    for r in rows { out.push(r?); }
    Ok(out)
}
