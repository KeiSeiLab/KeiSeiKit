//! Per-turn outcome dispatcher.
//!
//! `dispatch_outcome` translates a `TurnOutcome` (model said: text-only,
//! tool-use, or invoker-error) into a flat `Vec<LoopEvent>` the outer
//! `inner_loop` can yield in order. This file is split out from
//! `loop_driver.rs` so each cube stays inside the Constructor Pattern
//! 200-LOC file ceiling.

use super::loop_driver::{LoopEvent, TurnOutcome};
use super::registry::ToolRegistry;
use super::types::{ToolCall, ToolResult};
use std::sync::Arc;

/// Translate one `TurnOutcome` into a flat sequence of events. When the
/// outcome is `Continue`, also stash the produced tool results into
/// `*tool_results_out` so the outer loop can append them to history.
pub(crate) async fn dispatch_outcome(
    outcome: TurnOutcome,
    registry: &Arc<ToolRegistry>,
    turn: usize,
    conversation_id: &str,
    tool_results_out: &mut Option<Vec<ToolResult>>,
) -> Vec<LoopEvent> {
    let conv = conversation_id.to_string();
    match outcome {
        TurnOutcome::InvokerError(e) => vec![
            LoopEvent::Error(format!("model: {e}")),
            LoopEvent::Done {
                conversation_id: conv,
                turns: turn,
            },
        ],
        TurnOutcome::Final(texts) => final_events(texts, conv, turn + 1),
        TurnOutcome::Continue { texts, calls } => {
            continue_events(texts, calls, registry, tool_results_out).await
        }
    }
}

/// Build the event list for a `Final` outcome.
fn final_events(texts: Vec<String>, conv: String, turns: usize) -> Vec<LoopEvent> {
    let mut out: Vec<LoopEvent> = texts.into_iter().map(LoopEvent::AssistantText).collect();
    out.push(LoopEvent::Done {
        conversation_id: conv,
        turns,
    });
    out
}

/// Build the event list for a `Continue` outcome and stash tool results
/// into `tool_results_out` for the outer loop.
async fn continue_events(
    texts: Vec<String>,
    calls: Vec<ToolCall>,
    registry: &Arc<ToolRegistry>,
    tool_results_out: &mut Option<Vec<ToolResult>>,
) -> Vec<LoopEvent> {
    let mut out: Vec<LoopEvent> = texts.into_iter().map(LoopEvent::AssistantText).collect();
    let mut results = Vec::with_capacity(calls.len());
    for call in calls {
        out.push(LoopEvent::ToolUseStart {
            tool: call.name.clone(),
            input: call.input.clone(),
        });
        let res = registry.dispatch(call).await;
        out.push(LoopEvent::ToolUseResult {
            tool_use_id: res.tool_use_id.clone(),
            is_error: res.is_error,
        });
        results.push(res);
    }
    *tool_results_out = Some(results);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tool::loop_driver::{ContentBlock, TurnOutcome};

    #[tokio::test]
    async fn final_outcome_emits_text_and_done() {
        let registry = Arc::new(ToolRegistry::empty(std::path::PathBuf::from(".")));
        let mut sink: Option<Vec<ToolResult>> = None;
        let events = dispatch_outcome(
            TurnOutcome::Final(vec!["a".into(), "b".into()]),
            &registry,
            0,
            "c1",
            &mut sink,
        )
        .await;
        assert_eq!(events.len(), 3);
        assert!(matches!(events.last(), Some(LoopEvent::Done { turns, .. }) if *turns == 1));
        assert!(sink.is_none());
    }

    #[tokio::test]
    async fn invoker_error_emits_error_and_done() {
        let registry = Arc::new(ToolRegistry::empty(std::path::PathBuf::from(".")));
        let mut sink: Option<Vec<ToolResult>> = None;
        let events = dispatch_outcome(
            TurnOutcome::InvokerError("boom".into()),
            &registry,
            3,
            "c2",
            &mut sink,
        )
        .await;
        assert_eq!(events.len(), 2);
        assert!(matches!(&events[0], LoopEvent::Error(m) if m.contains("boom")));
    }

    #[test]
    fn content_block_text_pattern_match() {
        let b = ContentBlock::Text("hi".into());
        assert!(matches!(b, ContentBlock::Text(_)));
    }
}
