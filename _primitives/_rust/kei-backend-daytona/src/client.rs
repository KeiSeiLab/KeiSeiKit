//! Thin async REST client for Daytona.
//!
//! No Rust SDK exists upstream → we hit the public REST surface directly.
//! All CRUD calls are timeout-bounded; 429/503 are retried with exponential
//! backoff (max 3 retries, base 250ms).
//!
//! Architecture note — two API surfaces:
//!   1. Management API  (`base_url`)   — sandbox CRUD (`/sandbox/...`).
//!   2. Toolbox API     (per-sandbox)  — exec and file I/O.
//!      Base URL fetched via `GET /sandbox/{id}/toolbox-proxy-url`,
//!      cached per sandbox in `toolbox_cache`. See `toolbox.rs`.

use crate::error::{DaytonaError, Result};
use crate::toolbox::{self, ToolboxCache};
use crate::types::{CreateSandboxSpec, ExecOutput, Sandbox};
use reqwest::{Client, Method, RequestBuilder, Response, StatusCode};
use serde_json::Value;
use std::time::Duration;

const DEFAULT_TIMEOUT_SECS: u64 = 30;
const MAX_RETRIES: u32 = 3;
const RETRY_BASE_MS: u64 = 250;

/// Async Daytona REST client.
#[derive(Debug, Clone)]
pub struct DaytonaClient {
    http: Client,
    base_url: String,
    api_key: String,
    toolbox_cache: ToolboxCache,
}

impl DaytonaClient {
    /// Build a new client with the default 30s per-request timeout.
    pub fn new(api_key: impl Into<String>, base_url: impl Into<String>) -> Result<Self> {
        Self::with_timeout(api_key, base_url, Duration::from_secs(DEFAULT_TIMEOUT_SECS))
    }

    /// Build a client with a custom per-request timeout.
    pub fn with_timeout(
        api_key: impl Into<String>,
        base_url: impl Into<String>,
        timeout: Duration,
    ) -> Result<Self> {
        let http = Client::builder()
            .timeout(timeout)
            .build()
            .map_err(DaytonaError::from)?;
        Ok(Self {
            http,
            base_url: base_url.into(),
            api_key: api_key.into(),
            toolbox_cache: ToolboxCache::new(),
        })
    }

