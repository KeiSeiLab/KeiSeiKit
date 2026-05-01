//! POST /v1/runs, GET /v1/runs/{id}/events, POST /v1/runs/{id}/stop.
//!
//! `runs` is the asynchronous variant of chat-completions: POST returns
//! 202 + run id immediately, GET subscribes to the SSE event stream,
//! POST /stop fires the run's `CancellationToken` so the agent loop
//! exits gracefully at its next checkpoint.
//!
//! Constructor Pattern: state lives in `run_registry::RunRegistry`,
//! the real agent in `run_agent::run_real`. This file owns ONLY the
//! three HTTP handlers + validation + the per-chunk SSE mapper.
//!
//! P1.1.d (2026-04-28): wired the spawn path to the real agent loop
//! via `run_agent::run_real`. The previous `run_stub` is removed.

use super::error::OpenAiError;
use super::ids::run_id;
use super::run_agent::run_real;
use super::run_registry::{self, RunSlot};
use super::sse::AgentChunk;
use super::types::{RunObject, RunRequest};
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::response::sse::Event;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

/// `POST /v1/runs` — accept the request, allocate a run id, spawn the
/// real agent task, return 202 + RunObject immediately.
pub async fn create_run(
    State(state): State<AppState>,
    Json(req): Json<RunRequest>,
) -> Result<(axum::http::StatusCode, Json<RunObject>), OpenAiError> {
    validate(&req)?;
    let registry = run_registry::global();
    let id = run_id();
    let created_at = unix_secs();
    let (tx, rx) = mpsc::channel::<AgentChunk>(super::sse::CHANNEL_CAPACITY);
    let cancel = CancellationToken::new();
    registry.insert(id.clone(), make_slot(&req.model, created_at, rx, &cancel));
    spawn_real(state, registry, id.clone(), req.messages, tx, cancel);
    let body = make_run_object(id, created_at, req.model);
    Ok((axum::http::StatusCode::ACCEPTED, Json(body)))
}

fn make_slot(
    model: &str,
    created_at: u64,
    rx: mpsc::Receiver<AgentChunk>,
    cancel: &CancellationToken,
) -> RunSlot {
    RunSlot {
        model: model.to_string(),
        created_at,
        status: "queued".into(),
        rx: Arc::new(Mutex::new(Some(rx))),
        cancel: cancel.clone(),
    }
}

fn make_run_object(id: String, created_at: u64, model: String) -> RunObject {
    RunObject { id, object: "run", created_at, status: "queued".into(), model }
}

/// Spawn the real agent loop in the background. Mirrors P1.1.b's
/// chat-completions wiring but runs detached so POST /v1/runs can
/// return 202 immediately.
fn spawn_real(
    state: AppState,
    registry: super::run_registry::RunRegistry,
    id: String,
    messages: Vec<super::types::ChatMessage>,
    tx: mpsc::Sender<AgentChunk>,
    cancel: CancellationToken,
) {
    tokio::spawn(async move {
        run_real(state, registry, id, messages, tx, cancel).await;
    });
}

/// `GET /v1/runs/{id}/events` — SSE attached to the run's chunk channel.
/// Returns 404 if the run id is unknown OR has already been consumed.
pub async fn run_events(Path(id): Path<String>) -> Result<Response, OpenAiError> {
    let registry = run_registry::global();
    registry
        .get(&id)
        .ok_or_else(|| OpenAiError::NotFound(format!("run {id}")))?;
    let rx = registry
        .take_receiver(&id)
        .ok_or_else(|| OpenAiError::NotFound(format!("run {id} already consumed")))?;
    let id_for_closure = id.clone();
    let sse = super::sse::sse_from_rx(rx, move |chunk| run_event_for(&id_for_closure, chunk));
    Ok(sse.into_response())
}

fn run_event_for(id: &str, chunk: &AgentChunk) -> Option<Event> {
    match chunk {
        AgentChunk::Delta(t) => Some(
            Event::default()
                .event("run.message.delta")
                .data(json!({ "run_id": id, "delta": t }).to_string()),
        ),
        AgentChunk::Done(u) => Some(
            Event::default().event("run.completed").data(
                json!({
                    "run_id": id,
                    "usage": {
                        "prompt_tokens": u.prompt_tokens,
                        "completion_tokens": u.completion_tokens,
                        "total_tokens": u.total_tokens,
                    },
                })
                .to_string(),
            ),
        ),
        _ => None,
    }
}

/// `POST /v1/runs/{id}/stop` — fire the run's `CancellationToken` so
/// the agent loop exits at its next checkpoint.
pub async fn stop_run(
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, OpenAiError> {
    let registry = run_registry::global();
    let cancelled = registry.cancel(&id);
    if !cancelled {
        return Err(OpenAiError::NotFound(format!("run {id}")));
    }
    registry.mark(&id, "cancelling");
    Ok(Json(json!({
        "id": id,
        "object": "run.cancelled",
        "status": "cancelling",
    })))
}

fn validate(req: &RunRequest) -> Result<(), OpenAiError> {
    if req.model.trim().is_empty() {
        return Err(OpenAiError::BadRequest("model is required".into()));
    }
    if req.messages.is_empty() {
        return Err(OpenAiError::BadRequest("messages must not be empty".into()));
    }
    Ok(())
}

fn unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::super::types::ChatMessage;
    use super::*;

    fn req() -> RunRequest {
        RunRequest {
            model: "kei-cortex".into(),
            messages: vec![ChatMessage {
                role: "user".into(),
                content: Some("hi".into()),
                name: None,
                tool_call_id: None,
                tool_calls: None,
            }],
            metadata: None,
        }
    }

    #[test]
    fn validate_rejects_empty_messages() {
        let mut r = req();
        r.messages.clear();
        assert!(validate(&r).is_err());
    }

    #[tokio::test]
    async fn stop_unknown_run_returns_not_found() {
        let res = stop_run(Path("definitely-no-such-run-id".into())).await;
        assert!(matches!(res, Err(OpenAiError::NotFound(_))));
    }
}
