// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! `SttRequest` — input type for all STT backends.
//!
//! Deliberately backend-agnostic: each backend maps its fields to
//! provider-specific parameters in its own module.

use serde::{Deserialize, Serialize};

/// Parameters for a single STT transcription request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SttRequest {
    /// Raw audio bytes to transcribe.
    pub audio_bytes: Vec<u8>,

    /// MIME type of the audio data (e.g. `"audio/wav"`, `"audio/mpeg"`, `"audio/ogg"`).
    pub mime_type: String,

    /// BCP-47 language hint (e.g. `"en"`, `"ru"`). `None` → auto-detect.
    pub language: Option<String>,
}

impl SttRequest {
    /// Convenience constructor for WAV audio with no language hint.
    pub fn new_wav(audio_bytes: Vec<u8>) -> Self {
        Self {
            audio_bytes,
            mime_type: "audio/wav".to_string(),
            language: None,
        }
    }
}
