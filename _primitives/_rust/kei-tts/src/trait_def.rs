// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! `TtsBackend` — the core async trait all backend impls satisfy.
//!
//! Implementing this trait is sufficient to plug a new TTS provider into
//! the `from_env()` dispatch without modifying `lib.rs`. Each backend
//! module is self-contained and feature-gated.

use crate::error::TtsError;
use crate::request::TtsRequest;
use crate::response::TtsResponse;

/// Async TTS synthesis backend.
///
/// Implementations must be `Send + Sync` so they can be stored in a
/// `Box<dyn TtsBackend>` and shared across Tokio tasks.
#[async_trait::async_trait]
pub trait TtsBackend: Send + Sync {
    /// Synthesise `req.text` and return the audio bytes.
    async fn synth(&self, req: &TtsRequest) -> Result<TtsResponse, TtsError>;

    /// Short, stable identifier for this backend (e.g. `"piper"`).
    fn name(&self) -> &'static str;
}
