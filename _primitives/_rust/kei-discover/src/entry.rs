//! `Entry` — typed view of one `discover_index` row returned by
//! `list_available` / `search`.
//!
//! Conversion from the engine's `serde_json::Value` row is centralised
//! here so every public API function returns the same typed struct.

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    pub id: i64,
    pub slug: String,
    pub author: String,
    pub source_url: String,
    pub description: String,
    pub installed: bool,
    pub last_seen_ts: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Entry {
    /// Decode one row produced by `kei_entity_store::verbs::{get,list,search}`.
    pub fn from_json(v: &Value) -> Self {
        Self {
            id: as_i64(v, "id"),
            slug: as_str(v, "slug"),
            author: as_str(v, "author"),
            source_url: as_str(v, "source_url"),
            description: as_str(v, "description"),
            installed: as_i64(v, "installed") != 0,
            last_seen_ts: as_i64(v, "last_seen_ts"),
            created_at: as_i64(v, "created_at"),
            updated_at: as_i64(v, "updated_at"),
        }
    }
}

fn as_i64(v: &Value, key: &str) -> i64 {
    v.get(key).and_then(|x| x.as_i64()).unwrap_or(0)
}

fn as_str(v: &Value, key: &str) -> String {
    v.get(key).and_then(|x| x.as_str()).unwrap_or("").to_string()
}
