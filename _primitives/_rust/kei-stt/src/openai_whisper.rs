// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! OpenAI Whisper STT backend — calls `api.openai.com/v1/audio/transcriptions`.
//!
//! Sends a multipart/form-data POST with:
//!   - `file`: audio bytes (filename derived from MIME type)
//!   - `model`: `whisper-1`
//!   - `response_format`: `verbose_json`
//!   - `language`: BCP-47 code if provided
//!
//! Response `verbose_json` shape:
//! ```json
//! {"text":"...", "segments":[{"start":0.0,"end":1.0,"text":"..."}]}
//! ```
//!
//! Constructor surface:
//!   * [`OpenAiWhisperBackend::from_env`] — reads `OPENAI_API_KEY`.
//!   * [`OpenAiWhisperBackend::with_base_url`] — explicit URL + key (tests).

#![cfg(feature = "openai-whisper")]

use reqwest::multipart;

use crate::error::SttError;
use crate::request::SttRequest;
use crate::response::{Segment, SttResponse};
use crate::trait_def::SttBackend;

const DEFAULT_BASE_URL: &str = "https://api.openai.com";
const WHISPER_MODEL: &str = "whisper-1";

pub struct OpenAiWhisperBackend {
    api_key: String,
    client: reqwest::Client,
    base_url: String,
}

impl OpenAiWhisperBackend {
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

    /// Build from `OPENAI_API_KEY` env var.
    pub fn from_env() -> Result<Self, SttError> {
        let key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| SttError::MissingEnv("OPENAI_API_KEY".into()))?;
        Ok(Self::with_base_url(DEFAULT_BASE_URL, key))
    }

    fn filename_from_mime(mime: &str) -> &'static str {
        match mime {
            "audio/mpeg" => "audio.mp3",
            "audio/ogg"  => "audio.ogg",
            "audio/flac" => "audio.flac",
            _            => "audio.wav",
        }
    }
}

#[async_trait::async_trait]
impl SttBackend for OpenAiWhisperBackend {
    fn name(&self) -> &'static str { "openai-whisper" }

    async fn transcribe(&self, req: &SttRequest) -> Result<SttResponse, SttError> {
        let url = format!("{}/v1/audio/transcriptions", self.base_url);
        let filename = Self::filename_from_mime(&req.mime_type);

        let file_part = multipart::Part::bytes(req.audio_bytes.clone())
            .file_name(filename)
            .mime_str(&req.mime_type)
            .map_err(|e| SttError::InvalidAudio(e.to_string()))?;

        let mut form = multipart::Form::new()
            .part("file", file_part)
            .text("model", WHISPER_MODEL)
            .text("response_format", "verbose_json");

        if let Some(lang) = &req.language {
            form = form.text("language", lang.clone());
        }

        let resp = self.client
            .post(&url)
            .bearer_auth(&self.api_key)
            .multipart(form)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let text = resp.text().await.unwrap_or_default();
            return Err(SttError::Http(format!("http {status}: {text}")));
        }

        let body: serde_json::Value = resp.json().await
            .map_err(|e| SttError::InvalidResponse(e.to_string()))?;

        parse_openai_whisper_response(&body)
    }
}

fn parse_openai_whisper_response(body: &serde_json::Value) -> Result<SttResponse, SttError> {
    let text = body["text"]
        .as_str()
        .unwrap_or_default()
        .trim()
        .to_string();

    let language_detected = body["language"]
        .as_str()
        .map(|s| s.to_string());

    let segments = body["segments"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|s| {
            let start_ms = (s["start"].as_f64()? * 1000.0) as u64;
            let end_ms   = (s["end"].as_f64()? * 1000.0) as u64;
            let seg_text = s["text"].as_str()?.trim().to_string();
            Some(Segment { start_ms, end_ms, text: seg_text })
        })
        .collect();

    Ok(SttResponse { text, segments, language_detected })
}

#[cfg(test)]
#[path = "openai_whisper_test.rs"]
mod tests;
