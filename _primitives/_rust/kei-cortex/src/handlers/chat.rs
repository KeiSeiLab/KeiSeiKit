//! `POST /api/v1/cortex/pet/:user_id/chat` — agentic streaming chat endpoint.
//!
//! Pipeline: validate → discover CLAUDE.md/AGENTS.md → match `/skill-name` →
//! build system prompt → pick provider → run tool loop → translate
//! `LoopEvent` to SSE → wire client-disconnect to oneshot cancel.
//!
//! Cost recording (Wave 40): a token-usage accumulator wraps the model
//! invoker; after the agentic loop emits `Done`, we spawn a blocking
//! task to write the row to kei-ledger via `chat_cost::record_chat_cost`.
//! See `chat_stream.rs` for the actual wiring; this file owns the
//! handler shell and request validation only.

use super::chat_stream::run_loop_stream;
use crate::context;
use crate::error::AppError;
use crate::persona::load_and_render;
use crate::state::AppState;
use crate::validate;
use axum::extract::{Path, Query, State};
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::Json;
use futures::stream::Stream;
use kei_router::LlmError;
use serde::Deserialize;
use std::convert::Infallible;
use uuid::Uuid;

/// JSON request body.
#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    #[serde(default)]
    pub conversation_id: Option<String>,
}

/// Optional `?provider=<name>` selector.
#[derive(Debug, Default, Deserialize)]
pub struct ChatQuery {
    #[serde(default)]
    pub provider: Option<String>,
}

/// Type alias for the axum SSE response this handler returns.
type SseResponse = Sse<std::pin::Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send>>>;

/// Handler entry point. Validates inputs synchronously, then returns SSE.
pub async fn chat(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    Query(query): Query<ChatQuery>,
    Json(req): Json<ChatRequest>,
) -> Result<SseResponse, AppError> {
    validate::user_id(&user_id)?;
    validate_body(&req)?;
    let system = build_augmented_system(&state, &user_id, &req.message)?;
    let provider_name = pick_provider_name(&state, query.provider);
    validate_provider(&state, &provider_name)?;
    let conversation_id = req
        .conversation_id
        .clone()
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    let stream = run_loop_stream(
        system,
        req.message.clone(),
        conversation_id,
        state,
        user_id,
        provider_name,
        req.conversation_id.clone(),
    );
    let boxed: std::pin::Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send>> =
        Box::pin(stream);
    Ok(Sse::new(boxed).keep_alive(KeepAlive::default()))
}

/// Discover context + match skill + build augmented system prompt.
fn build_augmented_system(s: &AppState, uid: &str, msg: &str) -> Result<String, AppError> {
    let (_, persona) = load_and_render(&s.config().pet_root, uid)?;
    let ctx = context::discover(&s.config().cwd);
    let skill = context::match_skill_command(msg, &s.config().project_root);
    Ok(context::build_system_prompt(&persona, &ctx, skill.as_ref()))
}

/// Query param wins; else fallback to config default.
fn pick_provider_name(state: &AppState, q: Option<String>) -> String {
    q.unwrap_or_else(|| state.config().default_provider.clone())
}

/// Confirm the provider is registered (env had its key).
fn validate_provider(state: &AppState, name: &str) -> Result<(), AppError> {
    match state.router().pick(name) {
        Ok(_) => Ok(()),
        Err(LlmError::UnknownProvider(_)) => {
            Err(AppError::BadRequest(format!("unknown provider: {name}")))
        }
        Err(e) => Err(AppError::BadGateway(format!("router pick: {e}"))),
    }
}

/// Character ceiling for chat messages. Prevents runaway prompt injection
/// and upstream token cost abuse.
const MAX_MESSAGE_CHARS: usize = 50_000;

fn validate_body(req: &ChatRequest) -> Result<(), AppError> {
    if req.message.is_empty() {
        return Err(AppError::BadRequest("message is empty".into()));
    }
    let chars = req.message.chars().count();
    if chars > MAX_MESSAGE_CHARS {
        return Err(AppError::PayloadTooLarge(format!(
            "{chars} chars > {MAX_MESSAGE_CHARS}"
        )));
    }
    Ok(())
}

#[cfg(test)]
#[path = "chat_test.rs"]
mod tests;
