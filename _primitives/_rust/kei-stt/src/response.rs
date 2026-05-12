// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! `SttResponse` and `Segment` — output types for all STT backends.
//!
//! `segments` is empty when the backend does not provide word-level timing.

use serde::{Deserialize, Serialize};

/// A timed text segment from the transcription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
    /// Start of the segment in milliseconds from the audio start.
    pub start_ms: u64,

    /// End of the segment in milliseconds from the audio start.
    pub end_ms: u64,

    /// Transcribed text for this segment.
    pub text: String,
}

/// Result of a successful STT transcription call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SttResponse {
    /// Full transcribed text (concatenation of all segments).
    pub text: String,

    /// Word/sentence-level timing segments.
    /// Empty when the backend does not provide timing data.
    pub segments: Vec<Segment>,

    /// BCP-47 language code detected by the backend. `None` if not reported.
    pub language_detected: Option<String>,
}

impl SttResponse {
    /// Construct a minimal response with text only.
    pub fn text_only(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            segments: Vec::new(),
            language_detected: None,
        }
    }
}
