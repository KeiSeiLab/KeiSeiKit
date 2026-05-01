//! Low-level HTTP helpers shared by `Client`.
//!
//! Keeps `client.rs` under the Constructor-Pattern 200-LOC limit by hosting
//! the per-call decode + status-check primitives here.

use serde::de::DeserializeOwned;

use crate::error::ApiError;

/// Read body, decode JSON, otherwise translate to ApiError.
pub async fn decode_json_or_err<T: DeserializeOwned>(resp: reqwest::Response) -> Result<T, ApiError> {
    check_status(&resp)?;
    let bytes = resp
        .bytes()
        .await
        .map_err(|e| ApiError::Transport(e.to_string()))?;
    serde_json::from_slice::<T>(&bytes).map_err(|e| ApiError::DecodeError(e.to_string()))
}

/// Map non-2xx status to `ApiError`. 404 → `ModelNotFound` (caller path is hint).
pub fn check_status(resp: &reqwest::Response) -> Result<(), ApiError> {
    let status = resp.status();
    if status.is_success() {
        return Ok(());
    }
    if status.as_u16() == 404 {
        return Err(ApiError::ModelNotFound(
            resp.url().path().trim_start_matches('/').to_string(),
        ));
    }
    Err(ApiError::HttpError {
        status: status.as_u16(),
        body: format!("status {} from {}", status, resp.url()),
    })
}
