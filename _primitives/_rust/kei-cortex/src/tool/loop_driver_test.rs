//! Inline unit tests for `loop_driver.rs`. Extracted to a sibling so
//! the parent stays under the 200-LOC Constructor Pattern ceiling after
//! the Wave 44c CancellationToken refactor (F-HIGH-5).

use super::*;

#[test]
fn collect_tool_uses_filters() {
    let blocks = vec![
        ContentBlock::Text("hi".into()),
        ContentBlock::ToolUse(ToolCall {
            id: "tu1".into(),
            name: "read".into(),
            input: serde_json::json!({}),
        }),
    ];
    assert_eq!(collect_tool_uses(&blocks).len(), 1);
}

/// F-HIGH-5: cancel token fires mid-turn and the loop short-circuits
/// without waiting for the in-flight invoker to complete.
#[tokio::test]
async fn cancel_during_long_invoker_short_circuits() {
    use crate::tool::registry::ToolRegistry;
    use futures::StreamExt;
    use std::sync::Arc;

    // Invoker that "runs" for 60 seconds — simulates a slow agent call.
    let slow_invoker: ModelInvoker = Arc::new(|_msgs, _tools| {
        Box::pin(async move {
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
            Ok(ModelTurn {
                content: vec![ContentBlock::Text("never".into())],
                stop_reason: "end_turn".into(),
                usage: None,
            })
        })
    });
    let registry = Arc::new(ToolRegistry::empty(std::path::PathBuf::from(".")));
    let cancel = CancellationToken::new();
    let cancel_clone = cancel.clone();
    // Cancel after 50ms — well before the 60s slow invoker would finish.
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        cancel_clone.cancel();
    });
    let s = run_with_tools(slow_invoker, registry, vec![], "go".into(), "c-cancel".into(), cancel);
    let start = std::time::Instant::now();
    let events: Vec<_> = s.collect().await;
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "loop must abort within seconds of cancel, took {elapsed:?}"
    );
    assert!(events
        .iter()
        .any(|e| matches!(e, LoopEvent::Error(m) if m == "cancelled")));
    assert!(matches!(events.last(), Some(LoopEvent::Done { .. })));
}
