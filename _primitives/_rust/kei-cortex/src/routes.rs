//! Router assembly + CORS layer.
//!
//! `/healthz` is mounted OUTSIDE the auth middleware so monitors can hit it
//! without a token. Everything under `/api` goes through `require_bearer`
//! (defined in `routes_auth`).
//!
//! Per-route concurrency caps protect us from a runaway client draining our
//! upstream budget — `fal.ai` in particular bills per run, so we cap
//! `/portrait/stylize` at 2 concurrent installs system-wide. Other expensive
//! routes (`/tts`, `/stt`, `/chat`) get matching caps tuned to their bottleneck.

use crate::handlers::{
    chat, fs_list, health, ledger, memory, pet, portrait, stt, summary, term, tool_apply, tts,
    usage,
};
use crate::state::AppState;
use axum::error_handling::HandleErrorLayer;
use axum::extract::DefaultBodyLimit;
use axum::http::{header, HeaderValue, Method, StatusCode};
use axum::middleware;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;

#[path = "routes/openai/mod.rs"]
pub mod openai;

use tower::buffer::BufferLayer;
use tower::limit::ConcurrencyLimitLayer;
use tower::{BoxError, ServiceBuilder};
use tower_http::cors::CorsLayer;

// --- Body limits (per-route pre-parse gates) --------------------------------
const PORTRAIT_BODY_LIMIT: usize = 12 * 1024 * 1024; // 12 MiB (handler re-checks 10)
const STT_BODY_LIMIT: usize = 26 * 1024 * 1024;       // 26 MiB (handler re-checks 25)
const CHAT_BODY_LIMIT: usize = 256 * 1024;             // 256 KiB
const TOOL_APPLY_BODY_LIMIT: usize = 11 * 1024 * 1024; // 11 MiB (handler checks 10)
const INTERACTION_BODY_LIMIT: usize = 64 * 1024;       // 64 KiB
const TTS_BODY_LIMIT: usize = 32 * 1024;               // 32 KiB

// --- Concurrency budgets ----------------------------------------------------
const PORTRAIT_CONCURRENCY: usize = 2;
const TTS_CONCURRENCY: usize = 4;
const STT_CONCURRENCY: usize = 2;
const CHAT_CONCURRENCY: usize = 8;

/// Build the top-level router. `cors_origin` must have been validated at
/// `AppConfig` construction time so this function cannot fail.
pub fn build_router(state: AppState) -> Router {
    let cors = build_cors(state.config().cors_origin.as_str())
        .expect("cors_origin must be valid — validated in AppConfig::new");

    let api = build_api_router();
    let api = api
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            crate::routes_auth::require_bearer,
        ))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|_: BoxError| async {
                    (StatusCode::SERVICE_UNAVAILABLE, "server busy").into_response()
                }))
                .layer(BufferLayer::new(64))
                .layer(ConcurrencyLimitLayer::new(
                    PORTRAIT_CONCURRENCY + TTS_CONCURRENCY + STT_CONCURRENCY + CHAT_CONCURRENCY,
                )),
        );

    Router::new()
        .route("/healthz", get(health::healthz))
        .merge(api)
        .merge(openai::openai_router())
        .layer(cors)
        .with_state(state)
}

/// Assemble the protected API sub-router (no auth layer yet — applied by caller).
fn build_api_router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/cortex/summary", get(summary::summary))
        .route("/api/v1/cortex/pet/:user_id", get(pet::get_pet))
        .route(
            "/api/v1/cortex/pet/:user_id/interaction",
            post(pet::post_interaction)
                .layer(DefaultBodyLimit::max(INTERACTION_BODY_LIMIT)),
        )
        .route(
            "/api/v1/cortex/pet/:user_id/chat",
            post(chat::chat).layer(DefaultBodyLimit::max(CHAT_BODY_LIMIT)),
        )
        .route(
            "/api/v1/cortex/pet/:user_id/portrait/stylize",
            post(portrait::stylize).layer(DefaultBodyLimit::max(PORTRAIT_BODY_LIMIT)),
        )
        .route(
            "/api/v1/cortex/stt",
            post(stt::transcribe).layer(DefaultBodyLimit::max(STT_BODY_LIMIT)),
        )
        .route(
            "/api/v1/cortex/pet/:user_id/tts",
            post(tts::synthesize).layer(DefaultBodyLimit::max(TTS_BODY_LIMIT)),
        )
        .route("/api/v1/cortex/ledger/recent", get(ledger::recent))
        .route("/api/v1/cortex/memory/search", get(memory::search_memory))
        .route("/api/v1/cortex/usage", get(usage::usage))
        .route("/api/v1/cortex/fs/list", get(fs_list::list))
        .route(
            "/api/v1/cortex/tool/apply",
            post(tool_apply::apply).layer(DefaultBodyLimit::max(TOOL_APPLY_BODY_LIMIT)),
        )
        .route("/api/v1/cortex/term", get(term::ws_handler))
}

/// Build the CORS layer locked to a single origin.
fn build_cors(origin: &str) -> Result<CorsLayer, String> {
    let origin_header: HeaderValue = origin
        .parse()
        .map_err(|e| format!("parse cors origin {origin:?}: {e}"))?;
    Ok(CorsLayer::new()
        .allow_origin(origin_header)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .allow_credentials(true))
}

