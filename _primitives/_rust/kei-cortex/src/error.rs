//! Unified error type mapped to HTTP responses with JSON body.
//!
//! Handlers return `Result<T, AppError>` and axum converts the error via
//! `IntoResponse`. All outbound bodies share the shape
//! `{ "error": { "code": "...", "message": "..." } }` so the UI has a single
//! parser.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

/// Application-level error. Variants map 1:1 to HTTP status codes.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("missing bearer token")]
    Unauthorized,

    #[error("bearer token rejected")]
    Forbidden,

    #[error("resource not found: {0}")]
    NotFound(String),

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("upstream rate limit")]
    TooManyRequests,

    #[error("payload too large: {0}")]
    PayloadTooLarge(String),

    #[error("upstream timeout: {0}")]
    GatewayTimeout(String),

    #[error("bad gateway: {0}")]
    BadGateway(String),

    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),

    #[error("database error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("internal error: {0}")]
    Internal(String),
}

impl AppError {
    fn status_and_code(&self) -> (StatusCode, &'static str) {
        match self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized"),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "forbidden"),
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, "not_found"),
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, "bad_request"),
            AppError::Conflict(_) => (StatusCode::CONFLICT, "conflict"),
            AppError::TooManyRequests => (StatusCode::TOO_MANY_REQUESTS, "rate_limited"),
            AppError::PayloadTooLarge(_) => (StatusCode::PAYLOAD_TOO_LARGE, "payload_too_large"),
            AppError::GatewayTimeout(_) => (StatusCode::GATEWAY_TIMEOUT, "gateway_timeout"),
            AppError::BadGateway(_) => (StatusCode::BAD_GATEWAY, "bad_gateway"),
            AppError::Io(_) => (StatusCode::INTERNAL_SERVER_ERROR, "io_error"),
            AppError::Sqlite(_) => (StatusCode::INTERNAL_SERVER_ERROR, "db_error"),
            AppError::Serde(_) => (StatusCode::INTERNAL_SERVER_ERROR, "serde_error"),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal"),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code) = self.status_and_code();
        let body = Json(json!({
            "error": {
                "code": code,
                "message": self.to_string(),
            }
        }));
        (status, body).into_response()
    }
}
