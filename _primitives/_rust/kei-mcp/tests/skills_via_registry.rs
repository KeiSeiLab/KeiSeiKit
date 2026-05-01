//! Integration test — `resources/list` and `resources/read` flow through
//! the `kei-skills` `SkillRegistry` (Phase 3.1 SSoT).
//!
//! Walks ancestors of `CARGO_MANIFEST_DIR` to find the repo root's
//! `skills/` directory (KeiSeiKit corpus, ~45 SKILL.md files at time of
//! writing). Skips the test if the dir cannot be located — keeps the
//! suite green on isolated checkouts that don't carry the skills tree.
//!
//! Asserts:
//!   1. `resources/list` returns ≥ 30 entries (corpus headroom over 45).
//!   2. Three known-valid skill names — `research`, `refactor`, `onboard`
//!      — are reachable both from the list and via `resources/read`.
//!   3. `resources/read` for a bogus skill returns an error envelope.
//!   4. `resources/list` URIs match `skill://<name>` shape and carry a
//!      non-empty `description`.

use kei_mcp::{dispatch, JsonRpcRequest, ServerContext};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

fn make_request(method: &str, params: Value) -> JsonRpcRequest {
    JsonRpcRequest {
        jsonrpc: "2.0".into(),
        id: Some(json!(1)),
        method: method.into(),
        params: Some(params),
    }
}

/// Locate `skills/` by walking up from `CARGO_MANIFEST_DIR`. Returns
/// `None` if no ancestor directory contains a `skills/` subdir.
fn find_skills_root() -> Option<PathBuf> {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut cursor: &Path = &manifest;
    loop {
        let candidate = cursor.join("skills");
        if candidate.is_dir() {
            return Some(candidate);
        }
        match cursor.parent() {
            Some(p) => cursor = p,
            None => return None,
        }
    }
}

fn ctx_for_corpus() -> Option<ServerContext> {
    let skills_root = find_skills_root()?;
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let atoms_root = manifest
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or(manifest);
    Some(ServerContext::new(atoms_root, skills_root))
}

fn names_in_list_response(result: &Value) -> Vec<String> {
    result["resources"]
        .as_array()
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter_map(|r| r["name"].as_str().map(String::from))
        .collect()
}

#[tokio::test]
async fn list_returns_at_least_thirty_skills_from_repo_corpus() {
    let Some(ctx) = ctx_for_corpus() else {
        eprintln!("skills/ not found in ancestors — skipping corpus test");
        return;
    };
    let req = make_request("resources/list", json!({}));
    let resp = dispatch(req, &ctx).await;
    let result = resp.result.expect("resources/list should return result");
    let names = names_in_list_response(&result);
    assert!(
        names.len() >= 30,
        "expected ≥30 skills in corpus, found {}: {names:?}",
        names.len()
    );
    let entries = result["resources"]
        .as_array()
        .expect("resources is an array");
    for entry in entries {
        let uri = entry["uri"].as_str().expect("uri string");
        let name = entry["name"].as_str().expect("name string");
        assert_eq!(
            uri,
            format!("skill://{name}"),
            "uri must match skill://<name>"
        );
        assert_eq!(entry["mimeType"], "text/markdown");
        let desc = entry["description"].as_str().unwrap_or("");
        assert!(!desc.is_empty(), "description must be non-empty for {name}");
    }
}

#[tokio::test]
async fn known_skills_are_findable_in_list() {
    let Some(ctx) = ctx_for_corpus() else {
        return;
    };
    let req = make_request("resources/list", json!({}));
    let resp = dispatch(req, &ctx).await;
    let result = resp.result.expect("result");
    let names = names_in_list_response(&result);
    for expected in ["research", "refactor", "onboard"] {
        assert!(
            names.iter().any(|n| n == expected),
            "expected {expected} in list, got {names:?}"
        );
    }
}

#[tokio::test]
async fn read_known_skill_returns_serialized_text() {
    let Some(ctx) = ctx_for_corpus() else {
        return;
    };
    for name in ["research", "refactor", "onboard"] {
        let req = make_request(
            "resources/read",
            json!({ "uri": format!("skill://{name}") }),
        );
        let resp = dispatch(req, &ctx).await;
        let result = resp
            .result
            .unwrap_or_else(|| panic!("resources/read({name}) should return result"));
        let contents = result["contents"]
            .as_array()
            .unwrap_or_else(|| panic!("contents array for {name}"));
        assert_eq!(contents.len(), 1, "exactly one content entry for {name}");
        let text = contents[0]["text"].as_str().expect("text string");
        assert!(text.starts_with("---\n"), "{name}: must start with frontmatter fence");
        assert!(
            text.contains(&format!("name: {name}")),
            "{name}: serialized text must contain its name field"
        );
    }
}

#[tokio::test]
async fn read_unknown_skill_returns_error_envelope() {
    let Some(ctx) = ctx_for_corpus() else {
        return;
    };
    let req = make_request(
        "resources/read",
        json!({ "uri": "skill://this-skill-does-not-exist-xyzzy" }),
    );
    let resp = dispatch(req, &ctx).await;
    assert!(resp.result.is_none(), "unknown skill must not produce result");
    let e = resp.error.expect("error envelope");
    assert_eq!(e.code, -32602, "unknown skill maps to INVALID_PARAMS");
    assert!(
        e.message.contains("unknown skill"),
        "error message should mention unknown skill, got: {}",
        e.message
    );
}

#[tokio::test]
async fn read_with_non_skill_uri_returns_error() {
    let Some(ctx) = ctx_for_corpus() else {
        return;
    };
    let req = make_request(
        "resources/read",
        json!({ "uri": "file:///etc/passwd" }),
    );
    let resp = dispatch(req, &ctx).await;
    assert!(resp.result.is_none());
    let e = resp.error.expect("error envelope");
    assert_eq!(e.code, -32602);
    assert!(e.message.contains("not a skill uri"));
}
