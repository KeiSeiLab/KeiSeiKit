//! `resources/list` and `resources/read` — skills as MCP resources via
//! kei-skills `SkillRegistry` (canonical SSoT).
//!
//! Each loaded `Skill` becomes one resource:
//!   uri         = `skill://<name>`
//!   name        = `<name>`
//!   mimeType    = `text/markdown`
//!   description = frontmatter.description (truncated to 1024 chars per
//!                 agentskills.io spec) or fallback to `Skill: <name>`.
//!
//! `resources/read` returns the canonical serialized SKILL.md text under
//! the standard MCP `contents` array.
//!
//! HERMES-MIGRATION-PLAN P3.1.b — replaces raw walkdir/fs::read_to_string
//! with `ctx.skills_registry.list()` / `.get(name)` + `kei_skills::format::serialize`.

use crate::protocol::{err, ok, JsonRpcRequest, JsonRpcResponse, ServerContext, INVALID_PARAMS};
use kei_skills::format::serialize;
use serde_json::{json, Value};

pub fn list(req: JsonRpcRequest, ctx: &ServerContext) -> JsonRpcResponse {
    let resources: Vec<Value> = ctx
        .skills_registry
        .list()
        .into_iter()
        .map(skill_to_resource)
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
    let skill = match ctx.skills_registry.get(name) {
        Some(s) => s,
        None => return err(req.id, INVALID_PARAMS, format!("unknown skill: {name}")),
    };
    let text = match serialize(&skill) {
        Ok(t) => t,
        Err(e) => return err(req.id, INVALID_PARAMS, format!("serialize {name}: {e}")),
    };
    ok(
        req.id,
        json!({
            "contents": [{
                "uri": uri,
                "mimeType": "text/markdown",
                "text": text,
            }],
        }),
    )
}

fn skill_to_resource(skill: kei_skills::format::Skill) -> Value {
    let name = skill.frontmatter.name.clone();
    let desc = if skill.frontmatter.description.is_empty() {
        format!("Skill: {name}")
    } else {
        skill.frontmatter.description.clone()
    };
    json!({
        "uri": format!("skill://{name}"),
        "name": name,
        "mimeType": "text/markdown",
        "description": desc,
    })
}
