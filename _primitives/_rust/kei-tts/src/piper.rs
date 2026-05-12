// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! Piper TTS backend — spawns the `piper-tts` subprocess.
//!
//! Invocation: `piper-tts --model <model> --output_raw`
//! Text is written to stdin; raw PCM bytes are read from stdout.
//! A minimal RIFF WAV header is prepended when `format = Wav`.
//!
//! Constructor surface:
//!   * [`PiperBackend::from_env`] — reads `KEI_TTS_PIPER_MODEL` +
//!     optional `KEI_TTS_PIPER_BINARY` (default `piper-tts`).
//!   * [`PiperBackend::new`] — explicit binary + model paths (tests).

#![cfg(feature = "piper")]

use std::path::PathBuf;

use tokio::io::AsyncWriteExt as _;
use tokio::process::Command;

use crate::error::TtsError;
use crate::request::{AudioFormat, TtsRequest};
use crate::response::TtsResponse;
use crate::trait_def::TtsBackend;

const DEFAULT_BINARY: &str = "piper-tts";
// PCM parameters piper-tts emits by default.
const SAMPLE_RATE: u32 = 22050;
const CHANNELS: u16 = 1;
const BITS_PER_SAMPLE: u16 = 16;

pub struct PiperBackend {
    binary: PathBuf,
    model: PathBuf,
}

impl PiperBackend {
    /// Build from explicit binary path and model path.
    pub fn new(binary: impl Into<PathBuf>, model: impl Into<PathBuf>) -> Self {
        Self { binary: binary.into(), model: model.into() }
    }

    /// Build from env vars.
    /// Required: `KEI_TTS_PIPER_MODEL` (path to `.onnx` model file).
    /// Optional: `KEI_TTS_PIPER_BINARY` (default `piper-tts`).
    pub fn from_env() -> Result<Self, TtsError> {
        let model = std::env::var("KEI_TTS_PIPER_MODEL")
            .map_err(|_| TtsError::MissingEnv("KEI_TTS_PIPER_MODEL".into()))?;
        let binary = std::env::var("KEI_TTS_PIPER_BINARY")
            .unwrap_or_else(|_| DEFAULT_BINARY.to_string());
        Ok(Self::new(binary, model))
    }
}

/// Build a minimal 44-byte RIFF WAV header for PCM data.
fn wav_header(data_len: u32) -> Vec<u8> {
    let byte_rate = SAMPLE_RATE * u32::from(CHANNELS) * u32::from(BITS_PER_SAMPLE) / 8;
    let block_align: u16 = CHANNELS * BITS_PER_SAMPLE / 8;
    let mut h = Vec::with_capacity(44);
    h.extend_from_slice(b"RIFF");
    h.extend_from_slice(&(36u32 + data_len).to_le_bytes());
    h.extend_from_slice(b"WAVE");
    h.extend_from_slice(b"fmt ");
    h.extend_from_slice(&16u32.to_le_bytes());  // subchunk1 size
    h.extend_from_slice(&1u16.to_le_bytes());   // PCM
    h.extend_from_slice(&CHANNELS.to_le_bytes());
    h.extend_from_slice(&SAMPLE_RATE.to_le_bytes());
    h.extend_from_slice(&byte_rate.to_le_bytes());
    h.extend_from_slice(&block_align.to_le_bytes());
    h.extend_from_slice(&BITS_PER_SAMPLE.to_le_bytes());
    h.extend_from_slice(b"data");
    h.extend_from_slice(&data_len.to_le_bytes());
    h
}

#[async_trait::async_trait]
impl TtsBackend for PiperBackend {
    fn name(&self) -> &'static str { "piper" }

    async fn synth(&self, req: &TtsRequest) -> Result<TtsResponse, TtsError> {
        let mut child = Command::new(&self.binary)
            .arg("--model")
            .arg(&self.model)
            .arg("--output_raw")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn()
            .map_err(|e| TtsError::Subprocess(e.to_string()))?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(req.text.as_bytes()).await
                .map_err(|e| TtsError::Subprocess(e.to_string()))?;
        }

        let output = child.wait_with_output().await
            .map_err(|e| TtsError::Subprocess(e.to_string()))?;

        if !output.status.success() {
            return Err(TtsError::Subprocess(format!(
                "piper-tts exited with {}", output.status
            )));
        }

        let pcm = output.stdout;
        match req.format {
            AudioFormat::Wav => {
                let mut wav = wav_header(pcm.len() as u32);
                wav.extend_from_slice(&pcm);
                Ok(TtsResponse::new(wav, "audio/wav"))
            }
            _ => Ok(TtsResponse::new(pcm, "audio/pcm")),
        }
    }
}
