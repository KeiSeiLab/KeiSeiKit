//! Toolbox API helpers — exec, file upload/download via the per-sandbox proxy.
//!
//! The Daytona Toolbox API is a separate HTTP surface from the management API.
//! Its base URL is fetched lazily via `GET /sandbox/{id}/toolbox-proxy-url`
//! (management API) and cached per sandbox ID.
//!
//! Spec endpoints used:
//!   POST <toolbox_base>/toolbox/{id}/toolbox/process/execute
//!   POST <toolbox_base>/toolbox/{id}/toolbox/files/upload?path=<p>
//!   GET  <toolbox_base>/toolbox/{id}/toolbox/files/download?path=<p>

use crate::error::{DaytonaError, Result};
use crate::types::{ExecOutput, ExecResponse};
use reqwest::{Client, Method, Response, StatusCode};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Response schema for `GET /sandbox/{id}/toolbox-proxy-url`.
#[derive(Deserialize)]
pub(crate) struct ToolboxProxyUrl {
    pub url: String,
}

/// Per-sandbox toolbox URL cache.
#[derive(Debug, Clone, Default)]
pub(crate) struct ToolboxCache(pub Arc<Mutex<HashMap<String, String>>>);

impl ToolboxCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, id: &str) -> Option<String> {
        self.0.lock().unwrap().get(id).cloned()
    }

    pub fn set(&self, id: &str, url: String) {
        self.0.lock().unwrap().insert(id.to_string(), url);
    }
}

/// Resolve the toolbox base URL for `sandbox_id`.
///
/// Checks the cache first; on miss fetches from the management API.
pub(crate) async fn toolbox_url_for(
    http: &Client,
    api_key: &str,
    management_base: &str,
    cache: &ToolboxCache,
    sandbox_id: &str,
) -> Result<String> {
    if let Some(cached) = cache.get(sandbox_id) {
        return Ok(cached);
    }
    let url = format!("{}/sandbox/{}/toolbox-proxy-url", management_base, sandbox_id);
    let resp = http
        .request(Method::GET, &url)
        .bearer_auth(api_key)
        .header("accept", "application/json")
        .send()
        .await
        .map_err(DaytonaError::from)?;
    let resp = map_status(resp).await?;
    let proxy: ToolboxProxyUrl = parse_json(resp).await?;
    cache.set(sandbox_id, proxy.url.clone());
    Ok(proxy.url)
}

/// Execute a command in the sandbox via the Toolbox API.
///
/// `POST <toolbox_base>/toolbox/{id}/toolbox/process/execute`
/// with `ExecuteRequest { command }`.
pub(crate) async fn exec(
    http: &Client,
    api_key: &str,
    toolbox_base: &str,
    sandbox_id: &str,
    cmd: &str,
) -> Result<ExecOutput> {
    let url = format!(
        "{}/toolbox/{}/toolbox/process/execute",
        toolbox_base, sandbox_id
    );
    let body = serde_json::json!({ "command": cmd });
    let resp = http
        .request(Method::POST, &url)
        .bearer_auth(api_key)
        .header("accept", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(DaytonaError::from)?;
    let resp = map_status(resp).await?;
    let parsed: ExecResponse = parse_json(resp).await?;
    Ok(parsed.into())
}

/// Upload a file to the sandbox via the Toolbox API.
///
/// `POST <toolbox_base>/toolbox/{id}/toolbox/files/upload?path=<remote_path>`
/// Body: `multipart/form-data` with field `file` = file bytes.
///
/// Multipart bodies are not cloneable so this call is not retried.
pub(crate) async fn upload_file(
    http: &Client,
    api_key: &str,
    toolbox_base: &str,
    sandbox_id: &str,
    remote_path: &str,
    bytes: Vec<u8>,
) -> Result<()> {
    let url = format!(
        "{}/toolbox/{}/toolbox/files/upload",
        toolbox_base, sandbox_id
    );
    let part = reqwest::multipart::Part::bytes(bytes).file_name("file");
    let form = reqwest::multipart::Form::new().part("file", part);
    let resp = http
        .request(Method::POST, &url)
        .bearer_auth(api_key)
        .query(&[("path", remote_path)])
        .multipart(form)
        .send()
        .await
        .map_err(DaytonaError::from)?;
    map_status(resp).await.map(drop)
}

/// Download a file from the sandbox via the Toolbox API.
///
/// `GET <toolbox_base>/toolbox/{id}/toolbox/files/download?path=<remote_path>`
pub(crate) async fn download_file(
    http: &Client,
    api_key: &str,
    toolbox_base: &str,
    sandbox_id: &str,
    remote_path: &str,
) -> Result<Vec<u8>> {
    let url = format!(
        "{}/toolbox/{}/toolbox/files/download",
        toolbox_base, sandbox_id
    );
    let resp = http
        .request(Method::GET, &url)
        .bearer_auth(api_key)
        .header("accept", "application/json")
        .query(&[("path", remote_path)])
        .send()
        .await
        .map_err(DaytonaError::from)?;
    let resp = map_status(resp).await?;
    let b = resp.bytes().await.map_err(DaytonaError::from)?;
    Ok(b.to_vec())
}

/// Check HTTP status and convert errors.
async fn map_status(resp: Response) -> Result<Response> {
    let status = resp.status();
    if status.is_success() {
        return Ok(resp);
    }
    let body = resp.text().await.unwrap_or_default();
    Err(classify(status, body))
}

fn classify(status: StatusCode, body: String) -> DaytonaError {
    match status.as_u16() {
        401 | 403 => DaytonaError::Auth(body),
        404 => DaytonaError::NotFound(body),
        429 | 503 => DaytonaError::RateLimited(body),
        _ => DaytonaError::Unknown(format!("http {}: {}", status, body)),
    }
}

async fn parse_json<T: serde::de::DeserializeOwned>(resp: Response) -> Result<T> {
    let bytes = resp.bytes().await.map_err(DaytonaError::from)?;
    if bytes.is_empty() {
        let v: Value = Value::Null;
        return serde_json::from_value(v).map_err(DaytonaError::from);
    }
    serde_json::from_slice(&bytes).map_err(DaytonaError::from)
}
