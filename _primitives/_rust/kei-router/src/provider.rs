//! LLM provider trait — minimal abstraction across Anthropic, OpenAI, Kimi.
//!
//! Constructor Pattern: types here are wire-format-agnostic. Each provider impl
//! translates these into its own request shape and back into `StreamEvent`s.
//!
//! Wave 32 v0.40 design notes:
//! - `Message` is provider-agnostic role+content. Tool-use blocks not modelled
//!   in v0.40 (text-only streaming first; tool-call streaming added in v0.41).
//! - `StreamEvent::Token` is the only event a v0.40 caller needs to render.
//!   Other variants exist so v0.41 tool-call streaming doesn't break the trait.
//! - Cost cents are integer per-million-tokens. Sub-cent pricing rounds up.

use async_trait::async_trait;
use futures::stream::BoxStream;
use serde::{Deserialize, Serialize};

/// One conversation turn. Role is "user" | "assistant" | "system" depending
/// on provider; we forbid system as a `Message` because every provider takes
/// a separate `system` parameter on `stream_message`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// Tool definition exposed to the model. Schema is a JSON Schema object;
/// the provider impl wraps it in its own envelope (Anthropic `input_schema`,
/// OpenAI `parameters`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub schema: serde_json::Value,
}

/// One event from the streaming response. v0.40 implementations emit
/// `Token` and `Done` only; tool-call variants reserved for v0.41.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamEvent {
    /// Incremental text token.
    Token(String),
    /// Tool-call name signalled (v0.41).
    ToolCallStart { name: String, call_id: String },
    /// Tool-call argument fragment (v0.41).
    ToolCallDelta { call_id: String, fragment: String },
    /// Stream completed cleanly.
    Done,
}

/// Errors a provider may surface. `Upstream` carries a status + truncated body.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("missing API key for provider {0}")]
    MissingKey(&'static str),
    #[error("http: {0}")]
    Http(#[from] reqwest::Error),
    #[error("upstream {provider} status {status}: {body}")]
    Upstream { provider: &'static str, status: u16, body: String },
    #[error("rate limited by {0}")]
    RateLimit(&'static str),
    #[error("service unavailable from {0}")]
    ServiceUnavailable(&'static str),
    #[error("timeout calling {0}")]
    Timeout(&'static str),
    #[error("unknown provider {0}")]
    UnknownProvider(String),
}

/// Provider trait. Each implementation is one cube (one file).
#[async_trait]
pub trait Provider: Send + Sync {
    /// Stable wire name, e.g. "anthropic" / "openai" / "kimi".
    fn name(&self) -> &'static str;

    /// Cents per 1M input tokens. Used by `LlmRouter::cheapest_for_*`.
    fn cost_per_m_tok_input_cents(&self) -> u32;

    /// Cents per 1M output tokens.
    fn cost_per_m_tok_output_cents(&self) -> u32;

    /// Open a streaming Messages request. Returns a stream after the
    /// HTTP handshake completes (so 429/5xx surface before SSE framing).
    async fn stream_message(
        &self,
        system: &str,
        messages: &[Message],
        tools: Option<&[Tool]>,
    ) -> Result<BoxStream<'static, Result<StreamEvent, Error>>, Error>;
}
