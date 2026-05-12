// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! Whisper-local STT backend — spawns the `whisper` CLI subprocess.
//!
//! Invocation:
//!   `<binary> <audio_file> --model <model> --output_format json --output_dir <dir>`
//!
//! The openai-whisper Python package writes `<stem>.json` into `--output_dir`.
//! We parse `text` + `segments[].{start,end,text}` from that JSON.
//!
//! Constructor surface:
//!   * [`WhisperLocalBackend::from_env`] — reads `KEI_STT_WHISPER_BINARY` +
//!     `KEI_STT_WHISPER_MODEL` (default `base.en`).
//!   * [`WhisperLocalBackend::with_binary`] + [`WhisperLocalBackend::with_model`]
//!     — explicit overrides for testability.

#![cfg(feature = "whisper-local")]

use std::path::PathBuf;

use tokio::process::Command;

use crate::error::SttError;
use crate::request::SttRequest;
use crate::response::{Segment, SttResponse};
use crate::trait_def::SttBackend;

const DEFAULT_BINARY: &str = "whisper";
const DEFAULT_MODEL: &str = "base.en";

pub struct WhisperLocalBackend {
    binary: PathBuf,
    model: String,
}

impl WhisperLocalBackend {
    /// Build from explicit binary and model (used in tests).
    pub fn new(binary: impl Into<PathBuf>, model: impl Into<String>) -> Self {
        Self { binary: binary.into(), model: model.into() }
    }

    /// Build from env vars.
    /// Optional: `KEI_STT_WHISPER_BINARY` (default `whisper`).
    /// Optional: `KEI_STT_WHISPER_MODEL` (default `base.en`).
    pub fn from_env() -> Self {
        let binary = std::env::var("KEI_STT_WHISPER_BINARY")
            .unwrap_or_else(|_| DEFAULT_BINARY.to_string());
        let model = std::env::var("KEI_STT_WHISPER_MODEL")
            .unwrap_or_else(|_| DEFAULT_MODEL.to_string());
        Self::new(binary, model)
    }

    fn ext_from_mime(mime: &str) -> &'static str {
        match mime {
            "audio/mpeg" => "mp3",
            "audio/ogg" => "ogg",
            "audio/flac" => "flac",
            _ => "wav",
        }
    }
}

#[async_trait::async_trait]
impl SttBackend for WhisperLocalBackend {
    fn name(&self) -> &'static str { "whisper-local" }

    async fn transcribe(&self, req: &SttRequest) -> Result<SttResponse, SttError> {
        let ext = Self::ext_from_mime(&req.mime_type);

        // Write audio bytes to a named temp file.
        let audio_file = tempfile::Builder::new()
            .suffix(&format!(".{ext}"))
            .tempfile()
            .map_err(|e| SttError::Subprocess(e.to_string()))?;

        let out_dir = tempfile::tempdir()
            .map_err(|e| SttError::Subprocess(e.to_string()))?;

        tokio::fs::write(audio_file.path(), &req.audio_bytes)
            .await
            .map_err(|e| SttError::Subprocess(e.to_string()))?;

        // Build and run whisper CLI command.
        let mut cmd = Command::new(&self.binary);
        cmd.arg(audio_file.path())
            .arg("--model").arg(&self.model)
            .arg("--output_format").arg("json")
            .arg("--output_dir").arg(out_dir.path());

        if let Some(lang) = &req.language {
            cmd.arg("--language").arg(lang);
        }

        let output = cmd.output().await
            .map_err(|e| SttError::Subprocess(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
            return Err(SttError::Subprocess(format!(
                "whisper exited {}: {stderr}", output.status
            )));
        }

        // Find the produced JSON file (stem of input + ".json").
        let stem = audio_file.path()
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("audio");
        let json_path = out_dir.path().join(format!("{stem}.json"));

        let json_bytes = tokio::fs::read(&json_path).await
            .map_err(|e| SttError::InvalidResponse(format!("json file: {e}")))?;

        parse_whisper_json(&json_bytes)
    }
}

fn parse_whisper_json(bytes: &[u8]) -> Result<SttResponse, SttError> {
    let v: serde_json::Value = serde_json::from_slice(bytes)
        .map_err(|e| SttError::InvalidResponse(e.to_string()))?;

    let text = v["text"].as_str()
        .unwrap_or_default()
        .trim()
        .to_string();

    let segments = v["segments"]
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

    Ok(SttResponse { text, segments, language_detected: None })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn whisper_local_subprocess_error_on_bad_binary() {
        let backend = WhisperLocalBackend::new("/nonexistent/whisper", "base.en");
        let req = SttRequest::new_wav(b"RIFF....".to_vec());
        let err = backend.transcribe(&req).await
            .expect_err("should fail with non-existent binary");
        assert!(
            matches!(err, SttError::Subprocess(_)),
            "expected Subprocess error, got: {err:?}"
        );
    }

    #[test]
    fn parse_whisper_json_full() {
        let json = br#"{
            "text": " Hello world",
            "segments": [
                {"start": 0.0, "end": 1.5, "text": " Hello"},
                {"start": 1.5, "end": 2.0, "text": " world"}
            ]
        }"#;
        let resp = parse_whisper_json(json).expect("parse should succeed");
        assert_eq!(resp.text, "Hello world");
        assert_eq!(resp.segments.len(), 2);
        assert_eq!(resp.segments[0].end_ms, 1500);
        assert_eq!(resp.segments[1].start_ms, 1500);
    }

    #[test]
    fn parse_whisper_json_no_segments() {
        let json = br#"{"text": " Hi"}"#;
        let resp = parse_whisper_json(json).expect("parse should succeed");
        assert_eq!(resp.text, "Hi");
        assert!(resp.segments.is_empty());
    }
}
