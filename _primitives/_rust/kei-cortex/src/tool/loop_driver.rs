//! Agentic loop — orchestrates model ↔ tool turns until termination.
//!
//! Each turn: invoke model → emit text events → dispatch tool calls →
//! loop. Terminates on `stop_reason != "tool_use"`, on `MAX_TURNS`, or
//! when the cancel token fires.
//!
//! Wave 44c (2026-04-24, F-HIGH-5): cancel moved from
//! `oneshot::Receiver<()>` to `CancellationToken` and is `select!`'d
//! against the in-flight invoker so long-running tool turns cancel
//! within milliseconds, not after the turn completes.

use super::dispatch::dispatch_outcome;
use super::registry::ToolRegistry;
use super::types::{ToolCall, ToolResult};
use async_stream::stream;
use futures::future::BoxFuture;
use futures::stream::Stream;
use serde_json::Value;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

/// Hard cap on turns. Past this we abort with an error event so the user
/// is not silently billed for an infinite loop.
pub const MAX_TURNS: usize = 25;

/// One block in a model response.
#[derive(Debug, Clone)]
pub enum ContentBlock {
    Text(String),
    ToolUse(ToolCall),
}

/// Token usage reported by the provider alongside the model response.
/// Used by the chat handler to compute `cost_cents` for kei-ledger.
/// Wave 40 (2026-04-24).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TokenUsage {
    pub input_tokens: u64,
    pub output_tokens: u64,
}

/// One model turn (its content + the reason it stopped).
#[derive(Debug, Clone)]
pub struct ModelTurn {
    pub content: Vec<ContentBlock>,
    pub stop_reason: String,
    /// Tokens reported by the provider. `None` when the provider didn't
    /// surface a usage block (e.g. fakes in tests). Cost recording falls
    /// back to writing a zero-cents row when absent.
    pub usage: Option<TokenUsage>,
}

/// Boxed async invoker. The orchestrator wires the real Anthropic call;
/// tests inject a closure.
pub type ModelInvoker = Arc<
    dyn Fn(Vec<ConversationMessage>, Vec<Value>)
            -> BoxFuture<'static, Result<ModelTurn, String>>
        + Send
        + Sync,
>;

/// One entry in our local conversation. `Tool` rows carry the `tool_result`
/// payload that goes inside a `user` content block on the next API call.
#[derive(Debug, Clone)]
pub enum ConversationMessage {
    User(String),
    Assistant(Vec<ContentBlock>),
    Tool(Vec<ToolResult>),
}

/// Events streamed to the SSE client.
#[derive(Debug, Clone)]
pub enum LoopEvent {
    AssistantText(String),
    ToolUseStart { tool: String, input: Value },
    ToolUseResult { tool_use_id: String, is_error: bool },
    Error(String),
    Done { conversation_id: String, turns: usize },
}

/// One turn's outcome. Visible to the `dispatch` cube.
pub(crate) enum TurnOutcome {
    /// Text-only; loop terminates after emitting it.
    Final(Vec<String>),
    /// Tool calls present; loop dispatches and continues.
    Continue { texts: Vec<String>, calls: Vec<ToolCall> },
    /// Invoker errored; loop terminates.
    InvokerError(String),
}


pub fn run_with_tools(
    invoker: ModelInvoker,
    registry: Arc<ToolRegistry>,
    tool_defs: Vec<Value>,
    initial_user_message: String,
    conversation_id: String,
    cancel: CancellationToken,
) -> impl Stream<Item = LoopEvent> + Send + 'static {
    inner_loop(LoopState {
        invoker,
        registry,
        tool_defs,
        messages: vec![ConversationMessage::User(initial_user_message)],
        conversation_id,
        cancel,
    })
}

/// Bundle of state captured by the loop coroutine. Exists only to keep
/// `run_with_tools` ≤30 LOC and to pass arguments by struct rather than
/// six-tuple positional.
struct LoopState {
    invoker: ModelInvoker,
    registry: Arc<ToolRegistry>,
    tool_defs: Vec<Value>,
    messages: Vec<ConversationMessage>,
    conversation_id: String,
    cancel: CancellationToken,
}

/// Drives the turn loop, emits per-turn events, terminates on
/// max-turns / cancel / clean stop_reason. F-HIGH-5: the cancel token
/// is `select!`'d against the invoker future so a long-running model
/// call (60s+ bash / 120s+ agent) cancels promptly.
fn inner_loop(mut s: LoopState) -> impl Stream<Item = LoopEvent> + Send + 'static {
    stream! {
        let conv = s.conversation_id.clone();
        for turn in 0..MAX_TURNS {
            let outcome = tokio::select! {
                biased;
                _ = s.cancel.cancelled() => {
                    yield LoopEvent::Error("cancelled".into());
                    yield LoopEvent::Done { conversation_id: conv.clone(), turns: turn };
                    return;
                }
                o = invoke_one_turn(&s.invoker, &mut s.messages, &s.tool_defs) => o,
            };
            let mut tool_results: Option<Vec<ToolResult>> = None;
            let events = dispatch_outcome(outcome, &s.registry, turn, &conv, &mut tool_results).await;
            for ev in events {
                let stop = matches!(ev, LoopEvent::Done { .. });
                yield ev;
                if stop { return; }
            }
            if let Some(r) = tool_results {
                s.messages.push(ConversationMessage::Tool(r));
            }
        }
        yield LoopEvent::Error(format!("max turns ({MAX_TURNS}) reached"));
        yield LoopEvent::Done { conversation_id: conv, turns: MAX_TURNS };
    }
}

/// Invoke the model and classify the response into a `TurnOutcome`.
async fn invoke_one_turn(
    invoker: &ModelInvoker,
    messages: &mut Vec<ConversationMessage>,
    tool_defs: &[Value],
) -> TurnOutcome {
    let turn_data = match invoker(messages.clone(), tool_defs.to_vec()).await {
        Ok(t) => t,
        Err(e) => return TurnOutcome::InvokerError(e),
    };
    let texts: Vec<String> = turn_data
        .content
        .iter()
        .filter_map(|b| match b {
            ContentBlock::Text(t) => Some(t.clone()),
            _ => None,
        })
        .collect();
    messages.push(ConversationMessage::Assistant(turn_data.content.clone()));
    let calls = collect_tool_uses(&turn_data.content);
    if calls.is_empty() || turn_data.stop_reason != "tool_use" {
        TurnOutcome::Final(texts)
    } else {
        TurnOutcome::Continue { texts, calls }
    }
}

/// Pull `ToolUse` blocks out of a content list, cloning so the caller
/// can keep the original around for transcript purposes.
fn collect_tool_uses(content: &[ContentBlock]) -> Vec<ToolCall> {
    content
        .iter()
        .filter_map(|b| match b {
            ContentBlock::ToolUse(c) => Some(c.clone()),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
#[path = "loop_driver_test.rs"]
mod tests;
