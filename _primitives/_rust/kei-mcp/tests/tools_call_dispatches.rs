//! Integration test — `tools/call` dispatches to `<crate> run-atom <verb>`.
//!
//! Strategy: write a tiny shell-script "fake binary" into a tempdir, point
//! `KEI_RUNTIME_BIN_DIR` at that dir, and verify the handler's response
//! contains the JSON the script printed. This proves:
//!   - tool name is parsed into (crate, verb)
//!   - resolve_binary picks up KEI_RUNTIME_BIN_DIR
//!   - stdout JSON is captured into `content[0].text`

#![cfg(unix)]

use kei_mcp::{dispatch, JsonRpcRequest, ServerContext};
use serde_json::{json, Value};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

fn write_fake_binary(bin_dir: &Path, crate_name: &str) {
    fs::create_dir_all(bin_dir).unwrap();
    let path = bin_dir.join(crate_name);
    // The fake binary echoes a fixed JSON object regardless of args/stdin.
    let script = "#!/bin/sh\necho '{\"echoed\":true,\"verb_seen\":\"'\"$2\"'\"}'\n";
    fs::write(&path, script).unwrap();
    let mut perms = fs::metadata(&path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&path, perms).unwrap();
}

fn write_atom(root: &Path, crate_name: &str, verb: &str) {
    let atoms = root.join(crate_name).join("atoms");
    let schemas = atoms.join("schemas");
    fs::create_dir_all(&schemas).unwrap();
    fs::write(schemas.join(format!("{verb}-input.json")), "{}").unwrap();
    let md = format!(
        r#"---
atom: {crate_name}::{verb}
kind: query
version: "0.1.0"
input:
  schema: schemas/{verb}-input.json
side_effects: []
idempotent: true
stability: stable
---

# {crate_name}::{verb}

Search atoms.
"#
    );
    fs::write(atoms.join(format!("{verb}.md")), md).unwrap();
}

#[tokio::test]
async fn tools_call_resolves_binary_and_returns_stdout_json() {
    let tmp = tempfile::tempdir().unwrap();
    let atoms_root = tmp.path().join("atoms-root");
    let bin_dir = tmp.path().join("bin");
    let skills = tmp.path().join("skills");
    fs::create_dir_all(&skills).unwrap();
    write_atom(&atoms_root, "kei-task", "search");
    write_fake_binary(&bin_dir, "kei-task");

    // Scope the env var to this test invocation. KEI_RUNTIME_BIN_DIR is the
    // same env the kei-runtime binary uses, so handlers/tools.rs picks it up.
    std::env::set_var("KEI_RUNTIME_BIN_DIR", &bin_dir);

    let ctx = ServerContext::new(atoms_root, skills);
    let req = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        id: Some(json!(42)),
        method: "tools/call".into(),
        params: Some(json!({
            "name": "kei-task::search",
            "arguments": { "q": "anything" }
        })),
    };
    let resp = dispatch(req, &ctx).await;
    std::env::remove_var("KEI_RUNTIME_BIN_DIR");

    let result = resp.result.expect("expected success result");
    assert_eq!(result["isError"], false);
    let content = result["content"].as_array().expect("content array");
    let payload_str = content[0]["text"].as_str().expect("text payload");
    let payload: Value = serde_json::from_str(payload_str).expect("payload is JSON");
    assert_eq!(payload["echoed"], true);
    assert_eq!(payload["verb_seen"], "search");
}

#[tokio::test]
async fn tools_call_unknown_tool_yields_error() {
    let tmp = tempfile::tempdir().unwrap();
    let atoms_root = tmp.path().join("atoms-root");
    let skills = tmp.path().join("skills");
    fs::create_dir_all(&atoms_root).unwrap();
    fs::create_dir_all(&skills).unwrap();
    let ctx = ServerContext::new(atoms_root, skills);
    let req = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        id: Some(json!(43)),
        method: "tools/call".into(),
        params: Some(json!({ "name": "kei-nope::nada", "arguments": {} })),
    };
    let resp = dispatch(req, &ctx).await;
    assert!(resp.result.is_none());
    let e = resp.error.expect("error");
    assert!(e.message.contains("unknown tool"), "got: {}", e.message);
}
