//! `POST /api/v1/cortex/pet/:user_id/tts` — JSON text → ElevenLabs → mp3.
//!
//! Constructor Pattern: the handler body is a 4-step pipeline (validate,
//! resolve voice_id, call upstream, wrap response), each step a sibling
//! function <30 LOC. Voice id comes from `voice.voice_id` in the pet
//! manifest; absent → ElevenLabs "Rachel" default (see `voice_id` module).

use crate::elevenlabs;
use crate::error::AppError;
use crate::handlers::voice_id;
use crate::state::AppState;
use crate::validate;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{header, StatusCode};
use axum::response::Response;
use axum::Json;
use serde::Deserialize;

/// ElevenLabs charges by characters — we enforce a 3000-char ceiling.
const MAX_TEXT_CHARS: usize = 3000;

/// Wire-level request body.
#[derive(Debug, Deserialize)]
pub struct TtsRequest {
    pub text: String,
}

/// Handler entry point — wired in `routes.rs`.
pub async fn synthesize(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    Json(req): Json<TtsRequest>,
) -> Result<Response, AppError> {
    validate::user_id(&user_id)?;
    validate_text(&req.text)?;
    let id = voice_id::resolve(&state, &user_id)?;
    let bytes = run_elevenlabs(&id, &req.text).await?;
    build_audio_response(bytes)
}

/// Enforce non-empty text and the 3000-char ceiling.
fn validate_text(text: &str) -> Result<(), AppError> {
    if text.is_empty() {
        return Err(AppError::BadRequest("text is empty".into()));
    }
    let chars = text.chars().count();
    if chars > MAX_TEXT_CHARS {
        return Err(AppError::PayloadTooLarge(format!(
            "{chars} chars > {MAX_TEXT_CHARS}"
        )));
    }
    Ok(())
}

/// Call ElevenLabs, mapping its errors onto 504 / 502 / 500 as appropriate.
async fn run_elevenlabs(voice_id: &str, text: &str) -> Result<Vec<u8>, AppError> {
    match elevenlabs::synthesize(voice_id, text).await {
        Ok(bytes) => Ok(bytes),
        Err(elevenlabs::Error::Timeout) => {
            Err(AppError::GatewayTimeout("elevenlabs > 60s".into()))
        }
        Err(elevenlabs::Error::NoApiKey) => {
            Err(AppError::Internal("ELEVENLABS_API_KEY not set".into()))
        }
        Err(elevenlabs::Error::Upstream(status, body)) => Err(AppError::BadGateway(format!(
            "elevenlabs {status}: {body}"
        ))),
        Err(other) => Err(AppError::BadGateway(format!("elevenlabs: {other}"))),
    }
}

/// Wrap mp3 bytes in an `audio/mpeg` HTTP response.
fn build_audio_response(bytes: Vec<u8>) -> Result<Response, AppError> {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "audio/mpeg")
        .header(header::CONTENT_LENGTH, bytes.len())
        .body(Body::from(bytes))
        .map_err(|e| AppError::Internal(format!("build response: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_text_rejects_empty() {
        assert!(validate_text("").is_err());
    }

    #[test]
    fn validate_text_accepts_short() {
        assert!(validate_text("hello there").is_ok());
    }

    #[test]
    fn validate_text_rejects_overlong() {
        let s = "a".repeat(MAX_TEXT_CHARS + 1);
        assert!(matches!(
            validate_text(&s),
            Err(AppError::PayloadTooLarge(_))
        ));
    }

    #[test]
    fn validate_text_counts_chars_not_bytes() {
        // 2-byte chars (cyrillic) — 3000 of them should PASS the char check.
        let s = "я".repeat(MAX_TEXT_CHARS);
        assert!(validate_text(&s).is_ok());
        let s2 = "я".repeat(MAX_TEXT_CHARS + 1);
        assert!(validate_text(&s2).is_err());
    }

    #[test]
    fn build_audio_response_sets_content_type() {
        let resp = build_audio_response(vec![0xff, 0xfb, 0x00]).unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(header::CONTENT_TYPE).unwrap(),
            "audio/mpeg"
        );
        assert_eq!(resp.headers().get(header::CONTENT_LENGTH).unwrap(), "3");
    }
}
