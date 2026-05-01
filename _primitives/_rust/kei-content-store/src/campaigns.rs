//! Campaigns + campaign_assets join.
//!
//! `create_campaign` delegates to `kei_entity_store::verbs::create` under
//! `CAMPAIGNS_SCHEMA` — plain INTEGER-PK CRUD, engine-owned since
//! 2026-04-23.
//!
//! `attach_asset` / `campaign_assets` stay bespoke: `campaign_assets`
//! has a composite `(campaign_id, asset_id)` PK with no single-column
//! id, so it cannot be described as an `EntitySchema` (engine requires
//! exactly one PK field). The attach path also uses `INSERT OR IGNORE`
//! for idempotent joins, which the engine's plain-INSERT `create` verb
//! would not preserve.

use crate::schema::CAMPAIGNS_SCHEMA;
use crate::store::Store;
use anyhow::{anyhow, Result};
use kei_entity_store::verbs::create as v_create;
use rusqlite::params;
use serde_json::json;

pub fn create_campaign(store: &Store, name: &str, description: &str) -> Result<i64> {
    let input = json!({ "name": name, "description": description });
    let v = v_create::run(store.conn(), &CAMPAIGNS_SCHEMA, input)
        .map_err(|e| anyhow!("{e}"))?;
    v["id"].as_i64().ok_or_else(|| anyhow!("missing id in create response"))
}

pub fn attach_asset(store: &Store, campaign_id: i64, asset_id: i64) -> Result<()> {
    store.conn().execute(
        "INSERT OR IGNORE INTO campaign_assets (campaign_id, asset_id) VALUES (?1,?2)",
        params![campaign_id, asset_id],
    )?;
    Ok(())
}

pub fn campaign_assets(store: &Store, campaign_id: i64) -> Result<Vec<i64>> {
    let mut stmt = store.conn().prepare(
        "SELECT asset_id FROM campaign_assets WHERE campaign_id=?1"
    )?;
    let rows = stmt.query_map(params![campaign_id], |r| r.get::<_, i64>(0))?;
    let mut out = Vec::new();
    for r in rows { out.push(r?); }
    Ok(out)
}
