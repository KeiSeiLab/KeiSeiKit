//! POST /v1/responses, GET /v1/responses/{id}, DELETE /v1/responses/{id}.
//!
//! Stateful sibling of chat-completions — `previous_response_id` chains
//! turns server-side. State lives in the in-memory `SessionStore`.
//!
//! HERMES-MIGRATION P1.1.c: replaces `stub_agent_reply` with real loop
//! via `agent_runner::collect_reply` (sync) + `stream_events` +
//! `stream_forwarder::forward_responses` (stream). Mirrors P1.1.b.

use super::agent_runner;
use super::error::OpenAiError;
use super::session::{self, new_response_skeleton, SessionRecord, SessionStore};
use super::stream_forwarder;
use super::types::{ResponseObject, ResponsesRequest};
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use tokio_util::sync::CancellationToken;

/// Default system prompt when `instructions` is absent. Mirrors
/// `chat_completions::DEFAULT_SYSTEM` for surface parity.
const DEFAULT_SYSTEM: &str =
    "You are kei-cortex, a helpful assistant. Use <think> for reasoning and tool calls when appropriate.";

/// `POST /v1/responses`. Stream or sync depending on `req.stream`.
pub async fn create_response(
    State(state): State<AppState>,
    Json(req): Json<ResponsesRequest>,
) -> Result<Response, OpenAiError> {
    let store = session::global();
    validate(&req)?;
    let prior = req.previous_response_id.as_ref().and_then(|id| store.get(id));
    let prompt = build_prompt(&req, prior.as_ref());
    let system = system_prompt_from_req(&req);
    if req.stream {
        Ok(handle_stream(state, store, req, system, prompt).into_response())
    } else {
        Ok(handle_sync(state, store, req, system, prompt).await?.into_response())
    }
}

/// `GET /v1/responses/{id}`.
pub async fn get_response(Path(id): Path<String>) -> Result<Json<ResponseObject>, OpenAiError> {
    let store = session::global();
    let rec = store.get(&id).ok_or_else(|| OpenAiError::NotFound(format!("response {id}")))?;
    let obj = rec.last_response.ok_or_else(|| OpenAiError::NotFound(format!("response {id}")))?;
    Ok(Json(obj))
}

/// `DELETE /v1/responses/{id}`.
pub async fn delete_response(Path(id): Path<String>) -> Result<Json<serde_json::Value>, OpenAiError> {
    let store = session::global();
    let removed = store.delete(&id).is_some();
    Ok(Json(json!({ "id": id, "object": "response.deleted", "deleted": removed })))
}

fn validate(req: &ResponsesRequest) -> Result<(), OpenAiError> {
    if req.model.trim().is_empty() {
        return Err(OpenAiError::BadRequest("model is required".into()));
    }
    if req.input.is_null() {
        return Err(OpenAiError::BadRequest("input is required".into()));
    }
    Ok(())
}

/// `instructions` wins; blank or missing falls back to `DEFAULT_SYSTEM`.
/// Responses API has no chat-style `messages[]` so no `role: system` to inspect.
fn system_prompt_from_req(req: &ResponsesRequest) -> String {
    req.instructions
        .clone()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| DEFAULT_SYSTEM.to_string())
}

/// Build agent prompt: prior response output (chained) + current `input`.
fn build_prompt(req: &ResponsesRequest, prior: Option<&SessionRecord>) -> String {
    let mut buf = String::new();
    if let Some(resp) = prior.and_then(|p| p.last_response.as_ref()) {
        for blk in &resp.output {
            if let Some(t) = blk.get("text").and_then(|v| v.as_str()) {
                buf.push_str(t);
                buf.push_str("\n\n");
            }
        }
    }
    if let Some(s) = req.input.as_str() {
        buf.push_str(s);
    } else {
        buf.push_str(&req.input.to_string());
    }
    buf
}

/// Sync — run loop to completion, pack into `ResponseObject` envelope.
async fn handle_sync(
    state: AppState,
    store: SessionStore,
    req: ResponsesRequest,
    system: String,
    prompt: String,
) -> Result<Json<ResponseObject>, OpenAiError> {
    let obj = new_response_skeleton(req.model, req.previous_response_id.clone());
    let agent_id = format!("openai-responses-{}", obj.id);
    let (reply_text, _tool_calls, _usage) = agent_runner::collect_reply(
        &state,
        system,
        prompt,
        Vec::new(),
        "responses",
        agent_id,
        req.previous_response_id,
    )
    .await?;
    let mut obj = obj;
    obj.output = vec![output_text_block(&reply_text)];
    persist(&store, &obj);
    Ok(Json(obj))
}

/// Stream — drive loop and forward `LoopEvent`s as SSE via
/// `stream_forwarder::forward_responses`. `response.completed` carries
/// the persisted `ResponseObject` skeleton. Phase 2:
/// `stream_events_with_tracking` fires a fire-and-forget token-event
/// after the loop closes.
fn handle_stream(
    state: AppState,
    store: SessionStore,
    req: ResponsesRequest,
    system: String,
    prompt: String,
) -> Response {
    let obj = new_response_skeleton(req.model, req.previous_response_id.clone());
    persist(&store, &obj);
    let conv_id = format!("oai-{}", obj.id);
    let cancel = CancellationToken::new();
    let agent_id = format!("openai-responses-stream-{}", obj.id);
    let (upstream, _accum) = agent_runner::stream_events_with_tracking(
        &state,
        system,
        prompt,
        conv_id,
        cancel,
        "responses-stream",
        agent_id,
        req.previous_response_id,
    );
    stream_forwarder::forward_responses(upstream, obj)
}

/// Persist response by id so GET / continuation work.
fn persist(store: &SessionStore, obj: &ResponseObject) {
    let mut rec = store.get(&obj.id).unwrap_or_else(SessionRecord::empty);
    rec.last_response = Some(obj.clone());
    store.put(obj.id.clone(), rec);
}

/// Single `output[]` content block in the JSON envelope.
fn output_text_block(text: &str) -> serde_json::Value {
    json!({
        "type": "message",
        "role": "assistant",
        "content": [{ "type": "output_text", "text": text }],
        "text": text,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn req() -> ResponsesRequest {
        ResponsesRequest {
            model: "kei-cortex".into(),
            input: json!("hi"),
            previous_response_id: None,
            stream: false,
            instructions: None,
        }
    }

    #[test]
    fn validate_rejects_empty_model() {
        let mut r = req();
        r.model = "".into();
        assert!(validate(&r).is_err());
    }

    #[test]
    fn system_prompt_picks_instructions_field() {
        let mut r = req();
        r.instructions = Some("custom system".into());
        assert_eq!(system_prompt_from_req(&r), "custom system");
    }

    #[test]
    fn system_prompt_falls_back_when_instructions_absent_or_blank() {
        assert_eq!(system_prompt_from_req(&req()), DEFAULT_SYSTEM);
        let mut r = req();
        r.instructions = Some("   ".into());
        assert_eq!(system_prompt_from_req(&r), DEFAULT_SYSTEM);
    }

    #[tokio::test]
    async fn delete_returns_deleted_object_envelope() {
        let resp = delete_response(Path("nope-unique-id-xyz".into())).await.unwrap();
        assert_eq!(resp.0["object"], "response.deleted");
        assert_eq!(resp.0["deleted"], false);
    }
}
