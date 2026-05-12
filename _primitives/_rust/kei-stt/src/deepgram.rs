// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! Deepgram STT backend — calls `api.deepgram.com/v1/listen`.
//!
//! Endpoint: `POST /v1/listen?language={lang}&punctuate=true`
//! Auth: `Authorization: Token {DEEPGRAM_API_KEY}` header.
//! Body: raw audio bytes with the request MIME type.
//!
//! Response shape:
//! ```json
//! {"results":{"channels":[{"alternatives":[{
//!   "transcript":"...",
//!   "words":[{"word":"...","start":0.1,"end":0.4}]
//! }]}]}}
//! ```
//!
//! Constructor surface:
//!   * [`DeepgramBackend::from_env`] — reads `DEEPGRAM_API_KEY`.
//!   * [`DeepgramBackend::with_base_url`] — explicit URL + key (tests).

#![cfg(feature = "deepgram")]

use crate::error::SttError;
use crate::request::SttRequest;
use crate::response::{Segment, SttResponse};
use crate::trait_def::SttBackend;

const DEFAULT_BASE_URL: &str = "https://api.deepgram.com";

pub struct DeepgramBackend {
    api_key: String,
    client: reqwest::Client,
    base_url: String,
}

impl DeepgramBackend {
    /// Build from explicit base URL and API key (used in wiremock tests).
    pub fn with_base_url(
        base_url: impl Into<String>,
        api_key: impl Into<String>,
    ) -> Self {
        Self {
            api_key: api_key.into(),
            client: reqwest::Client::new(),
            base_url: base_url.into().trim_end_matches('/').to_string(),
        }
    }

    /// Build from `DEEPGRAM_API_KEY` env var.
    pub fn from_env() -> Result<Self, SttError> {
        let key = std::env::var("DEEPGRAM_API_KEY")
            .map_err(|_| SttError::MissingEnv("DEEPGRAM_API_KEY".into()))?;
        Ok(Self::with_base_url(DEFAULT_BASE_URL, key))
    }
}

#[async_trait::async_trait]
impl SttBackend for DeepgramBackend {
    fn name(&self) -> &'static str { "deepgram" }

    async fn transcribe(&self, req: &SttRequest) -> Result<SttResponse, SttError> {
        let mut url = format!("{}/v1/listen?punctuate=true", self.base_url);
        if let Some(lang) = &req.language {
            url.push_str(&format!("&language={lang}"));
        }

        let resp = self.client
            .post(&url)
            .header("Authorization", format!("Token {}", self.api_key))
            .header("Content-Type", &req.mime_type)
            .body(req.audio_bytes.clone())
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let text = resp.text().await.unwrap_or_default();
            return Err(SttError::Http(format!("http {status}: {text}")));
        }

        let body: serde_json::Value = resp.json().await
            .map_err(|e| SttError::InvalidResponse(e.to_string()))?;

        parse_deepgram_response(&body)
    }
}

fn parse_deepgram_response(body: &serde_json::Value) -> Result<SttResponse, SttError> {
    let alt = body
        .pointer("/results/channels/0/alternatives/0")
        .ok_or_else(|| SttError::InvalidResponse("missing alternatives".into()))?;

    let text = alt["transcript"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    let segments = alt["words"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|w| {
            let start_ms = (w["start"].as_f64()? * 1000.0) as u64;
            let end_ms   = (w["end"].as_f64()? * 1000.0) as u64;
            let word     = w["word"].as_str()?.to_string();
            Some(Segment { start_ms, end_ms, text: word })
        })
        .collect();

    Ok(SttResponse { text, segments, language_detected: None })
}

#[cfg(test)]
#[path = "deepgram_test.rs"]
mod tests;
