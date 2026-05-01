//! GET /v1/models — list the (single) model the daemon advertises.
//!
//! We expose ONE model id, configurable via `KEI_MODEL_NAME` (default
//! `kei-cortex`). Multi-model exposure would imply provider routing at
//! the OpenAI surface, which is a Phase-2 concern — for now any frontend
//! sees one entry and uses it as the `model` field on chat-completions.

use super::error::OpenAiError;
use super::types::{ModelList, ModelObject};
use axum::Json;
use std::time::{SystemTime, UNIX_EPOCH};

/// Env var consulted at request time so a daemon restart isn't needed
/// when re-branding the surface.
const ENV_MODEL_NAME: &str = "KEI_MODEL_NAME";
/// Default model id when the env var is unset.
const DEFAULT_MODEL_NAME: &str = "kei-cortex";

/// Handler for `GET /v1/models`.
pub async fn list_models() -> Result<Json<ModelList>, OpenAiError> {
    let id = current_model_name();
    let created = unix_secs();
    let body = ModelList {
        object: "list",
        data: vec![ModelObject {
            id,
            object: "model",
            created,
            owned_by: "keisei",
        }],
    };
    Ok(Json(body))
}

/// Read `KEI_MODEL_NAME`, falling back to the default. Empty / whitespace
/// values are treated as unset.
pub fn current_model_name() -> String {
    std::env::var(ENV_MODEL_NAME)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| DEFAULT_MODEL_NAME.to_string())
}

/// Unix-time seconds, with a safe fallback to 0 instead of `unwrap()`.
fn unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_name_is_kei_cortex() {
        // Avoid mutating env in parallel tests — only assert the default
        // string matches when env is empty.
        let prev = std::env::var(ENV_MODEL_NAME).ok();
        std::env::remove_var(ENV_MODEL_NAME);
        assert_eq!(current_model_name(), DEFAULT_MODEL_NAME);
        if let Some(v) = prev {
            std::env::set_var(ENV_MODEL_NAME, v);
        }
    }

    #[tokio::test]
    async fn list_models_returns_one_entry() {
        let resp = list_models().await.unwrap();
        assert_eq!(resp.0.data.len(), 1);
        assert_eq!(resp.0.object, "list");
    }
}
