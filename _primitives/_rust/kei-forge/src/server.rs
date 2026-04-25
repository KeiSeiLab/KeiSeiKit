//! Axum router — GET / (HTML form) and POST /forge (scaffold handler).
//!
//! Intentionally stateless: no `AppState`, no handles, no async init.
//! Every request is self-contained. This lets tests spin up `app()` in
//! an ephemeral Tokio runtime without setup teardown overhead.
//!
//! Security layers applied as middleware (see `crate::middleware`):
//! 1. `require_local_host` — reject non-127.0.0.1 Host headers (blocks
//!    DNS rebinding).
//! 2. `require_json_content_type` — reject urlencoded POSTs (blocks
//!    `<form>`-based CSRF).
//!
//! GET / responses additionally carry CSP + nosniff + frame-deny headers.

use axum::{
    http::{HeaderMap, StatusCode},
    middleware::from_fn,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use crate::form::{validate, ForgeRequest};
use crate::generate::{forge, ForgeResult};
use crate::headers::apply_security_headers;
use crate::html::FORM_HTML;
use crate::middleware::{require_json_content_type, require_local_host};

/// Build the router. Called by `main.rs` and by tests.
pub fn app() -> Router {
    Router::new()
        .route("/", get(render_form))
        .route("/forge", post(handle_forge))
        .layer(from_fn(require_json_content_type))
        .layer(from_fn(require_local_host))
}

async fn render_form() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    apply_security_headers(&mut headers);
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static("text/html; charset=utf-8"),
    );
    (StatusCode::OK, headers, FORM_HTML)
}

async fn handle_forge(Json(req): Json<ForgeRequest>) -> impl IntoResponse {
    if let Err(e) = validate(&req) {
        return (StatusCode::BAD_REQUEST, Json(ForgeResult::fail(e)));
    }
    let result = forge(&req);
    let status = if result.success {
        StatusCode::OK
    } else {
        StatusCode::UNPROCESSABLE_ENTITY
    };
    (status, Json(result))
}
