//! Moonshot Kimi K2 provider (OpenAI-compatible). Reads `KIMI_API_KEY` (or `MOONSHOT_API_KEY`).
//! Cost 60/250 cents/MTok (VERIFIED 2026-04 fal.ai + openrouter.ai — Kimi K2 Thinking $0.60/$2.50 per MTok).

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

pub const NAME: &str = "kimi";
pub const ENDPOINT: &str = "https://api.moonshot.ai/v1/chat/completions";
/// Last-resort fallback if env + kei-model registry both miss (W55 Risk #1).
pub const LEGACY_DEFAULT_MODEL: &str = "kimi-k2-thinking";
/// Hardcoded fallback prices in cents/MTok (M-1: kept for offline/registry-miss case).
/// VERIFIED 2026-04 fal.ai + openrouter.ai — Kimi K2 Thinking $0.60/$2.50 per MTok.
const FALLBACK_INPUT_CENTS: u32 = 60;
const FALLBACK_OUTPUT_CENTS: u32 = 250;
/// 3-tier resolve: `KIMI_MODEL` env → registry role `kei-router-kimi` → legacy.
pub fn default_model() -> String {
    std::env::var("KIMI_MODEL")
        .ok()
        .or_else(|| registry_lookup("kei-router-kimi"))
        .unwrap_or_else(|| LEGACY_DEFAULT_MODEL.into())
}
fn registry_lookup(role: &str) -> Option<String> {
    let registry = cached_registry()?;
    Some(kei_model::resolve(role, None, &[], registry, None).ok()?.model.id)
}

/// Cache the kei-model registry once across all cost-method calls (M-1).
fn cached_registry() -> Option<&'static Registry> {
    static CACHE: OnceLock<Option<Registry>> = OnceLock::new();
    CACHE.get_or_init(|| {
        let path = Registry::resolve_path(None).ok()?;
        Registry::load(&path).ok()
    })
    .as_ref()
}

/// M-1: derive cents/MTok from kei-model registry. Falls back when missing/placeholder.
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

pub struct KimiProvider {
    api_key: String,
    model: String,
    endpoint: String,
}

impl KimiProvider {
    pub fn from_env() -> Option<Self> {
        let key = std::env::var("KIMI_API_KEY")
            .or_else(|_| std::env::var("MOONSHOT_API_KEY"))
            .ok()?;
        Some(Self { api_key: key, model: default_model(), endpoint: ENDPOINT.into() })
    }

    pub fn with_endpoint(api_key: String, model: String, endpoint: String) -> Self {
        Self { api_key, model, endpoint }
    }
}

#[async_trait]
impl Provider for KimiProvider {
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
    let mut full: Vec<serde_json::Value> = Vec::with_capacity(messages.len() + 1);
    if !system.is_empty() {
        full.push(serde_json::json!({ "role": "system", "content": system }));
    }
    for m in messages {
        full.push(serde_json::json!({ "role": m.role, "content": m.content }));
    }
    serde_json::json!({
        "model": model,
        "stream": true,
        "messages": full,
    })
}

async fn send(endpoint: &str, api_key: &str, body: &serde_json::Value) -> Result<reqwest::Response, Error> {
    let resp = reqwest::Client::new()
        .post(endpoint)
        .header("authorization", format!("Bearer {}", api_key))
        .header("content-type", "application/json")
        .json(body)
        .send()
        .await?;
    check_status(resp).await
}

async fn check_status(resp: reqwest::Response) -> Result<reqwest::Response, Error> {
    let status = resp.status();
    if status.is_success() { return Ok(resp); }
    let code = status.as_u16();
    if code == 429 { return Err(Error::RateLimit(NAME)); }
    if code == 503 || code == 502 { return Err(Error::ServiceUnavailable(NAME)); }
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
                if payload == "[DONE]" {
                    closed = true;
                    yield StreamEvent::Done;
                    break;
                }
                if let Some(ev) = decode_event(&payload) {
                    yield ev;
                }
            }
        }
    }
}

#[derive(Debug, Deserialize)]
struct ChatChunk {
    #[serde(default)]
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    #[serde(default)]
    delta: ChoiceDelta,
}

#[derive(Debug, Default, Deserialize)]
struct ChoiceDelta {
    #[serde(default)]
    content: Option<String>,
}

fn decode_event(payload: &str) -> Option<StreamEvent> {
    let chunk: ChatChunk = serde_json::from_str(payload).ok()?;
    let first = chunk.choices.into_iter().next()?;
    let text = first.delta.content?;
    if text.is_empty() { return None; }
    Some(StreamEvent::Token(text))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_content_delta() {
        let payload = r#"{"choices":[{"delta":{"content":"k2 says hi"}}]}"#;
        assert!(matches!(decode_event(payload), Some(StreamEvent::Token(t)) if t == "k2 says hi"));
    }

    #[test]
    fn default_model_falls_back_to_legacy_when_no_env_no_registry() {
        std::env::remove_var("KIMI_MODEL");
        std::env::set_var("KEI_MODEL_REGISTRY", "/nonexistent/kei-model-registry-path.toml");
        assert_eq!(default_model(), LEGACY_DEFAULT_MODEL);
        std::env::remove_var("KEI_MODEL_REGISTRY");
    }

    #[test]
    fn cost_falls_back_when_model_unknown() {
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
