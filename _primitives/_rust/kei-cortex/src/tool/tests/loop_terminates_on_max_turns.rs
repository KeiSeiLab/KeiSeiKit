//! Validates the loop's hard termination guarantees:
//! - infinite tool-use stream stops at MAX_TURNS with an Error event
//! - cancellation via CancellationToken causes early Done
//! - clean end_turn stop_reason terminates immediately
//!
//! Wave 44c (F-HIGH-5): cancel migrated from `oneshot::Receiver<()>`
//! to `tokio_util::sync::CancellationToken`.

use crate::tool::loop_driver::{
    run_with_tools, ContentBlock, ConversationMessage, LoopEvent, ModelInvoker, ModelTurn,
    MAX_TURNS,
};
use crate::tool::registry::ToolRegistry;
use crate::tool::types::ToolCall;
use futures::StreamExt;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

/// Build an invoker that ALWAYS asks for one tool_use, simulating a
/// runaway agent. Each turn echoes a fresh `tu_<n>` id.
fn always_tool_use() -> ModelInvoker {
    let counter = Arc::new(tokio::sync::Mutex::new(0u32));
    Arc::new(move |_msgs, _tools| {
        let counter = counter.clone();
        Box::pin(async move {
            let mut c = counter.lock().await;
            *c += 1;
            let id = format!("tu_{}", *c);
            Ok(ModelTurn {
                content: vec![ContentBlock::ToolUse(ToolCall {
                    id,
                    name: "noop".into(),
                    input: serde_json::json!({}),
                })],
                stop_reason: "tool_use".into(),
                usage: None,
            })
        })
    })
}

/// A registry where `noop` always succeeds. Keeps the loop fed.
fn registry_with_noop() -> Arc<ToolRegistry> {
    let mut r = ToolRegistry::empty(std::path::PathBuf::from("."));
    r.register(
        "noop",
        Box::new(|_v| Box::pin(async move { Ok("ok".into()) })),
    );
    Arc::new(r)
}

#[tokio::test]
async fn loop_aborts_at_max_turns() {
    let invoker = always_tool_use();
    let registry = registry_with_noop();
    let cancel = CancellationToken::new();
    let s = run_with_tools(invoker, registry, vec![], "go".into(), "c1".into(), cancel);
    let events: Vec<_> = s.collect().await;
    let last = events.last().expect("at least one event");
    match last {
        LoopEvent::Done { turns, .. } => assert_eq!(*turns, MAX_TURNS),
        other => panic!("expected Done, got {other:?}"),
    }
    let saw_max_turns_error = events
        .iter()
        .any(|e| matches!(e, LoopEvent::Error(m) if m.contains("max turns")));
    assert!(saw_max_turns_error, "should emit max-turns error");
}

#[tokio::test]
async fn cancel_signal_short_circuits() {
    let invoker = always_tool_use();
    let registry = registry_with_noop();
    let cancel = CancellationToken::new();
    // Pre-fire the cancel before the stream is consumed.
    cancel.cancel();
    let s = run_with_tools(invoker, registry, vec![], "go".into(), "c2".into(), cancel);
    let events: Vec<_> = s.collect().await;
    assert!(events
        .iter()
        .any(|e| matches!(e, LoopEvent::Error(m) if m == "cancelled")));
    assert!(matches!(events.last(), Some(LoopEvent::Done { .. })));
}

#[tokio::test]
async fn clean_end_turn_terminates_immediately() {
    let invoker: ModelInvoker = Arc::new(|_msgs, _tools| {
        Box::pin(async {
            Ok(ModelTurn {
                content: vec![ContentBlock::Text("done".into())],
                stop_reason: "end_turn".into(),
                usage: None,
            })
        })
    });
    let registry = Arc::new(ToolRegistry::empty(std::path::PathBuf::from(".")));
    let cancel = CancellationToken::new();
    let s = run_with_tools(invoker, registry, vec![], "go".into(), "c3".into(), cancel);
    let events: Vec<_> = s.collect().await;
    if let Some(LoopEvent::Done { turns, .. }) = events.last() {
        assert_eq!(*turns, 1, "single clean turn should report 1");
    } else {
        panic!("missing Done");
    }
}

#[tokio::test]
async fn empty_history_passed_to_invoker() {
    let captured = Arc::new(tokio::sync::Mutex::new(Vec::<ConversationMessage>::new()));
    let cap = captured.clone();
    let invoker: ModelInvoker = Arc::new(move |msgs, _tools| {
        let cap = cap.clone();
        Box::pin(async move {
            *cap.lock().await = msgs;
            Ok(ModelTurn {
                content: vec![ContentBlock::Text("k".into())],
                stop_reason: "end_turn".into(),
                usage: None,
            })
        })
    });
    let registry = Arc::new(ToolRegistry::empty(std::path::PathBuf::from(".")));
    let cancel = CancellationToken::new();
    let s = run_with_tools(
        invoker,
        registry,
        vec![],
        "first".into(),
        "c4".into(),
        cancel,
    );
    let _: Vec<_> = s.collect().await;
    let msgs = captured.lock().await;
    assert_eq!(msgs.len(), 1);
    assert!(matches!(&msgs[0], ConversationMessage::User(s) if s == "first"));
}
