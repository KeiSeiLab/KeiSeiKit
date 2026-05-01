//! Regression test (Hermes P3.1.b) — `resources/list` and `resources/read`
//! flow through `kei_skills::SkillRegistry`, proven on a tempdir corpus
//! that does NOT depend on the real `skills/` tree.
//!
//! Complements `skills_via_registry.rs` (which uses the repo corpus and
//! self-skips when absent). This test always runs because it builds the
//! corpus inline. Three SKILL.md files exercise the SSoT contract:
//!   1. `alpha` with a real description
//!   2. `beta` with a real description
//!   3. `_archive/zeta` (must be filtered by `loader::is_archived`)
//!
//! Asserts:
//!   - Exactly 2 resources surface (alpha + beta), archived skipped by
//!     `kei_skills::loader::is_archived`.
//!   - `description` is taken verbatim from frontmatter.
//!   - `resources/read` returns canonical SKILL.md (round-trip via
//!     `kei_skills::format::serialize`).
//!   - Unknown skill returns `INVALID_PARAMS` error envelope.

use kei_mcp::{dispatch, JsonRpcRequest, ServerContext};
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

fn make_request(method: &str, params: Value) -> JsonRpcRequest {
    JsonRpcRequest {
        jsonrpc: "2.0".into(),
        id: Some(json!(1)),
        method: method.into(),
        params: Some(params),
    }
}

fn write_skill(root: &Path, rel_dir: &str, name: &str, description: &str, body: &str) {
    let dir = root.join(rel_dir);
    fs::create_dir_all(&dir).unwrap();
    let content = format!("---\nname: {name}\ndescription: {description}\n---\n\n{body}");
    fs::write(dir.join("SKILL.md"), content).unwrap();
}

fn build_corpus_ctx() -> (tempfile::TempDir, ServerContext) {
    let tmp = tempfile::tempdir().unwrap();
    let skills = tmp.path().join("skills");
    let atoms = tmp.path().join("atoms");
    fs::create_dir_all(&atoms).unwrap();
    write_skill(&skills, "alpha", "alpha", "Alpha doc", "Alpha body.\n");
    write_skill(&skills, "beta", "beta", "Beta doc", "Beta body.\n");
    write_skill(&skills, "_archive/zeta", "zeta", "Archived", "Archived body.\n");
    let ctx = ServerContext::new(atoms, skills);
    (tmp, ctx)
}

fn names_in(result: &Value) -> Vec<String> {
    result["resources"]
        .as_array()
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter_map(|r| r["name"].as_str().map(String::from))
        .collect()
}

fn description_for(result: &Value, target: &str) -> String {
    result["resources"]
        .as_array()
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .find(|r| r["name"].as_str() == Some(target))
        .and_then(|r| r["description"].as_str().map(String::from))
        .unwrap_or_default()
}

#[tokio::test]
async fn list_returns_only_non_archived_skills_from_tempdir() {
    let (_tmp, ctx) = build_corpus_ctx();
    let req = make_request("resources/list", json!({}));
    let resp = dispatch(req, &ctx).await;
    let result = resp.result.expect("list should return result");
    let mut names = names_in(&result);
    names.sort();
    assert_eq!(names, vec!["alpha", "beta"], "archived must be filtered");
}

#[tokio::test]
async fn list_emits_skill_uri_and_text_markdown_mime() {
    let (_tmp, ctx) = build_corpus_ctx();
    let req = make_request("resources/list", json!({}));
    let resp = dispatch(req, &ctx).await;
    let result = resp.result.expect("result");
    let entries = result["resources"].as_array().expect("array");
    for e in entries {
        let name = e["name"].as_str().unwrap();
        assert_eq!(e["uri"], format!("skill://{name}"));
        assert_eq!(e["mimeType"], "text/markdown");
    }
}

#[tokio::test]
async fn list_preserves_frontmatter_description_verbatim() {
    let (_tmp, ctx) = build_corpus_ctx();
    let req = make_request("resources/list", json!({}));
    let resp = dispatch(req, &ctx).await;
    let result = resp.result.expect("result");
    assert_eq!(description_for(&result, "alpha"), "Alpha doc");
    assert_eq!(description_for(&result, "beta"), "Beta doc");
}

#[tokio::test]
async fn read_returns_canonical_skill_text() {
    let (_tmp, ctx) = build_corpus_ctx();
    let req = make_request("resources/read", json!({ "uri": "skill://alpha" }));
    let resp = dispatch(req, &ctx).await;
    let result = resp.result.expect("read result");
    let contents = result["contents"].as_array().expect("contents");
    assert_eq!(contents.len(), 1);
    let text = contents[0]["text"].as_str().expect("text");
    assert!(text.starts_with("---\n"), "must start with frontmatter");
    assert!(text.contains("name: alpha"));
    assert!(text.contains("description: Alpha doc"));
    assert!(text.contains("Alpha body."));
}

#[tokio::test]
async fn read_unknown_skill_returns_invalid_params() {
    let (_tmp, ctx) = build_corpus_ctx();
    let req = make_request("resources/read", json!({ "uri": "skill://nope" }));
    let resp = dispatch(req, &ctx).await;
    assert!(resp.result.is_none());
    let e = resp.error.expect("error");
    assert_eq!(e.code, -32602);
    assert!(e.message.contains("unknown skill"));
}
