//! `initialize` method — MCP handshake.
//!
//! Returns server info + capability descriptor. Capabilities advertised:
//! `tools` (list+call), `resources` (list+read), `prompts` (list+get,
//! placeholder for now). Per MCP spec the protocol version is echoed
//! back to the client.

use crate::protocol::{ok, JsonRpcRequest, JsonRpcResponse, ServerContext};
use serde_json::json;

/// Default protocol version we advertise. Clients may negotiate a different
/// one via `params.protocolVersion`; we echo whichever the client sent if
/// present, otherwise fall back to this.
const DEFAULT_PROTOCOL_VERSION: &str = "2024-11-05";

pub fn handle(req: JsonRpcRequest, ctx: &ServerContext) -> JsonRpcResponse {
    let version = client_protocol_version(&req).unwrap_or_else(|| DEFAULT_PROTOCOL_VERSION.to_string());
    let result = json!({
        "protocolVersion": version,
        "capabilities": capabilities(),
        "serverInfo": {
            "name": ctx.server_name,
            "version": ctx.server_version,
        },
    });
    ok(req.id, result)
}

fn client_protocol_version(req: &JsonRpcRequest) -> Option<String> {
    req.params
        .as_ref()?
        .get("protocolVersion")?
        .as_str()
        .map(String::from)
}

/// Capability matrix advertised in the handshake. Each sub-object is an
/// MCP capability descriptor — empty object means "supported, no extra
/// flags". `prompts` is a placeholder stub for now.
fn capabilities() -> serde_json::Value {
    json!({
        "tools": { "listChanged": false },
        "resources": { "listChanged": false, "subscribe": false },
        "prompts": { "listChanged": false },
    })
}
