// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! ElevenLabs TTS backend — streams audio from `api.elevenlabs.io`.
//!
//! Endpoint: `POST /v1/text-to-speech/{voice_id}/stream`
//! Auth: `xi-api-key: {ELEVENLABS_API_KEY}` header.
//! Response: raw audio bytes (format-specific Content-Type).
//!
//! Constructor surface:
//!   * [`ElevenLabsBackend::from_env`] — reads `ELEVENLABS_API_KEY`.
//!   * [`ElevenLabsBackend::with_base_url`] — explicit URL + key (tests).

#![cfg(feature = "elevenlabs")]

use crate::error::TtsError;
use crate::request::{AudioFormat, TtsRequest};
use crate::response::TtsResponse;
use crate::trait_def::TtsBackend;

const DEFAULT_BASE_URL: &str = "https://api.elevenlabs.io";
const DEFAULT_VOICE_ID: &str = "21m00Tcm4TlvDq8ikWAM"; // Rachel

pub struct ElevenLabsBackend {
    api_key: String,
    client: reqwest::Client,
    base_url: String,
}

impl ElevenLabsBackend {
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

    /// Build from `ELEVENLABS_API_KEY` env var.
    pub fn from_env() -> Result<Self, TtsError> {
        let key = std::env::var("ELEVENLABS_API_KEY")
            .map_err(|_| TtsError::MissingEnv("ELEVENLABS_API_KEY".into()))?;
        Ok(Self::with_base_url(DEFAULT_BASE_URL, key))
    }

    fn format_param(fmt: AudioFormat) -> &'static str {
        match fmt {
            AudioFormat::Mp3 => "mp3_44100_128",
            AudioFormat::Ogg => "ogg_48000",
            AudioFormat::Wav | AudioFormat::Raw => "pcm_44100",
        }
    }
}

#[async_trait::async_trait]
impl TtsBackend for ElevenLabsBackend {
    fn name(&self) -> &'static str { "elevenlabs" }

    async fn synth(&self, req: &TtsRequest) -> Result<TtsResponse, TtsError> {
        let voice = req.voice_id.as_deref().unwrap_or(DEFAULT_VOICE_ID);
        let url = format!(
            "{}/v1/text-to-speech/{}/stream",
            self.base_url, voice
        );
        let body = serde_json::json!({
            "text": req.text,
            "output_format": Self::format_param(req.format),
        });
        let resp = self.client
            .post(&url)
            .header("xi-api-key", &self.api_key)
            .header("Content-Type", "application/json")
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
#[path = "elevenlabs_test.rs"]
mod tests;
