// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! Google Cloud TTS backend — calls `texttospeech.googleapis.com`.
//!
//! Endpoint: `POST /v1/text:synthesize?key={api_key}`
//! Response: JSON `{"audioContent": "<base64>"}`. Base64-decoded bytes
//! are returned as `TtsResponse.audio_bytes`.
//!
//! Constructor surface:
//!   * [`GoogleBackend::from_env`] — reads `GOOGLE_TTS_API_KEY`.
//!   * [`GoogleBackend::with_base_url`] — explicit URL + key (tests).

#![cfg(feature = "google")]

use base64::{engine::general_purpose::STANDARD as B64, Engine as _};

use crate::error::TtsError;
use crate::request::{AudioFormat, TtsRequest};
use crate::response::TtsResponse;
use crate::trait_def::TtsBackend;

const DEFAULT_BASE_URL: &str = "https://texttospeech.googleapis.com";
const DEFAULT_VOICE: &str = "en-US-Wavenet-D";
const DEFAULT_LANG: &str = "en-US";

pub struct GoogleBackend {
    api_key: String,
    client: reqwest::Client,
    base_url: String,
}

impl GoogleBackend {
    /// Build from explicit parameters (used in wiremock tests).
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

    /// Build from `GOOGLE_TTS_API_KEY` env var.
    pub fn from_env() -> Result<Self, TtsError> {
        let key = std::env::var("GOOGLE_TTS_API_KEY")
            .map_err(|_| TtsError::MissingEnv("GOOGLE_TTS_API_KEY".into()))?;
        Ok(Self::with_base_url(DEFAULT_BASE_URL, key))
    }

    fn encoding_str(fmt: AudioFormat) -> &'static str {
        match fmt {
            AudioFormat::Mp3 => "MP3",
            AudioFormat::Ogg => "OGG_OPUS",
            AudioFormat::Wav | AudioFormat::Raw => "LINEAR16",
        }
    }
}

#[derive(serde::Deserialize)]
struct GoogleResponse {
    #[serde(rename = "audioContent")]
    audio_content: String,
}

#[async_trait::async_trait]
impl TtsBackend for GoogleBackend {
    fn name(&self) -> &'static str { "google" }

    async fn synth(&self, req: &TtsRequest) -> Result<TtsResponse, TtsError> {
        let url = format!(
            "{}/v1/text:synthesize?key={}",
            self.base_url, self.api_key
        );
        let voice_name = req.voice_id.as_deref().unwrap_or(DEFAULT_VOICE);
        let lang = req.language.as_deref().unwrap_or(DEFAULT_LANG);
        let body = serde_json::json!({
            "input": { "text": req.text },
            "voice": { "languageCode": lang, "name": voice_name },
            "audioConfig": { "audioEncoding": Self::encoding_str(req.format) },
        });
        let resp = self.client
            .post(&url)
            .json(&body)
            .send()
            .await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let text = resp.text().await.unwrap_or_default();
            return Err(TtsError::Http(format!("http {status}: {text}")));
        }
        let parsed: GoogleResponse = resp.json().await
            .map_err(|e| TtsError::InvalidResponse(e.to_string()))?;
        let bytes = B64.decode(&parsed.audio_content)
            .map_err(|e| TtsError::InvalidResponse(format!("base64: {e}")))?;
        Ok(TtsResponse::new(bytes, req.format.mime_type().to_string()))
    }
}

#[cfg(test)]
#[path = "google_test.rs"]
mod tests;
