//! Integration test — `tools/list` walks a mock atom registry and emits
//! MCP-shape tool descriptors.
//!
//! Builds a minimal `<root>/<crate>/atoms/<verb>.md` layout in tempdir and
//! verifies that every atom round-trips into a `{name, description,
//! inputSchema}` triple.

use kei_mcp::{dispatch, JsonRpcRequest, ServerContext};
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

fn write_atom(root: &Path, crate_name: &str, verb: &str, body_first_para: &str) {
    let atoms = root.join(crate_name).join("atoms");
    let schemas = atoms.join("schemas");
    fs::create_dir_all(&schemas).unwrap();
    let input_schema_name = format!("{verb}-input.json");
    let output_schema_name = format!("{verb}-output.json");
    fs::write(
        schemas.join(&input_schema_name),
        r#"{"type":"object","properties":{"q":{"type":"string"}}}"#,
    )
    .unwrap();
    fs::write(schemas.join(&output_schema_name), "{}").unwrap();
    let md = format!(
        r#"---
atom: {crate_name}::{verb}
kind: query
version: "0.1.0"
input:
  schema: schemas/{input_schema_name}
output:
  schema: schemas/{output_schema_name}
side_effects: []
idempotent: true
stability: stable
---

# {crate_name}::{verb}

{body_first_para}

Followup paragraph that should NOT appear in the description.
"#,
    );
    fs::write(atoms.join(format!("{verb}.md")), md).unwrap();
}

fn make_ctx(root: std::path::PathBuf) -> ServerContext {
    let skills = root.join("__skills_unused");
    fs::create_dir_all(&skills).unwrap();
    ServerContext::new(root, skills)
}

#[tokio::test]
async fn tools_list_returns_two_atoms_with_descriptors() {
    let tmp = tempfile::tempdir().unwrap();
    write_atom(tmp.path(), "kei-task", "search", "Search across tasks by query.");
    write_atom(tmp.path(), "kei-sage", "ask", "Ask the sage a question.");
    let ctx = make_ctx(tmp.path().to_path_buf());

    let req = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        id: Some(json!(7)),
        method: "tools/list".into(),
        params: None,
    };
    let resp = dispatch(req, &ctx).await;
    let result = resp.result.expect("should have result");
    let tools = result["tools"].as_array().expect("tools array");
    assert_eq!(tools.len(), 2);
    // sorted alphabetically
    assert_eq!(tools[0]["name"], "kei-sage::ask");
    assert_eq!(tools[1]["name"], "kei-task::search");
    assert_eq!(tools[1]["description"], "Search across tasks by query.");
    // input schema loaded as JSON value (not a string)
    let input_schema: &Value = &tools[1]["inputSchema"];
    assert_eq!(input_schema["type"], "object");
    assert_eq!(input_schema["properties"]["q"]["type"], "string");
}

#[tokio::test]
async fn tools_list_handles_empty_root() {
    let tmp = tempfile::tempdir().unwrap();
    let ctx = make_ctx(tmp.path().to_path_buf());
    let req = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        id: Some(json!(8)),
        method: "tools/list".into(),
        params: None,
    };
    let resp = dispatch(req, &ctx).await;
    let result = resp.result.expect("should have result");
    assert_eq!(result["tools"].as_array().unwrap().len(), 0);
}
