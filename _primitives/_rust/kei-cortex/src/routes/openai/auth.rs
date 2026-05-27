//! Bearer-token middleware for the OpenAI-compatible `/v1/*` surface.
//!
//! Two acceptance modes:
//!   1. `KEI_API_KEY` env var is set → require `Authorization: Bearer <key>`
//!      and compare in constant time via `subtle::ConstantTimeEq`.
//!   2. `KEI_API_KEY` unset → allow only requests whose source IP is the
//!      loopback address (127.0.0.1 / ::1). This matches Hermes' default
//!      and prevents accidentally exposing a tokenless endpoint on the LAN.

use super::error::OpenAiError;
use axum::extract::{ConnectInfo, Request};
use axum::http::header;
use axum::middleware::Next;
use axum::response::Response;
use std::net::SocketAddr;

/// Env var name read at request time. Reading per-request lets the daemon
/// pick up a freshly-rotated key without restarting.
const ENV_KEY: &str = "KEI_API_KEY";

/// Tower middleware. Reject 401 on missing/invalid key when one is set;
/// allow loopback-only when no key is configured.
pub async fn require_openai_key(
    conn: Option<ConnectInfo<SocketAddr>>,
    req: Request,
    next: Next,
) -> Result<Response, OpenAiError> {
    match std::env::var(ENV_KEY).ok().filter(|s| !s.is_empty()) {
        Some(expected) => check_bearer(&expected, &req)?,
        None => check_loopback(conn.as_ref())?,
    }
    Ok(next.run(req).await)
}

/// Constant-time bearer-token comparison. Returns `Unauthorized` on
/// missing header, `Forbidden` on mismatch.
fn check_bearer(expected: &str, req: &Request) -> Result<(), OpenAiError> {
    let got = bearer_from_authorization(req).ok_or(OpenAiError::Unauthorized)?;
    if constant_time_eq(got.as_bytes(), expected.as_bytes()) {
        Ok(())
    } else {
        Err(OpenAiError::Forbidden)
    }
}

/// Constant-time byte-slice equality. Returns false fast on length
/// mismatch (length is not a secret in this protocol — it's the
/// fixed user-supplied API key length). The byte loop runs in time
/// proportional to `min(a.len(), b.len())` regardless of content,
/// matching `subtle::ConstantTimeEq` semantics for our use-case
/// without pulling in the dependency.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    // Length is a public, fixed-length design parameter (operator-set
    // `KEI_API_KEY`), not a secret — early return on length mismatch is benign.
    if a.len() != b.len() {
        return false;
    }
    let mut diff: u8 = 0;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

/// Pull `<token>` from `Authorization: Bearer <token>`.
fn bearer_from_authorization(req: &Request) -> Option<String> {
    let v = req.headers().get(header::AUTHORIZATION)?.to_str().ok()?;
    Some(v.strip_prefix("Bearer ")?.trim().to_string())
}

/// When no API key is configured, restrict to loopback callers. Missing
/// `ConnectInfo` (e.g. inside `Router::into_make_service` without
/// `into_make_service_with_connect_info`) is treated as non-loopback —
/// safer to refuse than to leak.
fn check_loopback(conn: Option<&ConnectInfo<SocketAddr>>) -> Result<(), OpenAiError> {
    let addr = conn.ok_or(OpenAiError::Unauthorized)?;
    if addr.0.ip().is_loopback() {
        Ok(())
    } else {
        Err(OpenAiError::Forbidden)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    #[test]
    fn bearer_extracted_from_authorization() {
        let mut req = Request::new(axum::body::Body::empty());
        req.headers_mut()
            .insert(header::AUTHORIZATION, HeaderValue::from_static("Bearer abc"));
        assert_eq!(bearer_from_authorization(&req).as_deref(), Some("abc"));
    }

    #[test]
    fn bearer_missing_yields_none() {
        let req = Request::new(axum::body::Body::empty());
        assert!(bearer_from_authorization(&req).is_none());
    }

    #[test]
    fn check_bearer_constant_time_match() {
        let mut req = Request::new(axum::body::Body::empty());
        req.headers_mut()
            .insert(header::AUTHORIZATION, HeaderValue::from_static("Bearer secret"));
        assert!(check_bearer("secret", &req).is_ok());
        assert!(matches!(
            check_bearer("other", &req),
            Err(OpenAiError::Forbidden)
        ));
    }
}
