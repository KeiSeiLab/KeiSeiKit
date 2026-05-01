//! OpenAI-style error envelope `{ "error": { message, type, code } }`.
//!
//! Local to the `/v1/*` surface so we can match the OpenAI wire format
//! exactly without leaking it into the existing kei-cortex `AppError`
//! (which uses the `{ "error": { code, message } }` shape).

use super::types::{OpenAiErrorBody, OpenAiErrorInner};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

/// Local error type for `/v1/*` handlers. Each variant becomes an
/// `OpenAiErrorBody` with status, type and code chosen to match the
/// OpenAI public reference.
#[derive(Debug, thiserror::Error)]
pub enum OpenAiError {
    #[error("missing or invalid Authorization header")]
    Unauthorized,

    #[error("forbidden")]
    Forbidden,

    #[error("not found: {0}")]
    NotFound(String),

    #[error("invalid request: {0}")]
    BadRequest(String),

    #[error("upstream error: {0}")]
    Upstream(String),

    #[error("internal server error: {0}")]
    Internal(String),
}

impl OpenAiError {
    fn parts(&self) -> (StatusCode, &'static str, &'static str) {
        match self {
            OpenAiError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, "invalid_request_error", "unauthorized")
            }
            OpenAiError::Forbidden => {
                (StatusCode::FORBIDDEN, "invalid_request_error", "forbidden")
            }
            OpenAiError::NotFound(_) => {
                (StatusCode::NOT_FOUND, "invalid_request_error", "not_found")
            }
            OpenAiError::BadRequest(_) => {
                (StatusCode::BAD_REQUEST, "invalid_request_error", "bad_request")
            }
            OpenAiError::Upstream(_) => {
                (StatusCode::BAD_GATEWAY, "api_error", "upstream_error")
            }
            OpenAiError::Internal(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "api_error", "internal")
            }
        }
    }
}

impl IntoResponse for OpenAiError {
    fn into_response(self) -> Response {
        let (status, kind, code) = self.parts();
        let body = Json(OpenAiErrorBody {
            error: OpenAiErrorInner {
                message: self.to_string(),
                kind,
                code,
            },
        });
        (status, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unauthorized_maps_to_401_invalid_request() {
        let (s, k, c) = OpenAiError::Unauthorized.parts();
        assert_eq!(s, StatusCode::UNAUTHORIZED);
        assert_eq!(k, "invalid_request_error");
        assert_eq!(c, "unauthorized");
    }

    #[test]
    fn bad_request_includes_message_in_display() {
        let e = OpenAiError::BadRequest("missing model".into());
        assert!(e.to_string().contains("missing model"));
    }
}
