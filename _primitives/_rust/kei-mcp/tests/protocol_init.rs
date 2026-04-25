//! Integration test — `initialize` handshake response shape.
//!
//! Drives `dispatch` directly (library API) so the test does not depend on
//! spawning the binary. Verifies:
//!   - `serverInfo.name` == "kei-mcp"
//!   - `serverInfo.version` non-empty
//!   - `capabilities.tools`, `.resources`, `.prompts` all present
//!   - client-supplied `protocolVersion` is echoed back

use kei_mcp::{dispatch, JsonRpcRequest, ServerContext};
use serde_json::{json, Value};

fn make_request(method: &str, params: Value) -> JsonRpcRequest {
    JsonRpcRequest {
        jsonrpc: "2.0".into(),
        id: Some(json!(1)),
        method: method.into(),
        params: Some(params),
    }
}

fn empty_ctx() -> ServerContext {
    let tmp = tempfile::tempdir().unwrap();
    let atoms = tmp.path().join("atoms-root");
    let skills = tmp.path().join("skills-root");
    std::fs::create_dir_all(&atoms).unwrap();
    std::fs::create_dir_all(&skills).unwrap();
    let ctx = ServerContext::new(atoms, skills);
    std::mem::forget(tmp); // keep alive — server lifetime
    ctx
}

#[tokio::test]
async fn initialize_returns_server_info() {
    let ctx = empty_ctx();
    let req = make_request("initialize", json!({ "protocolVersion": "2024-11-05" }));
    let resp = dispatch(req, &ctx).await;
    assert_eq!(resp.jsonrpc, "2.0");
    assert_eq!(resp.id, Some(json!(1)));
    let result = resp.result.expect("should have result");
    assert_eq!(result["serverInfo"]["name"], "kei-mcp");
    assert!(result["serverInfo"]["version"].as_str().unwrap().len() > 0);
    assert_eq!(result["protocolVersion"], "2024-11-05");
    assert!(result["capabilities"]["tools"].is_object());
    assert!(result["capabilities"]["resources"].is_object());
    assert!(result["capabilities"]["prompts"].is_object());
}

#[tokio::test]
async fn initialize_falls_back_to_default_protocol_when_unset() {
    let ctx = empty_ctx();
    let req = make_request("initialize", json!({}));
    let resp = dispatch(req, &ctx).await;
    let result = resp.result.expect("should have result");
    let pv = result["protocolVersion"].as_str().expect("protocolVersion string");
    assert!(!pv.is_empty(), "default protocol version must not be empty");
}

#[tokio::test]
async fn unknown_method_returns_method_not_found() {
    let ctx = empty_ctx();
    let req = make_request("totally/bogus", json!({}));
    let resp = dispatch(req, &ctx).await;
    assert!(resp.result.is_none());
    let e = resp.error.expect("should have error");
    assert_eq!(e.code, -32601);
    assert!(e.message.contains("totally/bogus"));
}
