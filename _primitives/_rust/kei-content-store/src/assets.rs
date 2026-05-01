use crate::schema::CONTENT_SCHEMA;
use crate::store::Store;
use anyhow::{anyhow, Result};
use kei_entity_store::verbs::{create as v_create, get as v_get};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Asset {
    pub id: i64,
    pub unit_type: String,
    pub title: String,
    pub content: String,
    pub media_type: String,
    pub file_path: String,
    pub file_hash: String,
    pub provider: String,
    pub cost_cents: i64,
    pub parent_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

pub fn register_asset(store: &Store, a: &Asset) -> Result<i64> {
    let unit_type = if a.unit_type.is_empty() { "asset" } else { &a.unit_type };
    let input = json!({
        "unit_type":  unit_type,
        "title":      a.title,
        "content":    a.content,
        "media_type": a.media_type,
        "file_path":  a.file_path,
        "file_hash":  a.file_hash,
        "provider":   a.provider,
        "cost_cents": a.cost_cents,
        "parent_id":  a.parent_id,
    });
    let v = v_create::run(store.conn(), &CONTENT_SCHEMA, input)
        .map_err(|e| anyhow!("{e}"))?;
    v["id"].as_i64().ok_or_else(|| anyhow!("missing id in create response"))
}

pub fn get_asset(store: &Store, id: i64) -> Result<Option<Asset>> {
    match v_get::run(store.conn(), &CONTENT_SCHEMA, json!({ "id": id })) {
        Ok(v) => Ok(Some(asset_from_json(v)?)),
        Err(e) if e.exit_code() == 2 => Ok(None),
        Err(e) => Err(anyhow!("{e}")),
    }
}

fn asset_from_json(v: Value) -> Result<Asset> {
    let obj = v.as_object().ok_or_else(|| anyhow!("expected object in get response"))?;
    Ok(Asset {
        id:         obj.get("id").and_then(|x| x.as_i64()).unwrap_or(0),
        unit_type:  obj.get("unit_type").and_then(|x| x.as_str()).unwrap_or("").to_string(),
        title:      obj.get("title").and_then(|x| x.as_str()).unwrap_or("").to_string(),
        content:    obj.get("content").and_then(|x| x.as_str()).unwrap_or("").to_string(),
        media_type: obj.get("media_type").and_then(|x| x.as_str()).unwrap_or("").to_string(),
        file_path:  obj.get("file_path").and_then(|x| x.as_str()).unwrap_or("").to_string(),
        file_hash:  obj.get("file_hash").and_then(|x| x.as_str()).unwrap_or("").to_string(),
        provider:   obj.get("provider").and_then(|x| x.as_str()).unwrap_or("").to_string(),
        cost_cents: obj.get("cost_cents").and_then(|x| x.as_i64()).unwrap_or(0),
        parent_id:  obj.get("parent_id").and_then(|x| x.as_i64()).unwrap_or(0),
        created_at: obj.get("created_at").and_then(|x| x.as_i64()).unwrap_or(0),
        updated_at: obj.get("updated_at").and_then(|x| x.as_i64()).unwrap_or(0),
    })
}
