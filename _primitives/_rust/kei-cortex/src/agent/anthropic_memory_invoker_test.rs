//! Inline unit tests for `anthropic_memory_invoker.rs`.
//!
//! Constructor Pattern: extracted to a sibling so the parent stays
//! under the 200-LOC ceiling. Tests cover the pure helpers
//! (body-building, text extraction, error-reply prefix discipline);
//! HTTP behaviour is covered by integration tests that use wiremock.

use super::*;
use serde_json::json;

#[test]
fn build_body_emits_user_and_assistant_turns() {
    let snap = vec![
        Turn {
            role: "user".into(),
            content: "hi".into(),
        },
        Turn {
            role: "assistant".into(),
            content: "hello".into(),
        },
    ];
    let body = build_review_body("sys", &snap, "review-prompt");
    let msgs = body["messages"].as_array().unwrap();
    assert_eq!(msgs.len(), 3, "two snapshot turns + trailing prompt");
    assert_eq!(msgs[0]["role"], "user");
    assert_eq!(msgs[0]["content"], "hi");
    assert_eq!(msgs[1]["role"], "assistant");
    assert_eq!(msgs[2]["content"], "review-prompt");
}

#[test]
fn build_body_filters_unknown_roles() {
    let snap = vec![
        Turn {
            role: "user".into(),
            content: "hi".into(),
        },
        Turn {
            role: "tool".into(),
            content: "ignored".into(),
        },
    ];
    let body = build_review_body("sys", &snap, "p");
    let msgs = body["messages"].as_array().unwrap();
    assert_eq!(msgs.len(), 2, "only user + trailing prompt survive");
}

#[test]
fn extract_first_text_returns_inline_text() {
    let raw = json!({"content":[{"type":"text","text":"reply"}]});
    assert_eq!(extract_first_text(&raw), Some("reply".to_string()));
}

#[test]
fn extract_first_text_skips_non_text_blocks() {
    let raw = json!({"content":[
        {"type":"tool_use","id":"x"},
        {"type":"text","text":"ok"},
    ]});
    assert_eq!(extract_first_text(&raw), Some("ok".to_string()));
}

#[test]
fn extract_first_text_returns_none_on_empty() {
    let raw = json!({"content":[]});
    assert_eq!(extract_first_text(&raw), None);
}

#[test]
fn error_reply_round_trips() {
    let r = error_reply("boom");
    assert!(is_error_reply(&r));
    assert!(r.contains("boom"));
}

#[test]
fn normal_reply_is_not_error_reply() {
    assert!(!is_error_reply("Saved a fact."));
    assert!(!is_error_reply("Nothing to save."));
}

#[test]
fn invoker_smoke_returns_error_when_api_key_missing() {
    // Tokio runtime per-test — we call the async path directly.
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let prev = std::env::var("ANTHROPIC_API_KEY").ok();
    std::env::remove_var("ANTHROPIC_API_KEY");
    let inv = AnthropicMemoryInvoker::new("sys".into());
    let fut = inv.invoke(vec![], "p".into());
    let reply = rt.block_on(fut);
    if let Some(v) = prev {
        std::env::set_var("ANTHROPIC_API_KEY", v);
    }
    assert!(is_error_reply(&reply));
    assert!(reply.contains("ANTHROPIC_API_KEY"));
}
