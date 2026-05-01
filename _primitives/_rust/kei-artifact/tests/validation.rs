//! Integration tests — schema validation edge cases.
//!
//! Constructor Pattern: one scenario per test, one assertion focus.

use kei_artifact::artifact::{emit, get, register_schema, validate_by_id};
use kei_artifact::schemas::register_builtins;
use kei_artifact::Store;
use serde_json::json;

fn seed() -> Store {
    let s = Store::open_memory().unwrap();
    register_builtins(&s).unwrap();
    s
}

#[test]
fn invalid_content_rejected_at_emit_time() {
    let s = seed();
    // spec requires goal, constraints, invariants — give only goal.
    let bad = serde_json::to_vec(&json!({"goal": "x"})).unwrap();
    let err = emit(&s, "spec", "kei-architect", &bad, None, None).unwrap_err();
    let msg = format!("{err:#}");
    assert!(
        msg.contains("constraints") || msg.contains("invariants"),
        "unexpected error: {msg}"
    );
}

#[test]
fn validate_by_id_passes_for_conforming_content() {
    let s = seed();
    let content = serde_json::to_vec(&json!({
        "summary": "ok",
        "changes": [{"path": "x", "op": "mod", "summary": "tiny"}]
    }))
    .unwrap();
    let id = emit(&s, "patch", "kei-code-implementer", &content, None, None).unwrap();
    assert!(validate_by_id(&s, &id).is_ok());
}

#[test]
fn validate_by_id_detects_drift_after_schema_override() {
    let s = seed();
    let content = serde_json::to_vec(&json!({
        "summary": "ok",
        "changes": [{"path": "x", "op": "mod", "summary": "t"}]
    }))
    .unwrap();
    let id = emit(&s, "patch", "kei-code-implementer", &content, None, None).unwrap();
    let stricter = r#"{
        "type": "object",
        "additionalProperties": false,
        "required": ["summary", "changes", "must_have"],
        "properties": {
            "summary": {"type": "string"},
            "changes": {"type": "array"},
            "must_have": {"type": "string"}
        }
    }"#;
    register_schema(&s, "patch", stricter).unwrap();
    let err = validate_by_id(&s, &id).unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("must_have"), "unexpected: {msg}");
}

#[test]
fn unknown_schema_name_rejected_at_emit() {
    let s = seed();
    let err = emit(&s, "not-a-real-schema", "x", b"{}", None, None).unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("unknown schema"), "unexpected: {msg}");
}

#[test]
fn review_schema_accepts_canonical_critic_output() {
    let s = seed();
    let content = serde_json::to_vec(&json!({
        "reviewer": "kei-critic",
        "findings": [
            {
                "severity": "high",
                "category": "bug",
                "title": "off-by-one",
                "file": "src/x.rs",
                "line": 42
            }
        ],
        "verdict": "request_changes"
    }))
    .unwrap();
    let id = emit(&s, "review", "kei-critic", &content, None, None).unwrap();
    let back = get(&s, &id).unwrap().unwrap();
    assert_eq!(back.schema_name, "review");
}

#[test]
fn research_schema_accepts_claims_with_evidence_grade() {
    let s = seed();
    let content = serde_json::to_vec(&json!({
        "question": "Does Rust's cargo support offline builds?",
        "claims": [
            {
                "claim": "cargo --offline works with pre-cached deps",
                "evidence_grade": "E1",
                "confidence": "100",
                "sources": [{"url": "https://doc.rust-lang.org/cargo/", "verified": true}]
            }
        ]
    }))
    .unwrap();
    let id = emit(&s, "research", "kei-researcher", &content, None, None).unwrap();
    assert!(validate_by_id(&s, &id).is_ok());
}

#[test]
fn plan_schema_rejects_empty_steps() {
    let s = seed();
    let bad = serde_json::to_vec(&json!({
        "goal": "g",
        "steps": []
    }))
    .unwrap();
    let err = emit(&s, "plan", "kei-architect", &bad, None, None).unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("array"), "unexpected: {msg}");
}

#[test]
fn patch_schema_rejects_invalid_op_enum() {
    let s = seed();
    let bad = serde_json::to_vec(&json!({
        "summary": "ok",
        "changes": [{"path": "x", "op": "RENAME", "summary": "t"}]
    }))
    .unwrap();
    let err = emit(&s, "patch", "kei-code-implementer", &bad, None, None).unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("enum"), "unexpected: {msg}");
}

#[test]
fn review_schema_rejects_empty_findings() {
    // v0.15.1 HIGH fix: review artifacts must list ≥ 1 finding so a `reject`
    // or `request_changes` verdict cannot ship with nothing to point at.
    let s = seed();
    let bad = serde_json::to_vec(&json!({
        "reviewer": "kei-critic",
        "findings": [],
        "verdict": "reject"
    }))
    .unwrap();
    let err = emit(&s, "review", "kei-critic", &bad, None, None).unwrap_err();
    let msg = format!("{err:#}");
    assert!(
        msg.contains("array") || msg.contains("min"),
        "unexpected: {msg}"
    );
}
