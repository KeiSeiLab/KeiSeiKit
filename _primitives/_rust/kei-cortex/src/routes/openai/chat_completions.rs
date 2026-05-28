//! POST /v1/chat/completions handler. Stateless by default; opt-in
//! continuity via `X-Kei-Session-Id`. Streaming delegates per-event
//! SSE translation to `stream_forwarder::forward_chat_completions`.

use super::agent_runner;
use super::chat_helpers::{
    build_completion_response, collect_session_messages, persist_turn, session_id_from_headers,
    unix_secs, validate,
};
use super::error::OpenAiError;
use super::ids::chat_completion_id;
use super::session::{self, SessionStore};
use super::stream_forwarder;
use super::translation::{build_assistant_message, filter_supported_tools, flatten_user_prompt};
use super::types::{ChatCompletionRequest, ChatCompletionResponse, ChatMessage};
use crate::state::AppState;
use crate::tool;
use tracing::debug;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum::Json;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

const DEFAULT_SYSTEM: &str =
    "You are kei-cortex, a helpful assistant. Use <think> for reasoning and tool calls when appropriate.";

/// Handler entry point — dispatches to `handle_stream` or `handle_sync`.
pub async fn chat_completions(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<ChatCompletionRequest>,
) -> Result<Response, OpenAiError> {
    let store = session::global();
    validate(&req)?;
    let session_id = session_id_from_headers(&headers);
    let kept_tools = match req.tools.as_ref() {
        Some(tools) => {
            let (kept, dropped) = filter_supported_tools(tools);
            if !dropped.is_empty() {
                debug!("openai: dropped unsupported tools: {dropped:?}");
            }
            kept
        }
        None => Vec::new(),
    };
    if req.stream {
        Ok(handle_stream(state, store, session_id, req, kept_tools)
            .await?
            .into_response())
    } else {
        Ok(handle_sync(state, store, session_id, req, kept_tools)
            .await?
            .into_response())
    }
}

fn system_prompt_from_req(req: &ChatCompletionRequest) -> String {
    req.messages
        .iter()
        .find(|m| m.role == "system")
        .and_then(|m| m.content.clone())
        .unwrap_or_else(|| DEFAULT_SYSTEM.to_string())
}

async fn handle_sync(
    state: AppState,
    store: SessionStore,
    session_id: Option<String>,
    req: ChatCompletionRequest,
    kept_tools: Vec<super::types::OpenAiTool>,
) -> Result<Json<ChatCompletionResponse>, OpenAiError> {
    let messages = collect_session_messages(&store, &session_id, &req);
    let prompt = flatten_user_prompt(&messages);
    let system = system_prompt_from_req(&req);
    let comp_id = super::ids::chat_completion_id();
    let agent_id = format!("openai-chat-{}", comp_id);
    let (reply_text, _tool_calls, usage) = agent_runner::collect_reply(
        &state,
        system,
        prompt,
        kept_tools,
        "chat-completions",
        agent_id,
        session_id.clone(),
    )
    .await?;
    let assistant = build_assistant_message(reply_text, Vec::new());
    persist_turn(&store, &session_id, &req.messages, &assistant);
    Ok(Json(build_completion_response(req.model, assistant, usage)))
}

/// Streaming path — forward each `LoopEvent` as its own SSE frame
/// via `stream_forwarder`; a tee task persists on `Done`. Phase 2:
/// `stream_events_with_tracking` fires a fire-and-forget token-event
/// after the loop forwarder closes so sleep-report sees stream calls
/// alongside sync ones.
async fn handle_stream(
    state: AppState,
    store: SessionStore,
    session_id: Option<String>,
    req: ChatCompletionRequest,
    _kept_tools: Vec<super::types::OpenAiTool>,
) -> Result<Response, OpenAiError> {
    let messages = collect_session_messages(&store, &session_id, &req);
    let prompt = flatten_user_prompt(&messages);
    let system = system_prompt_from_req(&req);
    let comp_id = chat_completion_id();
    let created = unix_secs();
    let conv_id = format!("oai-{comp_id}");
    let cancel = CancellationToken::new();
    let agent_id = format!("openai-chat-stream-{}", comp_id);
    let (upstream, accum) = agent_runner::stream_events_with_tracking(
        &state,
        system,
        prompt,
        conv_id,
        cancel,
        "chat-completions-stream",
        agent_id,
        session_id.clone(),
    );
    let downstream = spawn_tee_persist(upstream, store, session_id, req.messages.clone());
    Ok(stream_forwarder::forward_chat_completions(
        downstream,
        comp_id,
        req.model,
        created,
        accum,
    ))
}

