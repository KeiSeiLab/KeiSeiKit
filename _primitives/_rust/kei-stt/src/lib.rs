// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! `kei-stt` — speech-to-text abstraction with 3 backend impls.
//!
//! Backend is chosen at runtime via `KEI_STT_BACKEND` env var.
//! Compile-time feature flags gate which backends are available.
//!
//! | Feature          | Backend       | Default |
//! |------------------|---------------|---------|
//! | `whisper-local`  | local subprocess | ✓    |
//! | `deepgram`       | cloud API     | –       |
//! | `openai-whisper` | cloud API     | –       |
//!
//! # Quick start
//! ```no_run
//! # async fn example() -> Result<(), kei_stt::SttError> {
//! let backend = kei_stt::from_env()?;
//! let audio = std::fs::read("speech.wav").unwrap();
//! let req = kei_stt::SttRequest::new_wav(audio);
//! let resp = backend.transcribe(&req).await?;
//! println!("transcript: {}", resp.text);
//! # Ok(()) }
//! ```

pub mod error;
pub mod request;
pub mod response;
pub mod trait_def;

#[cfg(feature = "whisper-local")]
pub mod whisper_local;
#[cfg(feature = "deepgram")]
pub mod deepgram;
#[cfg(feature = "openai-whisper")]
pub mod openai_whisper;

pub use error::SttError;
pub use request::SttRequest;
pub use response::{Segment, SttResponse};
pub use trait_def::SttBackend;

/// Construct the backend selected by `KEI_STT_BACKEND`.
///
/// Defaults to `whisper-local` when the env var is absent or empty.
/// Returns `SttError::BackendNotEnabled` if the chosen backend's
/// feature flag was not compiled in.
pub fn from_env() -> Result<Box<dyn SttBackend>, SttError> {
    let name = std::env::var("KEI_STT_BACKEND")
        .unwrap_or_else(|_| "whisper-local".to_string());
    build_backend(&name)
}

fn build_backend(name: &str) -> Result<Box<dyn SttBackend>, SttError> {
    match name {
        #[cfg(feature = "whisper-local")]
        "whisper-local" => Ok(Box::new(whisper_local::WhisperLocalBackend::from_env())),
        #[cfg(feature = "deepgram")]
        "deepgram" => Ok(Box::new(deepgram::DeepgramBackend::from_env()?)),
        #[cfg(feature = "openai-whisper")]
        "openai-whisper" => Ok(Box::new(openai_whisper::OpenAiWhisperBackend::from_env()?)),
        other => Err(SttError::BackendNotEnabled(other.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_env_defaults_to_whisper_local() {
        std::env::remove_var("KEI_STT_BACKEND");
        let backend = from_env().expect("whisper-local backend should construct");
        assert_eq!(backend.name(), "whisper-local");
    }

    #[test]
    fn from_env_unknown_backend_errors() {
        std::env::remove_var("KEI_STT_BACKEND");
        let result = build_backend("unknown_provider");
        match result {
            Err(SttError::BackendNotEnabled(name)) => {
                assert_eq!(name, "unknown_provider");
            }
            Ok(_) => panic!("expected BackendNotEnabled, got Ok"),
            Err(e) => panic!("expected BackendNotEnabled, got: {e}"),
        }
    }
}
