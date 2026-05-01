//! Anthropic Messages API provider. Streams `text_delta` events.
//! Cost 100/500 cents/MTok (VERIFIED 2026-04 anthropic.com/pricing — Haiku 4.5: $1/$5 per MTok).

use async_stream::try_stream;
use async_trait::async_trait;
use bytes::Bytes;
use futures::stream::{BoxStream, StreamExt};
use kei_model::Registry;
use serde::Deserialize;
use std::sync::OnceLock;
use std::time::Duration;
use tokio::time::timeout;

use crate::provider::{Error, Message, Provider, StreamEvent, Tool};
use crate::providers::sse::SseParser;

pub const NAME: &str = "anthropic";
pub const ENDPOINT: &str = "https://api.anthropic.com/v1/messages";
pub const API_VERSION: &str = "2023-06-01";
/// Last-resort fallback if env + kei-model registry both miss (W55 Risk #1).
pub const LEGACY_DEFAULT_MODEL: &str = "claude-haiku-4-5";
/// Hardcoded fallback prices in cents/MTok (M-1: kept for offline/registry-miss case).
/// Verified 2026-04 anthropic.com/pricing — Haiku 4.5: $1/$5 per MTok.
const FALLBACK_INPUT_CENTS: u32 = 100;
const FALLBACK_OUTPUT_CENTS: u32 = 500;
/// 3-tier resolve: `ANTHROPIC_MODEL` env → registry role `kei-router-anthropic` → legacy.
pub fn default_model() -> String {
    std::env::var("ANTHROPIC_MODEL")
        .ok()
        .or_else(|| registry_lookup("kei-router-anthropic"))
        .unwrap_or_else(|| LEGACY_DEFAULT_MODEL.into())
}
fn registry_lookup(role: &str) -> Option<String> {
    let registry = cached_registry()?;
    Some(kei_model::resolve(role, None, &[], registry, None).ok()?.model.id)
}

/// Cache the kei-model registry once across all cost-method calls (M-1).
/// `None` if registry path can't be resolved or load fails.
fn cached_registry() -> Option<&'static Registry> {
    static CACHE: OnceLock<Option<Registry>> = OnceLock::new();
    CACHE.get_or_init(|| {
        let path = Registry::resolve_path(None).ok()?;
        Registry::load(&path).ok()
    })
    .as_ref()
}

/// M-1: derive cents/MTok from kei-model registry. Falls back to hardcoded
/// `fallback` when registry missing, model unknown, or pricing is placeholder.
fn input_cents_from_registry(model_id: &str, fallback: u32) -> u32 {
    let micro = cached_registry()
        .and_then(|r| r.get(model_id))
        .map(|m| m.pricing.input_per_mtok_micro)
        .unwrap_or(0);
    if micro == 0 { fallback } else { (micro / 1_000_000) as u32 }
}

fn output_cents_from_registry(model_id: &str, fallback: u32) -> u32 {
    let micro = cached_registry()
        .and_then(|r| r.get(model_id))
        .map(|m| m.pricing.output_per_mtok_micro)
        .unwrap_or(0);
    if micro == 0 { fallback } else { (micro / 1_000_000) as u32 }
}
const HANDSHAKE_BUDGET: Duration = Duration::from_secs(60);
const IDLE_BUDGET: Duration = Duration::from_secs(60);
const BODY_PREVIEW_CAP: usize = 512;

pub struct AnthropicProvider {
    api_key: String,
    model: String,
    endpoint: String,
}

impl AnthropicProvider {
    /// Construct from env (`ANTHROPIC_API_KEY`). Returns `None` if unset.
    pub fn from_env() -> Option<Self> {
        std::env::var("ANTHROPIC_API_KEY").ok().map(|api_key| Self {
            api_key,
            model: default_model(),
            endpoint: ENDPOINT.into(),
        })
    }

    /// For tests: explicit endpoint override.
    pub fn with_endpoint(api_key: String, model: String, endpoint: String) -> Self {
        Self { api_key, model, endpoint }
    }
}

#[async_trait]
impl Provider for AnthropicProvider {
    fn name(&self) -> &'static str { NAME }
    fn cost_per_m_tok_input_cents(&self) -> u32 {
        input_cents_from_registry(&self.model, FALLBACK_INPUT_CENTS)
    }
    fn cost_per_m_tok_output_cents(&self) -> u32 {
        output_cents_from_registry(&self.model, FALLBACK_OUTPUT_CENTS)
    }

    async fn stream_message(
        &self,
        system: &str,
        messages: &[Message],
        _tools: Option<&[Tool]>,
    ) -> Result<BoxStream<'static, Result<StreamEvent, Error>>, Error> {
        let body = build_body(&self.model, system, messages);
        let resp = match timeout(HANDSHAKE_BUDGET, send(&self.endpoint, &self.api_key, &body)).await {
            Ok(r) => r?,
            Err(_) => return Err(Error::Timeout(NAME)),
        };
        Ok(Box::pin(stream_events(resp)))
    }
}

