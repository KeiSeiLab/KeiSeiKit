// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! OpenAI TTS backend — calls `api.openai.com/v1/audio/speech`.
//!
//! Supported models: `tts-1` (fast) and `tts-1-hd` (higher quality).
//! Default voice: `alloy`. Format negotiated via `response_format` field.
//!
//! Constructor surface:
//!   * [`OpenAiBackend::from_env`] — reads `OPENAI_API_KEY`.
//!   * [`OpenAiBackend::with_base_url`] — explicit URL + key + model (tests).

#![cfg(feature = "openai")]

use crate::error::TtsError;
use crate::request::{AudioFormat, TtsRequest};
use crate::response::TtsResponse;
use crate::trait_def::TtsBackend;

const DEFAULT_BASE_URL: &str = "https://api.openai.com";
const DEFAULT_MODEL: &str = "tts-1";
const DEFAULT_VOICE: &str = "alloy";

pub struct OpenAiBackend {
    api_key: String,
    model: String,
    client: reqwest::Client,
    base_url: String,
}

impl OpenAiBackend {
    /// Build from explicit parameters (used in wiremock tests).
    pub fn with_base_url(
        base_url: impl Into<String>,
        api_key: impl Into<String>,
        model: impl Into<String>,
    ) -> Self {
        Self {
            api_key: api_key.into(),
            model: model.into(),
            client: reqwest::Client::new(),
            base_url: base_url.into().trim_end_matches('/').to_string(),
        }
    }

    /// Build from `OPENAI_API_KEY` env var. Reads optional
    /// `KEI_TTS_OPENAI_MODEL` (default `tts-1`).
    pub fn from_env() -> Result<Self, TtsError> {
        let key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| TtsError::MissingEnv("OPENAI_API_KEY".into()))?;
        let model = std::env::var("KEI_TTS_OPENAI_MODEL")
            .unwrap_or_else(|_| DEFAULT_MODEL.to_string());
        Ok(Self::with_base_url(DEFAULT_BASE_URL, key, model))
    }

    fn format_str(fmt: AudioFormat) -> &'static str {
        match fmt {
            AudioFormat::Mp3 => "mp3",
            AudioFormat::Ogg => "opus",
            AudioFormat::Wav => "wav",
            AudioFormat::Raw => "pcm",
        }
    }
}

#[async_trait::async_trait]
impl TtsBackend for OpenAiBackend {
    fn name(&self) -> &'static str { "openai" }

    async fn synth(&self, req: &TtsRequest) -> Result<TtsResponse, TtsError> {
        let url = format!("{}/v1/audio/speech", self.base_url);
        let voice = req.voice_id.as_deref().unwrap_or(DEFAULT_VOICE);
        let body = serde_json::json!({
            "model": self.model,
            "input": req.text,
            "voice": voice,
            "response_format": Self::format_str(req.format),
        });
        let resp = self.client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let text = resp.text().await.unwrap_or_default();
            return Err(TtsError::Http(format!("http {status}: {text}")));
        }
        let mime = req.format.mime_type().to_string();
        let bytes = resp.bytes().await
            .map_err(|e| TtsError::Http(e.to_string()))?
            .to_vec();
        Ok(TtsResponse::new(bytes, mime))
    }
}

#[cfg(test)]
#[path = "openai_test.rs"]
mod tests;
