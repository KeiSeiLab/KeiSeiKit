// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! `SttBackend` — the core async trait all backend impls satisfy.
//!
//! Implementing this trait is sufficient to plug a new STT provider into
//! the `from_env()` dispatch without modifying `lib.rs`. Each backend
//! module is self-contained and feature-gated.

use crate::error::SttError;
use crate::request::SttRequest;
use crate::response::SttResponse;

/// Async STT transcription backend.
///
/// Implementations must be `Send + Sync` so they can be stored in a
/// `Box<dyn SttBackend>` and shared across Tokio tasks.
#[async_trait::async_trait]
pub trait SttBackend: Send + Sync {
    /// Transcribe the audio in `req` and return the text plus optional segments.
    async fn transcribe(&self, req: &SttRequest) -> Result<SttResponse, SttError>;

    /// Short, stable identifier for this backend (e.g. `"whisper-local"`).
    fn name(&self) -> &'static str;
}
