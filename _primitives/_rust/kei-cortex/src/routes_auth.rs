//! Bearer-token middleware for the cortex API router.
//!
//! Separated from `routes.rs` so each file stays under 200 LOC.

use crate::auth::tokens_match;
use crate::error::AppError;
use crate::state::AppState;
use axum::extract::{Request, State};
use axum::http::header;
use axum::middleware::Next;
use axum::response::Response;

/// Bearer-token middleware.
///
/// Two acceptable transports — checked in order:
///   1. `Authorization: Bearer <token>` — standard HTTP requests.
///   2. `Sec-WebSocket-Protocol: bearer, <token>` — WS upgrade only,
///      because browsers cannot set the Authorization header on a
///      `new WebSocket(url, [...protocols])` call.
///
/// Missing → 401; mismatch → 403.
pub async fn require_bearer(
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
