//! OpenAI-compatible HTTP surface — `/v1/*` routes.
//!
//! Constructor Pattern: this `mod.rs` ONLY assembles the router.
//! Each cube under it owns one responsibility:
//!
//!   * `types.rs`              wire-format DTOs
//!   * `error.rs`              OpenAI-shaped error envelope
//!   * `auth.rs`               Bearer-token / loopback middleware
//!   * `translation.rs`        OpenAI ⇄ kei-cortex tool-name mapping
//!   * `sse.rs`                SSE primitives + `kei.tool.progress`
//!   * `stream_chunks.rs`      chat-completion stream frame builders
//!   * `session.rs`            in-memory session continuity store
//!   * `run_registry.rs`       in-memory `/v1/runs` slot store
//!   * `run_agent.rs`          real agent loop (P1.1.d) — drains
//!                              `tool::LoopEvent` into `AgentChunk`s
//!                              for the SSE handler in `runs.rs`
//!   * `ids.rs`                prefixed-uuid id generators
//!   * `models.rs`             GET /v1/models
//!   * `chat_completions.rs`   POST /v1/chat/completions
//!   * `chat_helpers.rs`       chat-completions validation helpers
//!   * `responses.rs`          POST /v1/responses + GET/DELETE
//!   * `runs.rs`               POST /v1/runs + events + stop
//!
//! State (`SessionStore`, `RunRegistry`) is held in process-global
//! `once_cell::Lazy` singletons (`session::global()`,
//! `run_registry::global()`) so the returned router is `Router<S>` for
//! any `S` and merges cleanly into the existing kei-cortex router
//! whose state is `AppState`.

#[path = "agent_runner.rs"]
pub mod agent_runner;
#[path = "auth.rs"]
pub mod auth;
#[path = "chat_completions.rs"]
pub mod chat_completions;
#[path = "chat_helpers.rs"]
pub mod chat_helpers;
#[path = "error.rs"]
pub mod error;
#[path = "ids.rs"]
pub mod ids;
#[path = "models.rs"]
pub mod models;
#[path = "responses.rs"]
pub mod responses;
#[path = "run_agent.rs"]
pub mod run_agent;
#[path = "run_registry.rs"]
pub mod run_registry;
#[path = "runs.rs"]
pub mod runs;
#[path = "session.rs"]
pub mod session;
#[path = "sse.rs"]
pub mod sse;
#[path = "stream_chunks.rs"]
pub mod stream_chunks;
#[path = "stream_forwarder.rs"]
pub mod stream_forwarder;
#[path = "translation.rs"]
pub mod translation;
#[path = "types.rs"]
pub mod types;

use axum::middleware;
use axum::routing::{delete, get, post};
use axum::Router;

use crate::state::AppState;

/// Build the `/v1/*` sub-router. Auth middleware reads `KEI_API_KEY`
/// from env at request time. When the env var is unset, only loopback
/// callers are allowed (see `auth::require_openai_key`).
pub fn openai_router() -> Router<AppState> {
    Router::new()
        .route(
            "/v1/chat/completions",
            post(chat_completions::chat_completions),
        )
        .route("/v1/responses", post(responses::create_response))
        .route("/v1/responses/:id", get(responses::get_response))
        .route("/v1/responses/:id", delete(responses::delete_response))
        .route("/v1/runs", post(runs::create_run))
        .route("/v1/runs/:id/events", get(runs::run_events))
        .route("/v1/runs/:id/stop", post(runs::stop_run))
        .route("/v1/models", get(models::list_models))
        .layer(middleware::from_fn(auth::require_openai_key))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn router_builds() {
        let _r: Router<AppState> = openai_router();
    }
}
