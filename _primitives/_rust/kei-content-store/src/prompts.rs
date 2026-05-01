//! Prompts — hash-deduplicated prompt registry.
//!
//! Stays bespoke (not promoted to engine) because `register_prompt`
//! uses `INSERT OR IGNORE` + re-query by `UNIQUE(prompt_hash, model)`
//! to collapse duplicate text+model submissions to the same id.
//! The engine `create` verb is plain `INSERT` (no OR IGNORE), which
//! would break `prompt_dedup_by_hash` semantics. The table DDL still
//! lives in `CONTENT_SCHEMA::custom_migrations`.

use crate::store::Store;
use anyhow::Result;
use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Prompt {
    pub id: i64,
    pub prompt_text: String,
    pub prompt_hash: String,
    pub prompt_type: String,
    pub model: String,
    pub version: i64,
    pub parent_id: i64,
    pub created_at: i64,
}

pub fn register_prompt(store: &Store, p: &Prompt) -> Result<i64> {
    let now = Utc::now().timestamp();
    let hash = hash_prompt(&p.prompt_text);
    store.conn().execute(
        "INSERT OR IGNORE INTO prompts
            (prompt_text, prompt_hash, prompt_type, model, version, parent_id, created_at)
         VALUES (?1,?2,?3,?4,?5,?6,?7)",
        params![p.prompt_text, hash, p.prompt_type, p.model,
            if p.version == 0 { 1 } else { p.version }, p.parent_id, now],
    )?;
    let id: i64 = store.conn().query_row(
        "SELECT id FROM prompts WHERE prompt_hash=?1 AND model=?2",
        params![hash, p.model], |r| r.get(0))?;
    Ok(id)
}

pub fn history(store: &Store, parent_id: i64) -> Result<Vec<Prompt>> {
    let mut stmt = store.conn().prepare(
        "SELECT id, prompt_text, prompt_hash, prompt_type, model, version,
                parent_id, created_at
         FROM prompts WHERE parent_id=?1 OR id=?1 ORDER BY created_at",
    )?;
    let rows = stmt.query_map(params![parent_id], |r| {
        Ok(Prompt {
            id: r.get(0)?, prompt_text: r.get(1)?, prompt_hash: r.get(2)?,
            prompt_type: r.get(3)?, model: r.get(4)?, version: r.get(5)?,
            parent_id: r.get(6)?, created_at: r.get(7)?,
        })
    })?;
    let mut out = Vec::new();
    for r in rows { out.push(r?); }
    Ok(out)
}

fn hash_prompt(s: &str) -> String {
    let d = Sha256::digest(s.as_bytes());
    format!("{:x}", d)
}