/// Tee `upstream` into a fresh receiver: forward every event and
/// persist the accumulated assistant text on `Done`.
fn spawn_tee_persist(
    mut upstream: mpsc::Receiver<tool::LoopEvent>,
    store: SessionStore,
    session_id: Option<String>,
    request_msgs: Vec<ChatMessage>,
) -> mpsc::Receiver<tool::LoopEvent> {
    let (tx, rx) = mpsc::channel::<tool::LoopEvent>(agent_runner::EVENT_CHANNEL_CAPACITY);
    tokio::spawn(async move {
        let mut acc = String::new();
        while let Some(ev) = upstream.recv().await {
            if let tool::LoopEvent::AssistantText(t) = &ev {
                acc.push_str(t);
            }
            let stop = matches!(ev, tool::LoopEvent::Done { .. });
            if tx.send(ev).await.is_err() {
                return;
            }
            if stop {
                let assistant = build_assistant_message(acc.clone(), Vec::new());
                persist_turn(&store, &session_id, &request_msgs, &assistant);
                return;
            }
        }
    });
    rx
}

#[cfg(test)]
mod tests {
    use super::*;

    fn msg(role: &str, content: &str) -> ChatMessage {
        ChatMessage {
            role: role.into(),
            content: Some(content.into()),
            name: None,
            tool_call_id: None,
            tool_calls: None,
        }
    }

    fn req_with(messages: Vec<ChatMessage>) -> ChatCompletionRequest {
        ChatCompletionRequest {
            model: "kei-cortex".into(),
            messages,
            stream: false,
            temperature: None,
            max_tokens: None,
            tools: None,
            tool_choice: None,
        }
    }

    #[test]
    fn system_prompt_picks_role_system_message() {
        let req = req_with(vec![msg("system", "custom system"), msg("user", "hi")]);
        assert_eq!(system_prompt_from_req(&req), "custom system");
    }

    #[test]
    fn system_prompt_falls_back_to_default_when_no_system_message() {
        let req = req_with(vec![msg("user", "hi")]);
        assert_eq!(system_prompt_from_req(&req), DEFAULT_SYSTEM);
    }

    #[tokio::test]
    async fn tee_forwards_events_and_persists_on_done() {
        let store = SessionStore::new();
        let sid = Some("sess-tee-1".to_string());
        let (tx, upstream) = mpsc::channel::<tool::LoopEvent>(8);
        let request_msgs = vec![msg("user", "hi")];
        let mut downstream = spawn_tee_persist(upstream, store.clone(), sid.clone(), request_msgs);
        tx.send(tool::LoopEvent::AssistantText("hello ".into())).await.unwrap();
        tx.send(tool::LoopEvent::AssistantText("world".into())).await.unwrap();
        tx.send(tool::LoopEvent::Done { conversation_id: "c".into(), turns: 1 })
            .await
            .unwrap();
        let mut count = 0;
        while downstream.recv().await.is_some() {
            count += 1;
        }
        assert_eq!(count, 3);
        let rec = store.get(sid.as_deref().unwrap()).expect("session persisted");
        let last = rec.messages.last().expect("assistant message persisted");
        assert_eq!(last.role, "assistant");
        assert_eq!(last.content.as_deref(), Some("hello world"));
    }
}
