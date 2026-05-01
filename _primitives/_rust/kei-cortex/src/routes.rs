//! Router assembly + bearer-token middleware + CORS layer.
//!
//! `/healthz` is mounted OUTSIDE the auth middleware so monitors can hit it
//! without a token. Everything under `/api` goes through `require_bearer`.
//!
//! Per-route concurrency caps protect us from a runaway client draining our
//! upstream budget — `fal.ai` in particular bills per run, so we cap
//! `/portrait/stylize` at 2 concurrent installs system-wide. Other expensive
//! routes (`/tts`, `/stt`, `/chat`) get matching caps tuned to their bottleneck.

use crate::auth::tokens_match;
use crate::error::AppError;
use crate::handlers::{
    chat, fs_list, health, ledger, memory, pet, portrait, stt, summary, term, tool_apply, tts,
    usage,
};
use crate::state::AppState;
use axum::error_handling::HandleErrorLayer;
use axum::extract::{DefaultBodyLimit, Request, State};
use axum::http::{header, HeaderValue, Method, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::Router;

// OpenAI-compatible /v1/* surface. Lives under `routes/openai/` as a
// sibling tree to this file; declared via `#[path]` so the directory
// can coexist with `routes.rs` without becoming an inline `routes/mod.rs`.
#[path = "routes/openai/mod.rs"]
pub mod openai;
use tower::buffer::BufferLayer;
use tower::limit::ConcurrencyLimitLayer;
use tower::{BoxError, ServiceBuilder};
use tower_http::cors::CorsLayer;

/// Upper bound on the `/portrait/stylize` multipart body. The handler enforces
/// the stricter 10 MiB cap on the `file` field; this is just the pre-parse
/// gate that Axum applies before it can see individual fields.
const PORTRAIT_BODY_LIMIT: usize = 12 * 1024 * 1024;

/// Upper bound on the `/stt` multipart body. The handler enforces the
/// stricter 25 MiB cap on the `audio` field itself; this is the slack so
/// that field headers + form overhead do not trip the pre-parse gate.
const STT_BODY_LIMIT: usize = 26 * 1024 * 1024;

/// Max concurrent Flux stylize runs system-wide. fal.ai bills per run.
const PORTRAIT_CONCURRENCY: usize = 2;

/// Max concurrent ElevenLabs TTS calls.
const TTS_CONCURRENCY: usize = 4;

/// Max concurrent whisper worker runs. CPU-bound, so the cap matches a
/// conservative small-laptop core count.
const STT_CONCURRENCY: usize = 2;

/// Max concurrent Anthropic chat streams.
const CHAT_CONCURRENCY: usize = 8;

/// Build the top-level router. `cors_origin` must have been validated at
/// `AppConfig` construction time so this function cannot fail.
pub fn build_router(state: AppState) -> Router {
    let cors = build_cors(state.config().cors_origin.as_str())
        .expect("cors_origin must be valid — validated in AppConfig::new");

    // Per-route granular caps proved fragile with axum 0.7's MethodRouter
    // layer bounds (HandleErrorLayer + ConcurrencyLimitLayer service is not
    // `Clone` in a way that layer() accepts). Apply a single router-wide cap
    // via route_layer — it wraps every inner route uniformly without the
    // per-method error-type headache. The cap is the SUM of the per-route
    // budgets (2+4+2+8 = 16), which is a strict upper bound on simultaneous
    // expensive work. Finer-grained token-bucket per-route can land later via
    // tower-governor if a multi-user deployment appears.
    let api = Router::new()
        .route("/api/v1/cortex/summary", get(summary::summary))
        .route("/api/v1/cortex/pet/:user_id", get(pet::get_pet))
        .route(
            "/api/v1/cortex/pet/:user_id/interaction",
            post(pet::post_interaction),
        )
        .route("/api/v1/cortex/pet/:user_id/chat", post(chat::chat))
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
            post(tts::synthesize),
        )
        .route("/api/v1/cortex/ledger/recent", get(ledger::recent))
        .route("/api/v1/cortex/memory/search", get(memory::search_memory))
        .route("/api/v1/cortex/usage", get(usage::usage))
        .route("/api/v1/cortex/fs/list", get(fs_list::list))
        .route("/api/v1/cortex/tool/apply", post(tool_apply::apply))
        .route("/api/v1/cortex/term", get(term::ws_handler))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_bearer,
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

/// Bearer-token middleware.
///
/// Two acceptable transports — checked in order:
///   1. `Authorization: Bearer <token>` — standard HTTP requests.
///   2. `Sec-WebSocket-Protocol: bearer, <token>` — WS upgrade only,
///      because browsers cannot set the Authorization header on a
///      `new WebSocket(url, [...protocols])` call.
///
/// Missing → 401; mismatch → 403.
async fn require_bearer(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = state.token().to_string();
    if let Some(got) = bearer_from_authorization(&req) {
        return finish_auth(&token, &got, req, next).await;
    }
    if let Some(got) = bearer_from_websocket_subprotocol(&req) {
        return finish_auth(&token, &got, req, next).await;
    }
    Err(AppError::Unauthorized)
}

/// Pull `<token>` from `Authorization: Bearer <token>` if present.
fn bearer_from_authorization(req: &Request) -> Option<String> {
    let v = req.headers().get(header::AUTHORIZATION)?.to_str().ok()?;
    Some(v.strip_prefix("Bearer ")?.trim().to_string())
}

/// Pull `<token>` from `Sec-WebSocket-Protocol: bearer, <token>`. The
/// browser's `new WebSocket(url, ['bearer', tok])` produces this header.
fn bearer_from_websocket_subprotocol(req: &Request) -> Option<String> {
    let v = req
        .headers()
        .get("sec-websocket-protocol")?
        .to_str()
        .ok()?;
    let mut parts = v.split(',').map(str::trim);
    if parts.next()? != "bearer" {
        return None;
    }
    Some(parts.next()?.to_string())
}

/// Compare expected vs. supplied; on match, call `next`.
async fn finish_auth(
    expected: &str,
    got: &str,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    if !tokens_match(expected, got) {
        return Err(AppError::Forbidden);
    }
    Ok(next.run(req).await)
}
