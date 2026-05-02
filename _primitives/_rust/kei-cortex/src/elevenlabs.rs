//! ElevenLabs TTS client — `POST /v1/text-to-speech/{voice_id}`.
//!
//! Public surface: a single async function `synthesize(voice_id, text)` that
//! returns the raw mp3 bytes from ElevenLabs. Uses `eleven_turbo_v2_5` with
//! stability=0.5 / similarity_boost=0.75 — these values are the documented
//! middle-of-the-road defaults per the ElevenLabs API reference.
//!
//! `ELEVENLABS_API_KEY` is read from the environment on every call so the
//! user may rotate it without restarting the daemon (RULE 0.8 — env by
//! reference, never hardcoded). We never log the full text; only its length
//! and the response status / byte count.

use crate::http_helpers::{read_capped, HTTP_CLIENT};
use std::time::Duration;

/// Errors surfaced to the caller; handlers map them onto HTTP codes.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("ELEVENLABS_API_KEY environment variable not set")]
    NoApiKey,
    #[error("elevenlabs http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("elevenlabs returned status {0}: {1}")]
    Upstream(u16, String),
    #[error("elevenlabs call exceeded 60s budget")]
    Timeout,
}

const TTS_BASE: &str = "https://api.elevenlabs.io/v1/text-to-speech";
const BUDGET: Duration = Duration::from_secs(60);
const BODY_PREVIEW_CAP: usize = 512;
const MODEL_ID: &str = "eleven_turbo_v2_5";
/// Cap on successful audio response bodies (64 MiB).
const AUDIO_BODY_CAP: usize = 64 * 1024 * 1024;
/// Cap on error response bodies (16 KiB).
const ERROR_BODY_CAP: usize = 16 * 1024;

/// Synthesize speech for `text` using the given `voice_id`. Returns mp3 bytes.
pub async fn synthesize(voice_id: &str, text: &str) -> Result<Vec<u8>, Error> {
    let key = std::env::var("ELEVENLABS_API_KEY").map_err(|_| Error::NoApiKey)?;
    let fut = call_tts(&key, voice_id, text);
    match tokio::time::timeout(BUDGET, fut).await {
        Ok(r) => r,
        Err(_) => Err(Error::Timeout),
    }
}

/// POST the JSON body and collect the audio bytes. Split from `synthesize`
/// so the timeout wrapper stays a thin shell.
async fn call_tts(
    key: &str,
    voice_id: &str,
    text: &str,
) -> Result<Vec<u8>, Error> {
    let url = format!("{TTS_BASE}/{voice_id}");
    let body = build_body(text);
    let resp = HTTP_CLIENT
        .post(&url)
        .header("xi-api-key", key)
        .header("Content-Type", "application/json")
        .header("Accept", "audio/mpeg")
        .json(&body)
        .send()
        .await?;
    decode_audio(resp).await
}

/// Build the JSON request body. Voice settings are the documented API
/// defaults; model is `eleven_turbo_v2_5` for low latency + English/Multi.
fn build_body(text: &str) -> serde_json::Value {
    serde_json::json!({
        "text": text,
        "model_id": MODEL_ID,
        "voice_settings": {
            "stability": 0.5,
            "similarity_boost": 0.75,
        },
    })
}

/// Turn an ElevenLabs response into raw mp3 bytes; map non-2xx to `Upstream`.
async fn decode_audio(resp: reqwest::Response) -> Result<Vec<u8>, Error> {
    use crate::http_helpers::CappedReadError;
    let status = resp.status();
    if !status.is_success() {
        let raw = read_capped(resp, ERROR_BODY_CAP).await.unwrap_or_default();
        let body = String::from_utf8_lossy(&raw).into_owned();
        return Err(Error::Upstream(status.as_u16(), truncate(&body, BODY_PREVIEW_CAP)));
    }
    match read_capped(resp, AUDIO_BODY_CAP).await {
        Ok(bytes) => Ok(bytes),
        Err(CappedReadError::TooLarge) => {
            Err(Error::Upstream(502, "audio body exceeded 64 MiB cap".into()))
        }
        Err(CappedReadError::Http(e)) => Err(Error::Http(e)),
    }
}

/// Cap a string at `max` bytes on a char boundary. Used for error previews
/// so we never log unbounded upstream content.
fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        return s.to_string();
    }
    let mut end = max;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    s[..end].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_body_shape() {
        let body = build_body("hello there");
        assert_eq!(body["text"], "hello there");
        assert_eq!(body["model_id"], MODEL_ID);
        assert_eq!(body["voice_settings"]["stability"], 0.5);
        assert_eq!(body["voice_settings"]["similarity_boost"], 0.75);
    }

    #[test]
    fn build_body_escapes_quotes() {
        // serde_json::Value handles escaping; emitting to string must round-trip.
        let body = build_body("say \"hi\"");
        let s = body.to_string();
        assert!(s.contains("say \\\"hi\\\""));
    }

    #[test]
    fn model_id_is_turbo_v2_5() {
        assert_eq!(MODEL_ID, "eleven_turbo_v2_5");
    }

    #[test]
    fn truncate_caps_long_strings() {
        let long = "a".repeat(10_000);
        let out = truncate(&long, 256);
        assert_eq!(out.len(), 256);
    }

    #[test]
    fn truncate_leaves_short_strings() {
        assert_eq!(truncate("hi", 256), "hi");
    }
}
