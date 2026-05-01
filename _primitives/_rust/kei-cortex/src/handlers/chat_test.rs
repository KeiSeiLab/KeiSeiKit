//! Inline unit tests for `chat.rs`.
//!
//! Loop-termination paths are covered exhaustively in
//! `tool/tests/loop_terminates_on_max_turns.rs`. These tests focus on the
//! per-event SSE translation, body validation, and provider validation —
//! the surfaces unique to the handler layer.
//!
//! After Wave 40 split, the SSE event constructors and `loop_event_to_sse`
//! moved to sibling cubes (`chat_events.rs`, `chat_stream.rs`); the tests
//! reach them via the parent `super::chat_*::` paths.

use super::super::chat_events::{
    done_event, error_event, sentiment_event, token_event, tool_use_result_event,
    tool_use_start_event,
};
use super::super::chat_stream::loop_event_to_sse;
use super::*;
use crate::config::AppConfig;
use crate::state::AppState;
use std::path::PathBuf;
use std::sync::Arc;

fn dummy_state() -> AppState {
    let cfg = AppConfig::new(
        Some(9999),
        Some("https://example.test".into()),
        Some(PathBuf::from("/tmp/kc-tok")),
        Some(PathBuf::from("/tmp/kc-led")),
        Some(PathBuf::from("/tmp/kc-pets")),
        Some(PathBuf::from("/tmp/kc-mem.sqlite")),
        Some(PathBuf::from("/tmp/kc-live2d")),
    );
    AppState::with_router(cfg, "tok".into(), Arc::new(kei_router::LlmRouter::new()))
}

#[test]
fn empty_message_rejected() {
    let req = ChatRequest { message: String::new(), conversation_id: None };
    assert!(validate_body(&req).is_err());
}

#[test]
fn token_event_shape_builds() {
    drop(token_event("hi"));
}

#[test]
fn tool_use_start_event_shape_builds() {
    drop(tool_use_start_event("read", &serde_json::json!({"path": "/x"})));
}

#[test]
fn tool_use_result_event_shape_builds() {
    drop(tool_use_result_event("tu_1", false));
}

#[test]
fn error_event_shape_builds() {
    drop(error_event("upstream: boom"));
}

#[test]
fn sentiment_event_shape_builds() {
    drop(sentiment_event("I am happy"));
}

#[test]
fn done_event_shape_builds() {
    drop(done_event("abc-123"));
}

#[test]
fn unknown_provider_rejected_as_bad_request() {
    let st = dummy_state();
    let err = validate_provider(&st, "no-such-provider").unwrap_err();
    assert!(matches!(err, AppError::BadRequest(_)));
}

#[test]
fn pick_provider_query_wins_over_default() {
    let st = dummy_state();
    let picked = pick_provider_name(&st, Some("openai".into()));
    assert_eq!(picked, "openai");
}

#[test]
fn pick_provider_falls_back_to_config() {
    let st = dummy_state();
    let picked = pick_provider_name(&st, None);
    assert_eq!(picked, st.config().default_provider);
}

#[tokio::test]
async fn loop_event_assistant_text_emits_token() {
    let mut acc = String::new();
    let evs = loop_event_to_sse(
        crate::tool::LoopEvent::AssistantText("hi".into()),
        &mut acc,
        "c1",
    );
    assert_eq!(evs.len(), 1);
    assert_eq!(acc, "hi");
}

#[tokio::test]
async fn loop_event_done_emits_sentiment_then_done() {
    let mut acc = String::from("good");
    let evs = loop_event_to_sse(
        crate::tool::LoopEvent::Done {
            conversation_id: "c1".into(),
            turns: 1,
        },
        &mut acc,
        "c1",
    );
    assert_eq!(evs.len(), 2);
}

#[tokio::test]
async fn loop_event_tool_use_start_emits_one_frame() {
    let mut acc = String::new();
    let evs = loop_event_to_sse(
        crate::tool::LoopEvent::ToolUseStart {
            tool: "read".into(),
            input: serde_json::json!({"path": "/x"}),
        },
        &mut acc,
        "c1",
    );
    assert_eq!(evs.len(), 1);
}

#[tokio::test]
async fn loop_event_tool_result_emits_one_frame() {
    let mut acc = String::new();
    let evs = loop_event_to_sse(
        crate::tool::LoopEvent::ToolUseResult {
            tool_use_id: "tu_1".into(),
            is_error: false,
        },
        &mut acc,
        "c1",
    );
    assert_eq!(evs.len(), 1);
}

#[tokio::test]
async fn loop_event_error_emits_one_frame() {
    let mut acc = String::new();
    let evs = loop_event_to_sse(
        crate::tool::LoopEvent::Error("boom".into()),
        &mut acc,
        "c1",
    );
    assert_eq!(evs.len(), 1);
}
