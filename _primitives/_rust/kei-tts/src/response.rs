// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! `TtsResponse` — output of a successful TTS synthesis call.

use serde::{Deserialize, Serialize};

/// Audio data returned by a TTS backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsResponse {
    /// Raw bytes of the synthesised audio file.
    pub audio_bytes: Vec<u8>,

    /// MIME type of the audio data (e.g. `"audio/mpeg"`, `"audio/wav"`).
    pub mime_type: String,
}

impl TtsResponse {
    /// Construct a response with explicit audio data and MIME type.
    pub fn new(audio_bytes: Vec<u8>, mime_type: impl Into<String>) -> Self {
        Self { audio_bytes, mime_type: mime_type.into() }
    }
}