    /// GET /sandbox/{name} — returns `None` on 404, otherwise the sandbox.
    pub async fn get_sandbox(&self, name: &str) -> Result<Option<Sandbox>> {
        let url = format!("{}/sandbox/{}", self.base_url, name);
        match self.send(self.req(Method::GET, &url)).await {
            Ok(resp) => Ok(Some(parse_json(resp).await?)),
            Err(DaytonaError::NotFound(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// GET /sandbox — enumerate all sandboxes for cost-guard quota counting.
    /// Empty array on 404 (legacy endpoint may not exist on older instances).
    pub async fn list_sandboxes(&self) -> Result<Vec<Sandbox>> {
        let url = format!("{}/sandbox", self.base_url);
        match self.send(self.req(Method::GET, &url)).await {
            Ok(resp) => Ok(parse_json(resp).await?),
            Err(DaytonaError::NotFound(_)) => Ok(Vec::new()),
            Err(e) => Err(e),
        }
    }

    /// POST /sandbox — create a new sandbox from `spec`.
    pub async fn create_sandbox(&self, spec: &CreateSandboxSpec) -> Result<Sandbox> {
        let url = format!("{}/sandbox", self.base_url);
        let resp = self.send(self.req(Method::POST, &url).json(spec)).await?;
        parse_json(resp).await
    }

    /// POST /sandbox/{name}/start — resume a stopped/hibernated sandbox.
    pub async fn start_sandbox(&self, name: &str) -> Result<()> {
        let url = format!("{}/sandbox/{}/start", self.base_url, name);
        self.send(self.req(Method::POST, &url)).await.map(drop)
    }

    /// POST /sandbox/{name}/stop — preserve filesystem.
    pub async fn stop_sandbox(&self, name: &str) -> Result<()> {
        let url = format!("{}/sandbox/{}/stop", self.base_url, name);
        self.send(self.req(Method::POST, &url)).await.map(drop)
    }

    /// DELETE /sandbox/{name} — destroy filesystem too.
    pub async fn delete_sandbox(&self, name: &str) -> Result<()> {
        let url = format!("{}/sandbox/{}", self.base_url, name);
        self.send(self.req(Method::DELETE, &url)).await.map(drop)
    }

    /// Execute a command inside the sandbox via the Toolbox API.
    ///
    /// Resolves the toolbox base URL via `GET /sandbox/{id}/toolbox-proxy-url`,
    /// then `POST <toolbox_base>/toolbox/{id}/toolbox/process/execute`.
    /// Body field `command` confirmed from `ExecuteRequest` schema in spec.
    pub async fn exec(&self, sandbox_id: &str, cmd: &str) -> Result<ExecOutput> {
        let tb = self.toolbox_base(sandbox_id).await?;
        toolbox::exec(&self.http, &self.api_key, &tb, sandbox_id, cmd).await
    }

    /// Upload a file to the sandbox via the Toolbox API.
    ///
    /// `POST <toolbox_base>/toolbox/{id}/toolbox/files/upload?path=<remote_path>`
    /// with `multipart/form-data` field `file`.
    pub async fn upload_file(&self, sandbox_id: &str, remote_path: &str, bytes: Vec<u8>) -> Result<()> {
        let tb = self.toolbox_base(sandbox_id).await?;
        toolbox::upload_file(&self.http, &self.api_key, &tb, sandbox_id, remote_path, bytes).await
    }

    /// Download a file from the sandbox via the Toolbox API.
    ///
    /// `GET <toolbox_base>/toolbox/{id}/toolbox/files/download?path=<remote_path>`
    pub async fn download_file(&self, sandbox_id: &str, remote_path: &str) -> Result<Vec<u8>> {
        let tb = self.toolbox_base(sandbox_id).await?;
        toolbox::download_file(&self.http, &self.api_key, &tb, sandbox_id, remote_path).await
    }

    /// Resolve the toolbox proxy URL for `sandbox_id` (cached after first fetch).
    async fn toolbox_base(&self, sandbox_id: &str) -> Result<String> {
        toolbox::toolbox_url_for(
            &self.http,
            &self.api_key,
            &self.base_url,
            &self.toolbox_cache,
            sandbox_id,
        )
        .await
    }

    /// Build a request with bearer auth + JSON accept.
    fn req(&self, method: Method, url: &str) -> RequestBuilder {
        self.http
            .request(method, url)
            .bearer_auth(&self.api_key)
            .header("accept", "application/json")
    }

    /// Send with retry on 429/503.
    async fn send(&self, builder: RequestBuilder) -> Result<Response> {
        let mut attempt: u32 = 0;
        loop {
            let cloned = builder.try_clone().ok_or_else(|| {
                DaytonaError::Unknown("non-cloneable request body".into())
            })?;
            let res = cloned.send().await.map_err(DaytonaError::from);
            match map_response(res).await {
                Ok(resp) => return Ok(resp),
                Err(e) if is_retriable(&e) && attempt < MAX_RETRIES => {
                    let backoff = RETRY_BASE_MS * (1 << attempt);
                    tokio::time::sleep(Duration::from_millis(backoff)).await;
                    attempt += 1;
                }
                Err(e) => return Err(e),
            }
        }
    }
}

async fn map_response(r: Result<Response>) -> Result<Response> {
    let resp = r?;
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

fn is_retriable(e: &DaytonaError) -> bool {
    matches!(e, DaytonaError::RateLimited(_))
}

async fn parse_json<T: serde::de::DeserializeOwned>(resp: Response) -> Result<T> {
    let bytes = resp.bytes().await.map_err(DaytonaError::from)?;
    if bytes.is_empty() {
        return serde_json::from_value(Value::Null).map_err(DaytonaError::from);
    }
    serde_json::from_slice(&bytes).map_err(DaytonaError::from)
}