fn build_body(model: &str, system: &str, messages: &[Message]) -> serde_json::Value {
    serde_json::json!({
        "model": model,
        "max_tokens": 1024,
        "system": system,
        "stream": true,
        "messages": messages,
    })
}

async fn send(endpoint: &str, api_key: &str, body: &serde_json::Value) -> Result<reqwest::Response, Error> {
    let resp = reqwest::Client::new()
        .post(endpoint)
        .header("x-api-key", api_key)
        .header("anthropic-version", API_VERSION)
        .header("content-type", "application/json")
        .json(body)
        .send()
        .await?;
    check_status(resp).await
}

async fn check_status(resp: reqwest::Response) -> Result<reqwest::Response, Error> {
    let status = resp.status();
    if status.is_success() {
        return Ok(resp);
    }
    let code = status.as_u16();
    if code == 429 { return Err(Error::RateLimit(NAME)); }
    if code == 503 || code == 529 { return Err(Error::ServiceUnavailable(NAME)); }
    let body = resp.text().await.unwrap_or_default();
    Err(Error::Upstream { provider: NAME, status: code, body: truncate(&body, BODY_PREVIEW_CAP) })
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max { return s.to_string(); }
    let mut end = max;
    while end > 0 && !s.is_char_boundary(end) { end -= 1; }
    s[..end].to_string()
}

fn stream_events(resp: reqwest::Response) -> impl futures::Stream<Item = Result<StreamEvent, Error>> + Send + 'static {
    try_stream! {
        let mut parser = SseParser::new();
        let mut bytes_stream = resp.bytes_stream();
        let mut closed = false;
        while !closed {
            let next = timeout(IDLE_BUDGET, bytes_stream.next()).await;
            let chunk_opt = match next { Ok(x) => x, Err(_) => Err(Error::Timeout(NAME))? };
            let Some(chunk) = chunk_opt else { break };
            let chunk: Bytes = chunk.map_err(Error::Http)?;
            for payload in parser.push(&chunk) {
                if let Some(ev) = decode_event(&payload) {
                    if matches!(ev, StreamEvent::Done) { closed = true; }
                    yield ev;
                }
            }
        }
    }
}

#[derive(Debug, Deserialize)]
struct Envelope {
    #[serde(rename = "type", default)]
    kind: Option<String>,
    #[serde(default)]
    delta: Option<Delta>,
}

#[derive(Debug, Deserialize)]
struct Delta {
    #[serde(rename = "type", default)]
    kind: Option<String>,
    #[serde(default)]
    text: Option<String>,
}

fn decode_event(payload: &str) -> Option<StreamEvent> {
    let env: Envelope = serde_json::from_str(payload).ok()?;
    if env.kind.as_deref() == Some("message_stop") {
        return Some(StreamEvent::Done);
    }
    let d = env.delta?;
    if d.kind.as_deref() != Some("text_delta") { return None; }
    Some(StreamEvent::Token(d.text?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_text_delta() {
        let payload = r#"{"type":"content_block_delta","delta":{"type":"text_delta","text":"hi"}}"#;
        assert!(matches!(decode_event(payload), Some(StreamEvent::Token(t)) if t == "hi"));
    }

    #[test]
    fn decodes_message_stop() {
        let payload = r#"{"type":"message_stop"}"#;
        assert!(matches!(decode_event(payload), Some(StreamEvent::Done)));
    }

    #[test]
    fn ignores_ping() {
        let payload = r#"{"type":"ping"}"#;
        assert!(decode_event(payload).is_none());
    }

    #[test]
    fn default_model_falls_back_to_legacy_when_no_env_no_registry() {
        std::env::remove_var("ANTHROPIC_MODEL");
        std::env::set_var("KEI_MODEL_REGISTRY", "/nonexistent/kei-model-registry-path.toml");
        assert_eq!(default_model(), LEGACY_DEFAULT_MODEL);
        std::env::remove_var("KEI_MODEL_REGISTRY");
    }

    #[test]
    fn cost_falls_back_when_model_unknown() {
        // Unknown model → hardcoded fallback values returned
        assert_eq!(
            input_cents_from_registry("definitely-nonexistent-model-xyz", FALLBACK_INPUT_CENTS),
            FALLBACK_INPUT_CENTS
        );
        assert_eq!(
            output_cents_from_registry("definitely-nonexistent-model-xyz", FALLBACK_OUTPUT_CENTS),
            FALLBACK_OUTPUT_CENTS
        );
    }
}
