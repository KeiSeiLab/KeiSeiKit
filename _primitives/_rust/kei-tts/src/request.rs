// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! `TtsRequest` and `AudioFormat` — input types for all TTS backends.
//!
//! `TtsRequest` is deliberately backend-agnostic: each backend maps its
//! fields to provider-specific parameters in its own module.

use serde::{Deserialize, Serialize};

/// Output audio encoding requested from the backend.
///
/// Not every backend supports every format; unsupported formats result
/// in `TtsError::InvalidResponse`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AudioFormat {
    /// MPEG Layer 3 — the most widely supported lossy format.
    Mp3,
    /// Ogg Vorbis — open, patent-free lossy format.
    Ogg,
    /// RIFF WAVE — uncompressed PCM container.
    Wav,
    /// Raw PCM bytes with no container header (piper default).
    Raw,
}

impl AudioFormat {
    /// Returns the MIME type string for the format.
    pub fn mime_type(self) -> &'static str {
        match self {
            AudioFormat::Mp3 => "audio/mpeg",
            AudioFormat::Ogg => "audio/ogg",
            AudioFormat::Wav => "audio/wav",
            AudioFormat::Raw => "audio/pcm",
        }
    }
}

/// Parameters for a single TTS synthesis request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsRequest {
    /// The text to synthesise.
    pub text: String,

    /// Backend-specific voice identifier. `None` lets the backend use
    /// its own default voice.
    pub voice_id: Option<String>,

    /// BCP-47 language tag (e.g. `"ru"`, `"en-US"`). `None` → auto.
    pub language: Option<String>,

    /// Desired output audio encoding.
    pub format: AudioFormat,
}

impl TtsRequest {
    /// Convenience constructor for plain text with backend defaults.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            voice_id: None,
            language: None,
            format: AudioFormat::Mp3,
        }
    }
}
