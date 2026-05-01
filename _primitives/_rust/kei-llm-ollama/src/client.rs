//! HTTP client wrapping the Ollama daemon.
//!
//! Default base URL is `http://127.0.0.1:11434` — local-only by design.
//! See `<https://github.com/ollama/ollama/blob/main/docs/api.md>` for schema.

use std::time::Duration;

use bytes::Bytes;
use futures::stream::Stream;

use crate::api::{ChatReq, ChatResp, GenerateReq, GenerateResp, TagsResp, VersionResp};
use crate::error::{classify_reqwest_error, ApiError};
use crate::http_io::{check_status, decode_json_or_err};
use crate::stream::{chunk_stream, Chunk};

pub const DEFAULT_BASE_URL: &str = "http://127.0.0.1:11434";
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(60);

/// Thin wrapper around `reqwest::Client` aimed at the Ollama daemon.
#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
    http: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self::new(DEFAULT_BASE_URL)
    }
}

impl Client {
    /// New client. `base_url` should be `http://host:port` (no trailing slash).
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            http: reqwest::Client::new(),
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// `GET /api/tags` — list installed models.
    pub async fn tags(&self) -> Result<TagsResp, ApiError> {
        self.tags_with_timeout(DEFAULT_TIMEOUT).await
    }

    /// `GET /api/tags` with explicit per-call timeout.
    pub async fn tags_with_timeout(&self, t: Duration) -> Result<TagsResp, ApiError> {
        self.get_json("/api/tags", t).await
    }

    /// `GET /api/version` with explicit per-call timeout (used by health probe).
    pub async fn version_with_timeout(&self, t: Duration) -> Result<VersionResp, ApiError> {
        self.get_json("/api/version", t).await
    }

    /// `POST /api/generate` — non-streaming.
    pub async fn generate(&self, req: &GenerateReq) -> Result<GenerateResp, ApiError> {
        let mut body = to_value(req)?;
        body["stream"] = serde_json::Value::Bool(false);
        self.post_json("/api/generate", &body, DEFAULT_TIMEOUT).await
    }

    /// `POST /api/chat` — non-streaming.
    pub async fn chat(&self, req: &ChatReq) -> Result<ChatResp, ApiError> {
        let mut body = to_value(req)?;
        body["stream"] = serde_json::Value::Bool(false);
        self.post_json("/api/chat", &body, DEFAULT_TIMEOUT).await
    }

    /// `POST /api/generate` — streaming. No timeout (Ollama generation can be slow).
    pub async fn generate_stream(
        &self,
        req: &GenerateReq,
    ) -> Result<impl Stream<Item = Result<Chunk, ApiError>> + Send + 'static, ApiError> {
        let mut body = to_value(req)?;
        body["stream"] = serde_json::Value::Bool(true);
        self.open_stream("/api/generate", &body).await
    }

    /// `POST /api/chat` — streaming. No timeout.
    pub async fn chat_stream(
        &self,
        req: &ChatReq,
    ) -> Result<impl Stream<Item = Result<Chunk, ApiError>> + Send + 'static, ApiError> {
        let mut body = to_value(req)?;
        body["stream"] = serde_json::Value::Bool(true);
        self.open_stream("/api/chat", &body).await
    }

    /// `POST /api/pull` — model download. Returns raw NDJSON bytes-stream.
    pub async fn pull_stream(
        &self,
        model: &str,
    ) -> Result<impl Stream<Item = Result<Bytes, reqwest::Error>> + Send + 'static, ApiError> {
        let url = format!("{}/api/pull", self.base_url);
        let body = serde_json::json!({ "name": model, "stream": true });
        let resp = self
            .http
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| classify_reqwest_error(e, &url, 0))?;
        check_status(&resp)?;
        Ok(resp.bytes_stream())
    }

    /// `DELETE /api/delete` — remove an installed model.
    pub async fn delete(&self, model: &str) -> Result<(), ApiError> {
        let url = format!("{}/api/delete", self.base_url);
        let body = serde_json::json!({ "name": model });
        let resp = self
            .http
            .delete(&url)
            .timeout(DEFAULT_TIMEOUT)
            .json(&body)
            .send()
            .await
            .map_err(|e| classify_reqwest_error(e, &url, DEFAULT_TIMEOUT.as_millis() as u64))?;
        check_status(&resp)
    }

    /// `POST /api/show` — model details (raw JSON value).
    pub async fn show(&self, model: &str) -> Result<serde_json::Value, ApiError> {
        let body = serde_json::json!({ "name": model });
        self.post_json("/api/show", &body, DEFAULT_TIMEOUT).await
    }

    async fn get_json<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        t: Duration,
    ) -> Result<T, ApiError> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .get(&url)
            .timeout(t)
            .send()
            .await
            .map_err(|e| classify_reqwest_error(e, &url, t.as_millis() as u64))?;
        decode_json_or_err(resp).await
    }

    async fn post_json<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &serde_json::Value,
        t: Duration,
    ) -> Result<T, ApiError> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .post(&url)
            .timeout(t)
            .json(body)
            .send()
            .await
            .map_err(|e| classify_reqwest_error(e, &url, t.as_millis() as u64))?;
        decode_json_or_err(resp).await
    }

    async fn open_stream(
        &self,
        path: &str,
        body: &serde_json::Value,
    ) -> Result<impl Stream<Item = Result<Chunk, ApiError>> + Send + 'static, ApiError> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .post(&url)
            .json(body)
            .send()
            .await
            .map_err(|e| classify_reqwest_error(e, &url, 0))?;
        check_status(&resp)?;
        Ok(chunk_stream(resp.bytes_stream()))
    }
}

fn to_value<T: serde::Serialize>(req: &T) -> Result<serde_json::Value, ApiError> {
    serde_json::to_value(req).map_err(|e| ApiError::DecodeError(e.to_string()))
}
