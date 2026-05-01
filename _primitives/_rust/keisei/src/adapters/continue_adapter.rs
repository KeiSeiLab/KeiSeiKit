//! Continue.dev adapter — writes MCP server entry into `~/.continue/`.
//!
//! Config path strategy [UNVERIFIED — see note]:
//!   1. If `~/.continue/config.yaml` exists → YAML mode
//!   2. Else if `~/.continue/config.json` exists → JSON mode
//!   3. Else if `~/.continue/` exists → create `config.yaml` fresh
//!   4. Else `detect()` returns false (graceful)
//!
//! Schema (both forms), under top-level `mcpServers`:
//! ```yaml
//! mcpServers:
//!   - name: keisei
//!     command: /path/to/kei-mcp-server
//!     args: []
//!     env:
//!       KEISEI_BRAIN_ROOT: /Volumes/Brain1
//! ```
//!
//! NOTE: Continue's exact MCP/plugin schema is [UNVERIFIED] in this
//! session. Adapter uses list-form `mcpServers` from v0.18 prototypes +
//! public Continue `config.yaml` docs. Detach preserves unrelated keys.
//!
//! Security (v0.19 audit): collision-safe — existing `name: keisei` with
//! different content → `NameConflict`, no silent overwrite.

use crate::adapter::ClientAdapter;
use crate::brain::Brain;
use crate::error::{Error, Result};
use crate::fsx::write_atomic;
use crate::paths;
use crate::scope::Scope;
use serde_json::{json, Value};
use std::path::PathBuf;

pub const SERVER_NAME: &str = "keisei";
pub const CLIENT_NAME: &str = "continue";

#[derive(Clone, Copy, PartialEq, Eq)]
enum Form {
    Yaml,
    Json,
}

pub struct ContinueAdapter;

impl ContinueAdapter {
    pub fn new() -> Self {
        Self
    }

    fn continue_dir(&self) -> PathBuf {
        paths::resolve_home().join(".continue")
    }

    fn pick_form_and_path(&self) -> (Form, PathBuf) {
        let dir = self.continue_dir();
        let yaml = dir.join("config.yaml");
        let json = dir.join("config.json");
        if yaml.is_file() {
            (Form::Yaml, yaml)
        } else if json.is_file() {
            (Form::Json, json)
        } else {
            // Default for a fresh install: YAML (Continue's preferred form).
            (Form::Yaml, yaml)
        }
    }
}

impl Default for ContinueAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientAdapter for ContinueAdapter {
    fn name(&self) -> &str {
        CLIENT_NAME
    }

    fn detect(&self) -> bool {
        self.continue_dir().is_dir()
    }

    fn supported_scopes(&self) -> &[Scope] {
        // Continue has no per-project MCP config surface today — user only.
        &[Scope::User]
    }

    fn attach(&self, brain: &Brain, _scope: Scope) -> Result<()> {
        let (form, cfg) = self.pick_form_and_path();
        if let Some(parent) = cfg.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut doc = load_doc(&cfg, form)?;
        merge_entry(&mut doc, brain)?;
        write_doc(&cfg, form, &doc)
    }

    fn detach(&self, _brain_name: &str, _scope: Scope) -> Result<()> {
        let (form, cfg) = self.pick_form_and_path();
        if !cfg.is_file() {
            return Ok(());
        }
        let mut doc = load_doc(&cfg, form)?;
        remove_entry(&mut doc);
        write_doc(&cfg, form, &doc)
    }

    fn config_path(&self, _scope: Scope) -> PathBuf {
        self.pick_form_and_path().1
    }

    fn post_attach_hint(&self, brain: &Brain, _scope: Scope) -> String {
        format!(
            "reload the Continue extension in VS Code — '{}' goes under Experimental MCP",
            brain.name()
        )
    }
}

/// Load doc as a generic `serde_json::Value`. YAML → Value via serde_yaml,
/// JSON → Value directly. Unifying on `Value` keeps the merge logic form-
/// independent.
fn load_doc(cfg: &std::path::Path, form: Form) -> Result<Value> {
    if !cfg.is_file() {
        return Ok(json!({}));
    }
    let raw = std::fs::read_to_string(cfg)?;
    if raw.trim().is_empty() {
        return Ok(json!({}));
    }
    let parsed: Value = match form {
        Form::Yaml => serde_yaml::from_str(&raw).map_err(|e| Error::ConfigParseError {
            path: cfg.to_path_buf(),
            reason: e.to_string(),
        })?,
        Form::Json => serde_json::from_str(&raw)?,
    };
    Ok(parsed)
}

fn write_doc(cfg: &std::path::Path, form: Form, doc: &Value) -> Result<()> {
    let text = match form {
        Form::Yaml => serde_yaml::to_string(doc)?,
        Form::Json => serde_json::to_string_pretty(doc)?,
    };
    write_atomic(cfg, &text)
}

fn build_entry(brain: &Brain) -> Result<Value> {
    let mcp = brain.mcp_server_path()?;
    Ok(json!({
        "name": SERVER_NAME,
        "command": mcp.to_string_lossy(),
        "args": [],
        "env": {
            "KEISEI_BRAIN_ROOT": brain.root.to_string_lossy(),
            "KEISEI_BRAIN_NAME": brain.name(),
        }
    }))
}

fn merge_entry(doc: &mut Value, brain: &Brain) -> Result<()> {
    if !doc.is_object() {
        *doc = json!({});
    }
    let obj = doc.as_object_mut().expect("doc is object after guard");
    let entry = build_entry(brain)?;
    let servers = obj
        .entry("mcpServers".to_string())
        .or_insert_with(|| Value::Array(Vec::new()));
    if !servers.is_array() {
        *servers = Value::Array(Vec::new());
    }
    let arr = servers.as_array_mut().expect("array after guard");
    if let Some(existing) = arr
        .iter()
        .find(|v| v.get("name").and_then(|n| n.as_str()) == Some(SERVER_NAME))
    {
        if existing != &entry {
            return Err(Error::NameConflict {
                name: SERVER_NAME.to_string(),
                existing_client: CLIENT_NAME.to_string(),
            });
        }
    }
    arr.retain(|v| v.get("name").and_then(|n| n.as_str()) != Some(SERVER_NAME));
    arr.push(entry);
    Ok(())
}

fn remove_entry(doc: &mut Value) {
    if !doc.is_object() {
        return;
    }
    let obj = doc.as_object_mut().expect("doc is object after guard");
    if let Some(servers) = obj.get_mut("mcpServers") {
        if let Some(arr) = servers.as_array_mut() {
            arr.retain(|v| v.get("name").and_then(|n| n.as_str()) != Some(SERVER_NAME));
            if arr.is_empty() {
                obj.remove("mcpServers");
            }
        }
    }
}
