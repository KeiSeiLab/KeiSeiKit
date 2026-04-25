//! `resources/list` and `resources/read` — skills as MCP resources.
//!
//! Each `<skills_root>/<name>/SKILL.md` becomes one resource:
//!   uri        = `skill://<name>`
//!   name       = `<name>`
//!   mimeType   = `text/markdown`
//!   description = "Skill: <name>" (literal — no parsing of frontmatter
//!                 yet; richer description can be added later by reading
//!                 the SKILL.md frontmatter `description:` field)
//!
//! `resources/read` returns the SKILL.md text under the standard MCP
//! `contents` array.

use crate::protocol::{err, ok, JsonRpcRequest, JsonRpcResponse, ServerContext, INVALID_PARAMS};
use serde_json::{json, Value};
use std::path::Path;
use walkdir::WalkDir;

pub fn list(req: JsonRpcRequest, ctx: &ServerContext) -> JsonRpcResponse {
    let resources: Vec<Value> = enumerate_skills(&ctx.skills_root)
        .into_iter()
        .map(|name| {
            json!({
                "uri": format!("skill://{name}"),
                "name": name,
                "mimeType": "text/markdown",
                "description": format!("Skill: {name}"),
            })
        })
        .collect();
    ok(req.id, json!({ "resources": resources }))
}

pub fn read(req: JsonRpcRequest, ctx: &ServerContext) -> JsonRpcResponse {
    let uri = match req.params.as_ref().and_then(|p| p.get("uri")).and_then(Value::as_str) {
        Some(u) => u.to_string(),
        None => return err(req.id, INVALID_PARAMS, "missing uri"),
    };
    let name = match uri.strip_prefix("skill://") {
        Some(n) => n,
        None => return err(req.id, INVALID_PARAMS, format!("not a skill uri: {uri}")),
    };
    let path = ctx.skills_root.join(name).join("SKILL.md");
    let text = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) => return err(req.id, INVALID_PARAMS, format!("cannot read {}: {e}", path.display())),
    };
    ok(req.id, json!({
        "contents": [{
            "uri": uri,
            "mimeType": "text/markdown",
            "text": text,
        }],
    }))
}

/// Walk `<root>/*/SKILL.md` and return the directory names (sorted).
/// `follow_links(false)` matches discover_atoms hardening.
fn enumerate_skills(root: &Path) -> Vec<String> {
    let mut names: Vec<String> = WalkDir::new(root)
        .max_depth(2)
        .follow_links(false)
        .into_iter()
        .flatten()
        .filter(|e| e.file_name() == "SKILL.md")
        .filter_map(|e| {
            e.path()
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .map(String::from)
        })
        .collect();
    names.sort();
    names.dedup();
    names
}
