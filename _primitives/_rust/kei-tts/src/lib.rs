// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! `kei-tts` — text-to-speech abstraction with 4 backend impls.
//!
//! Backend is chosen at runtime via `KEI_TTS_BACKEND` env var.
//! Compile-time feature flags gate which backends are available.
//!
//! | Feature       | Backend    | Default |
//! |---------------|------------|---------|
//! | `piper`       | local      | ✓       |
//! | `elevenlabs`  | cloud      | –       |
//! | `openai`      | cloud      | –       |
//! | `google`      | cloud      | –       |
//!
//! # Quick start
//! ```no_run
//! # async fn example() -> Result<(), kei_tts::TtsError> {
//! let backend = kei_tts::from_env()?;
//! let req = kei_tts::TtsRequest::new("Hello, world!");
//! let resp = backend.synth(&req).await?;
//! std::fs::write("out.mp3", &resp.audio_bytes).ok();
//! # Ok(()) }
//! ```

pub mod error;
pub mod request;
pub mod response;
pub mod trait_def;

#[cfg(feature = "elevenlabs")]
pub mod elevenlabs;
#[cfg(feature = "google")]
pub mod google;
#[cfg(feature = "openai")]
pub mod openai;
#[cfg(feature = "piper")]
pub mod piper;

pub use error::TtsError;
pub use request::{AudioFormat, TtsRequest};
pub use response::TtsResponse;
pub use trait_def::TtsBackend;

/// Construct the backend selected by `KEI_TTS_BACKEND`.
///
/// Defaults to `piper` when the env var is absent or empty.
/// Returns `TtsError::BackendNotEnabled` if the chosen backend's
/// feature flag was not compiled in.
pub fn from_env() -> Result<Box<dyn TtsBackend>, TtsError> {
    let name = std::env::var("KEI_TTS_BACKEND")
        .unwrap_or_else(|_| "piper".to_string());
    build_backend(&name)
}

fn build_backend(name: &str) -> Result<Box<dyn TtsBackend>, TtsError> {
    match name {
        #[cfg(feature = "piper")]
        "piper" => Ok(Box::new(piper::PiperBackend::from_env()?)),
        #[cfg(feature = "elevenlabs")]
        "elevenlabs" => Ok(Box::new(elevenlabs::ElevenLabsBackend::from_env()?)),
        #[cfg(feature = "openai")]
        "openai" => Ok(Box::new(openai::OpenAiBackend::from_env()?)),
        #[cfg(feature = "google")]
        "google" => Ok(Box::new(google::GoogleBackend::from_env()?)),
        other => Err(TtsError::BackendNotEnabled(other.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// NOTE: env-var tests are run with `-- --test-threads=1` to avoid
    /// races between tests that mutate process-global env.

    #[test]
    fn from_env_defaults_to_piper() {
        // Ensure a previous test has not leaked KEI_TTS_BACKEND.
        std::env::remove_var("KEI_TTS_BACKEND");
        // piper backend requires KEI_TTS_PIPER_MODEL — set a dummy path.
        std::env::set_var("KEI_TTS_PIPER_MODEL", "/tmp/dummy.onnx");
        let backend = from_env().expect("piper backend should construct from env");
        assert_eq!(backend.name(), "piper");
        std::env::remove_var("KEI_TTS_PIPER_MODEL");
    }

    #[test]
    fn from_env_unknown_backend_errors() {
        std::env::remove_var("KEI_TTS_BACKEND");
        let result = build_backend("invalid_provider");
        match result {
            Err(TtsError::BackendNotEnabled(name)) => {
                assert_eq!(name, "invalid_provider");
            }
            Ok(_) => panic!("expected BackendNotEnabled, got Ok"),
            Err(e) => panic!("expected BackendNotEnabled, got different error: {e}"),
        }
    }

    /// Verify piper backend propagates subprocess error on bad model path.
    /// Skipped entirely when `piper-tts` binary is not on PATH.
    #[cfg(feature = "piper")]
    #[tokio::test]
    async fn piper_subprocess_error_on_bad_model() {
        let available = std::process::Command::new("piper-tts")
            .arg("--help")
            .output()
            .is_ok();
        if !available {
            eprintln!("piper-tts not on PATH — skipping binary test");
            return;
        }
        use crate::piper::PiperBackend;
        use crate::trait_def::TtsBackend;
        let backend = PiperBackend::new("piper-tts", "/nonexistent/model.onnx");
        let req = TtsRequest::new("hello");
        let err = backend.synth(&req).await
            .expect_err("bad model path should fail");
        assert!(matches!(err, TtsError::Subprocess(_)));
    }
}
