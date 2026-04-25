//! Integration test — `tools/call` enforces the 60s timeout (MISS-4).
//!
//! Strategy: write a fake binary that sleeps significantly longer than the
//! cap. Use `tokio::test(start_paused = true)` so virtual time can be
//! advanced past the 60s deadline without the test thread blocking. The
//! real child is reaped by `kill_on_drop(true)` set in `spawn_and_collect`.
//!
//! Real wall-clock spent on this test: a couple of milliseconds — the
//! `sleep 90` child is spawned but killed before it finishes its first
//! second.

#![cfg(unix)]

use kei_mcp::{dispatch, JsonRpcRequest, ServerContext};
use serde_json::json;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::time::Duration;

fn write_sleeping_binary(bin_dir: &Path, crate_name: &str) {
    fs::create_dir_all(bin_dir).unwrap();
    let path = bin_dir.join(crate_name);
    // 90s sleep — well past the 60s cap. `kill_on_drop(true)` reaps it.
    let script = "#!/bin/sh\nsleep 90\necho '{\"never\":\"reached\"}'\n";
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

Sleeps forever.
"#
    );
    fs::write(atoms.join(format!("{verb}.md")), md).unwrap();
}

#[tokio::test(start_paused = true)]
async fn tools_call_returns_internal_error_on_timeout() {
    let tmp = tempfile::tempdir().unwrap();
    let atoms_root = tmp.path().join("atoms-root");
    let bin_dir = tmp.path().join("bin");
    let skills = tmp.path().join("skills");
    fs::create_dir_all(&skills).unwrap();
    write_atom(&atoms_root, "kei-snore", "wait");
    write_sleeping_binary(&bin_dir, "kei-snore");

    std::env::set_var("KEI_RUNTIME_BIN_DIR", &bin_dir);
    let ctx = ServerContext::new(atoms_root, skills);
    let req = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        id: Some(json!(99)),
        method: "tools/call".into(),
        params: Some(json!({
            "name": "kei-snore::wait",
            "arguments": {}
        })),
    };

    // Drive dispatch and virtual-time advance concurrently. With
    // `start_paused = true` the `tokio::time::timeout` inside
    // `invoke_atom` only fires once virtual time crosses 60s. We
    // interleave an `advance` step into the same task so the test
    // does not need `tokio::spawn` (which would require `'static`).
    let dispatch_fut = dispatch(req, &ctx);
    let advance_fut = async {
        tokio::task::yield_now().await;
        tokio::time::advance(Duration::from_secs(61)).await;
        // Park forever — the select picks the dispatch outcome.
        std::future::pending::<()>().await;
    };
    let resp = tokio::select! {
        r = dispatch_fut => r,
        _ = advance_fut => panic!("advance branch should never resolve"),
    };

    std::env::remove_var("KEI_RUNTIME_BIN_DIR");

    assert!(resp.result.is_none(), "result must be unset on timeout");
    let e = resp.error.expect("error must be set");
    assert_eq!(e.code, -32603, "INTERNAL_ERROR per spec");
    assert!(
        e.message.contains("atom timeout"),
        "expected 'atom timeout' in message, got: {}",
        e.message
    );
}
