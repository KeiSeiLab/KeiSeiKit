//! Integration tests — core CRUD (register_schema / emit / get / list / chain).
//!
//! Constructor Pattern: each test = one scenario, one assertion focus.
//! Companion file `validation.rs` tests schema-validation edge cases.

use kei_artifact::artifact::{chain, emit, get, list, register_schema, ArtifactFilter};
use kei_artifact::hash::artifact_id;
use kei_artifact::schemas::{register_builtins, BUILTIN};
use kei_artifact::Store;
use serde_json::json;

fn seed() -> Store {
    let s = Store::open_memory().unwrap();
    register_builtins(&s).unwrap();
    s
}

fn spec_bytes(goal: &str) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "goal": goal, "constraints": [], "invariants": []
    }))
    .unwrap()
}

#[test]
fn register_builtin_schemas_and_list_them() {
    let s = seed();
    let names = kei_artifact::artifact::list_schemas(&s).unwrap();
    for (n, _) in BUILTIN {
        assert!(names.iter().any(|x| x == n), "missing {n}");
    }
    assert_eq!(names.len(), 5);
}

#[test]
fn register_schema_custom_and_query_back() {
    let s = seed();
    let custom = r#"{"type":"object","additionalProperties":false,"properties":{}}"#;
    register_schema(&s, "custom", custom).unwrap();
    let names = kei_artifact::artifact::list_schemas(&s).unwrap();
    assert!(names.iter().any(|n| n == "custom"));
}

#[test]
fn emit_get_roundtrip_for_spec_schema() {
    let s = seed();
    let bytes = spec_bytes("ship v0.15");
    let id = emit(&s, "spec", "kei-architect", &bytes, None, None).unwrap();
    let got = get(&s, &id).unwrap().unwrap();
    assert_eq!(got.schema_name, "spec");
    assert_eq!(got.source_agent, "kei-architect");
    assert_eq!(got.content, bytes);
    assert_eq!(got.id, artifact_id("spec", &bytes));
}

#[test]
fn chain_walks_parent_handoff_up_the_graph() {
    let s = seed();
    let spec = spec_bytes("g");
    let spec_id = emit(&s, "spec", "kei-architect", &spec, None, None).unwrap();

    let plan = serde_json::to_vec(&json!({
        "goal": "g",
        "steps": [{"step": "s1", "verify": "v1"}]
    }))
    .unwrap();
    let plan_id = emit(&s, "plan", "kei-architect", &plan, None, Some(&spec_id)).unwrap();

    let patch = serde_json::to_vec(&json!({
        "summary": "first cut",
        "changes": [{"path": "a.rs", "op": "add", "summary": "new"}]
    }))
    .unwrap();
    let patch_id = emit(&s, "patch", "kei-code-implementer", &patch, None, Some(&plan_id)).unwrap();

    let walk = chain(&s, &patch_id).unwrap();
    assert_eq!(walk.len(), 3);
    assert_eq!(walk[0].id, spec_id);
    assert_eq!(walk[1].id, plan_id);
    assert_eq!(walk[2].id, patch_id);
}

#[test]
fn list_filters_by_schema_source_and_since() {
    let s = seed();
    let a = spec_bytes("a");
    let b = spec_bytes("b");
    emit(&s, "spec", "kei-architect", &a, None, None).unwrap();
    emit(&s, "spec", "kei-researcher", &b, None, None).unwrap();

    let by_source = list(&s, &ArtifactFilter {
        source_agent: Some("kei-architect".into()),
        ..Default::default()
    })
    .unwrap();
    assert_eq!(by_source.len(), 1);
    assert_eq!(by_source[0].source_agent, "kei-architect");

    let by_schema = list(&s, &ArtifactFilter {
        schema_name: Some("spec".into()),
        ..Default::default()
    })
    .unwrap();
    assert_eq!(by_schema.len(), 2);

    let none = list(&s, &ArtifactFilter {
        schema_name: Some("plan".into()),
        ..Default::default()
    })
    .unwrap();
    assert!(none.is_empty());
}

#[test]
fn duplicate_emit_is_idempotent_same_id() {
    let s = seed();
    let content = spec_bytes("g");
    let id1 = emit(&s, "spec", "kei-architect", &content, None, None).unwrap();
    let id2 = emit(&s, "spec", "kei-architect", &content, None, None).unwrap();
    assert_eq!(id1, id2);
    let all = list(&s, &ArtifactFilter::default()).unwrap();
    assert_eq!(all.len(), 1);
}

#[test]
fn missing_parent_rejected() {
    let s = seed();
    let content = spec_bytes("g");
    let err = emit(&s, "spec", "kei-architect", &content, None, Some("deadbeef")).unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("parent"), "unexpected: {msg}");
}
