//! kei-router integration tests — mirror LBM router_test.go semantics.

use kei_router::{DynRule, Method, Router};

#[test]
fn exact_match_search_knowledge() {
    let r = Router::new();
    let out = r.route("search knowledge base for rust async");
    assert_eq!(out.tool, "search_knowledge");
    assert_eq!(out.method, Method::Keyword);
    assert!(out.confidence > 0.7);
}

#[test]
fn fuzzy_match_find_importers_with_path() {
    let r = Router::new();
    let out = r.route("who imports /src/router.rs");
    assert_eq!(out.tool, "find_importers");
    assert_eq!(
        out.params.get("path").and_then(|v| v.as_str()),
        Some("/src/router.rs")
    );
}

#[test]
fn no_match_fallback_knowledge() {
    let r = Router::new();
    let out = r.route("hello this is not a routed query");
    assert_eq!(out.tool, "search_knowledge");
    assert_eq!(out.method, Method::Fallback);
    assert!(out.confidence < 0.3);
}

#[test]
fn no_match_fallback_code_with_path() {
    let r = Router::new();
    let out = r.route("what happened in /tmp/mystery.rs");
    assert_eq!(out.tool, "search_code");
    assert_eq!(out.method, Method::Fallback);
}

#[test]
fn confidence_ranking_keyword_above_fallback() {
    let r = Router::new();
    let kw = r.route("knowledge stats please");
    let fb = r.route("asdf zxcv qwer");
    assert!(kw.confidence > fb.confidence);
}

#[test]
fn dynamic_rule_addition() {
    let mut r = Router::new();
    r.add_dynamic(vec![DynRule {
        tool: "custom_tool".into(),
        keywords: vec!["magic-keyword".into()],
    }]);
    let out = r.route("please run magic-keyword now");
    assert_eq!(out.tool, "custom_tool");
    assert_eq!(out.method, Method::Keyword);
}

#[test]
fn remote_mcp_forward_hint() {
    let r = Router::new();
    let out = r.route_with_hint("completely novel utterance xyz");
    assert_eq!(out.method, Method::Fallback);
    assert_eq!(out.params.get("_forward"), Some(&serde_json::json!(true)));
}

#[test]
fn id_extraction_for_get_task() {
    let r = Router::new();
    let out = r.route("get task id=42");
    assert_eq!(out.tool, "get_task");
    assert_eq!(out.params.get("id").and_then(|v| v.as_i64()), Some(42));
}
