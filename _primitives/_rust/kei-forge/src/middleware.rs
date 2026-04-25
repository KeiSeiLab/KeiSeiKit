//! HTTP middleware — defence against cross-origin / DNS-rebinding attacks.
//!
//! Two layers:
//! - [`require_local_host`] — rejects requests whose `Host:` header is not
//!   exactly `localhost:8747` or `127.0.0.1:8747`. Blocks DNS-rebinding
//!   (attacker points `a.evil.com` → 127.0.0.1 while browser still trusts
//!   the evil.com origin for Same-Origin-Policy purposes).
//! - [`require_json_content_type`] — rejects `POST /forge` unless body is
//!   `application/json`. Blocks CSRF via `<form>` submissions: urlencoded
//!   POSTs are SOP-safe (no preflight), but JSON bodies trigger CORS
//!   preflight so SOP engages.
//!
//! Both are advisory: they compose via `axum::middleware::from_fn` and
//! never touch application state.

use axum::{
    extract::Request,
    http::{header, Method, StatusCode},
    middleware::Next,
    response::Response,
};

const ALLOWED_HOSTS: &[&str] = &["localhost:8747", "127.0.0.1:8747"];

/// Reject requests whose `Host:` is not an exact allow-list match.
///
/// Returns 421 Misdirected Request on mismatch (RFC 7540 §9.1.2).
pub async fn require_local_host(req: Request, next: Next) -> Result<Response, StatusCode> {
    let host = req
        .headers()
        .get(header::HOST)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if ALLOWED_HOSTS.iter().any(|&h| h == host) {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::MISDIRECTED_REQUEST)
    }
}

/// Reject POSTs whose `Content-Type` is not `application/json`.
///
/// GET and other methods pass through unchanged. Returns 415 Unsupported
/// Media Type on mismatch.
pub async fn require_json_content_type(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if req.method() != Method::POST {
        return Ok(next.run(req).await);
    }
    let ct = req
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    // Match only the media type, ignoring optional `; charset=…`.
    let base = ct.split(';').next().unwrap_or("").trim();
    if base.eq_ignore_ascii_case("application/json") {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNSUPPORTED_MEDIA_TYPE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request as HttpRequest;
    use axum::middleware::from_fn;
    use axum::routing::{get, post};
    use axum::Router;
    use tower::ServiceExt;

    fn test_app() -> Router {
        Router::new()
            .route("/", get(|| async { "ok" }))
            .route("/forge", post(|| async { "ok" }))
            .layer(from_fn(require_json_content_type))
            .layer(from_fn(require_local_host))
    }

    #[tokio::test]
    async fn blocks_evil_host() {
        let app = test_app();
        let resp = app
            .oneshot(
                HttpRequest::builder()
                    .uri("/")
                    .header("host", "evil.com")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::MISDIRECTED_REQUEST);
    }

    #[tokio::test]
    async fn blocks_urlencoded_post() {
        let app = test_app();
        let resp = app
            .oneshot(
                HttpRequest::builder()
                    .method("POST")
                    .uri("/forge")
                    .header("host", "127.0.0.1:8747")
                    .header("content-type", "application/x-www-form-urlencoded")
                    .body(Body::from("x=1"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }
}
