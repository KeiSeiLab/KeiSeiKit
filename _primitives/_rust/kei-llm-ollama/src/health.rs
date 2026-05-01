//! Liveness probe for the Ollama daemon.
//!
//! Used by the W60 router to decide whether to route to the Ollama backend
//! or fall back to llama.cpp / mlx. Short timeout (1s default) — never blocks
//! the parent for long.

use std::time::Duration;

use crate::api::VersionResp;
use crate::client::Client;
use crate::error::ApiError;

/// Quick `is the daemon up?` probe. Returns `true` on 2xx /api/tags within timeout.
pub async fn is_running(client: &Client) -> bool {
    client
        .tags_with_timeout(Duration::from_millis(1000))
        .await
        .is_ok()
}

/// Full health snapshot — version + model count.
#[derive(Debug, Clone, serde::Serialize)]
pub struct HealthSnapshot {
    pub running: bool,
    pub version: Option<String>,
    pub models_count: Option<usize>,
}

/// Fetch a complete health snapshot. `running=false` if either probe fails.
pub async fn snapshot(client: &Client) -> HealthSnapshot {
    let timeout = Duration::from_millis(1500);
    let version = fetch_version(client, timeout).await.ok();
    let models_count = client
        .tags_with_timeout(timeout)
        .await
        .ok()
        .map(|t| t.models.len());
    HealthSnapshot {
        running: version.is_some() || models_count.is_some(),
        version: version.map(|v| v.version),
        models_count,
    }
}

async fn fetch_version(client: &Client, timeout: Duration) -> Result<VersionResp, ApiError> {
    client.version_with_timeout(timeout).await
}
