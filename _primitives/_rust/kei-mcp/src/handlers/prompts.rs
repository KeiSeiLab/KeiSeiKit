//! `prompts/list` and `prompts/get` — placeholder. Returns an empty list.
//!
//! Future: walk a prompts directory (TBD) and emit MCP prompt descriptors.
//! For now we honour the spec by returning a well-formed empty `prompts`
//! array on list, and a `not found` error on get.

use crate::protocol::{err, ok, JsonRpcRequest, JsonRpcResponse, ServerContext, INVALID_PARAMS};
use serde_json::{json, Value};

pub fn list(req: JsonRpcRequest, _ctx: &ServerContext) -> JsonRpcResponse {
    let empty: Vec<Value> = Vec::new();
    ok(req.id, json!({ "prompts": empty }))
}

pub fn get(req: JsonRpcRequest, _ctx: &ServerContext) -> JsonRpcResponse {
    let name = req
        .params
        .as_ref()
        .and_then(|p| p.get("name"))
        .and_then(Value::as_str)
        .unwrap_or("<missing>");
    err(
        req.id,
        INVALID_PARAMS,
        format!("prompt not found: {name} (server has no prompts registered)"),
    )
}
