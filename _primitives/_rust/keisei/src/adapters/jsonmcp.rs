//! Shared JSON MCP-server merge helpers for JSON-keyed adapters.
//!
//! Constructor Pattern: single responsibility — own the "load → upsert /
//! remove under a named outer key → atomic write" pipeline that every
//! JSON-keyed adapter (claude-code, cursor, zed) was duplicating in
//! ~95%-identical form. Continue is YAML-based and does NOT use this.
//!
//! Error surfacing is uniform across the three callers: JSON parse
//! failures flow through `Error::ConfigParseError` rather than the raw
//! serde_json error (zed was already doing this before the dedup; the
//! other two silently converted via `#[from]` and lost the config path).

use crate::brain::Brain;
use crate::error::{Error, Result};
use crate::fsx::write_atomic_json;
use serde_json::{json, Map, Value};
use std::path::Path;

/// Load a JSON document from disk, returning `{}` for a missing or
/// empty file. Parse errors are wrapped in `ConfigParseError { path }`
/// so the user sees which file is malformed.
pub fn load_json_or_empty(path: &Path) -> Result<Value> {
    if !path.is_file() {
        return Ok(json!({}));
    }
    let raw = std::fs::read_to_string(path)?;
    if raw.trim().is_empty() {
        return Ok(json!({}));
    }
    serde_json::from_str(&raw).map_err(|e| Error::ConfigParseError {
        path: path.to_path_buf(),
        reason: e.to_string(),
    })
}

/// Build the MCP-server entry shape used by every JSON-keyed adapter.
/// Separated so a test can assert shape stability across call-sites.
pub fn build_mcp_entry(brain: &Brain) -> Result<Value> {
    let mcp = brain.mcp_server_path()?;
    Ok(json!({
        "command": mcp.to_string_lossy(),
        "args": [],
        "env": {
            "KEISEI_BRAIN_ROOT": brain.root.to_string_lossy(),
            "KEISEI_BRAIN_NAME": brain.name(),
        }
    }))
}

/// Upsert `{entry_key: new_entry}` under `doc[outer_key]`. If the outer
/// key is missing, creates it as an empty object first. If the entry
/// already exists with different content, returns `Error::NameConflict`
/// with `existing_client = client_label` so the user sees which adapter
/// is guarding the collision.
pub fn upsert_under_key(
    doc: &mut Value,
    outer_key: &str,
    entry_key: &str,
    new_entry: Value,
    client_label: &str,
) -> Result<()> {
    if !doc.is_object() {
        *doc = json!({});
    }
    let obj = doc.as_object_mut().expect("doc is object after guard");
    let servers = obj
        .entry(outer_key.to_string())
        .or_insert_with(|| Value::Object(Map::new()));
    if !servers.is_object() {
        *servers = Value::Object(Map::new());
    }
    let map = servers.as_object_mut().expect("servers is object");
    if let Some(existing) = map.get(entry_key) {
        if existing != &new_entry {
            return Err(Error::NameConflict {
                name: entry_key.to_string(),
                existing_client: client_label.to_string(),
            });
        }
    }
    map.insert(entry_key.to_string(), new_entry);
    Ok(())
}

/// Remove `doc[outer_key][entry_key]` and prune `outer_key` when it's
/// left empty. Returns whether anything was removed.
pub fn remove_under_key(doc: &mut Value, outer_key: &str, entry_key: &str) -> bool {
    if !doc.is_object() {
        return false;
    }
    let obj = doc.as_object_mut().expect("doc is object after guard");
    let mut removed = false;
    if let Some(servers) = obj.get_mut(outer_key) {
        if let Some(map) = servers.as_object_mut() {
            removed = map.remove(entry_key).is_some();
            if map.is_empty() {
                obj.remove(outer_key);
            }
        }
    }
    removed
}

/// Atomically persist the document to the target path.
pub fn persist(doc: &Value, path: &Path) -> Result<()> {
    write_atomic_json(path, doc)
}
