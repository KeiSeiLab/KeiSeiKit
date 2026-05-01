//! `POST /api/v1/cortex/stt` — multipart audio → Whisper → JSON transcript.
//!
//! Constructor Pattern: the handler body is a 3-step pipeline (parse form,
//! call upstream, wrap JSON), each step a sibling function <30 LOC. We
//! enforce a 25 MiB per-field cap (OpenAI's documented Whisper limit) and
//! whitelist the MIME prefixes we send to Whisper's ffmpeg layer.

use crate::error::AppError;
use crate::whisper_local;
use axum::extract::Multipart;
use axum::Json;
use serde::Serialize;

/// OpenAI Whisper's own documented cap on the request audio field.
const MAX_AUDIO_BYTES: usize = 25 * 1024 * 1024;

/// Wire-level response body for a successful transcription.
#[derive(Debug, Serialize)]
pub struct TranscribeResponse {
    pub transcript: String,
}

/// Parsed, validated multipart form payload.
struct AudioForm {
    bytes: Vec<u8>,
    mime: String,
}

/// Handler entry point — wired in `routes.rs`.
pub async fn transcribe(multipart: Multipart) -> Result<Json<TranscribeResponse>, AppError> {
    let form = parse_form(multipart).await?;
    let transcript = run_whisper(form.bytes, &form.mime).await?;
    Ok(Json(TranscribeResponse { transcript }))
}

/// Walk the multipart payload, collecting the `audio` field.
async fn parse_form(mut mp: Multipart) -> Result<AudioForm, AppError> {
    while let Some(field) = mp
        .next_field()
        .await
        .map_err(|e| AppError::BadRequest(format!("multipart: {e}")))?
    {
        if field.name().unwrap_or("") == "audio" {
            return read_audio_field(field).await;
        }
    }
    Err(AppError::BadRequest("missing audio field".into()))
}

/// Read the `audio` field, enforcing MIME prefix and 25 MiB cap.
async fn read_audio_field(
    field: axum::extract::multipart::Field<'_>,
) -> Result<AudioForm, AppError> {
    let mime = field.content_type().unwrap_or("").to_string();
    validate_mime(&mime)?;
    let bytes = field
        .bytes()
        .await
        .map_err(|e| AppError::BadRequest(format!("audio body: {e}")))?;
    if bytes.len() > MAX_AUDIO_BYTES {
        return Err(AppError::PayloadTooLarge(format!(
            "{} bytes > 25 MiB",
            bytes.len()
        )));
    }
    if bytes.is_empty() {
        return Err(AppError::BadRequest("audio field is empty".into()));
    }
    Ok(AudioForm {
        bytes: bytes.to_vec(),
        mime,
    })
}

/// Whitelist the MIME prefixes Whisper's ffmpeg layer can demux reliably.
fn validate_mime(mime: &str) -> Result<(), AppError> {
    let ok = mime.starts_with("audio/webm")
        || mime.starts_with("audio/wav")
        || mime.starts_with("audio/x-wav")
        || mime.starts_with("audio/mpeg")
        || mime.starts_with("audio/mp3")
        || mime.starts_with("audio/mp4")
        || mime.starts_with("audio/m4a")
        || mime.starts_with("audio/ogg");
    if ok {
        Ok(())
    } else {
        Err(AppError::BadRequest(format!(
            "unsupported audio content-type {mime:?}"
        )))
    }
}

/// Call local faster-whisper worker, mapping its errors onto HTTP statuses.
async fn run_whisper(bytes: Vec<u8>, mime: &str) -> Result<String, AppError> {
    match whisper_local::transcribe(bytes, mime).await {
        Ok(text) => Ok(text),
        Err(whisper_local::Error::Timeout(d)) => {
            Err(AppError::GatewayTimeout(format!("whisper {d:?}")))
        }
        Err(whisper_local::Error::WorkerMissing(p)) => Err(AppError::Internal(format!(
            "whisper worker not found at {}",
            p.display()
        ))),
        Err(whisper_local::Error::WorkerFailed { code, stderr }) => Err(AppError::BadGateway(
            format!("whisper exit {code}: {stderr}"),
        )),
        Err(whisper_local::Error::Io(e)) => Err(AppError::Internal(format!("whisper io: {e}"))),
        Err(whisper_local::Error::PythonMissing) => Err(AppError::Internal(
            "python3 not found — set KEI_WHISPER_PYTHON".into(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_mime_accepts_common_types() {
        assert!(validate_mime("audio/webm").is_ok());
        assert!(validate_mime("audio/webm;codecs=opus").is_ok());
        assert!(validate_mime("audio/wav").is_ok());
        assert!(validate_mime("audio/mpeg").is_ok());
        assert!(validate_mime("audio/mp4").is_ok());
        assert!(validate_mime("audio/ogg").is_ok());
    }

    #[test]
    fn validate_mime_rejects_bad_types() {
        assert!(validate_mime("").is_err());
        assert!(validate_mime("video/mp4").is_err());
        assert!(validate_mime("image/png").is_err());
        assert!(validate_mime("application/json").is_err());
    }

    #[test]
    fn max_bytes_is_25_mib() {
        assert_eq!(MAX_AUDIO_BYTES, 25 * 1024 * 1024);
    }

    #[test]
    fn response_shape_roundtrips() {
        let r = TranscribeResponse {
            transcript: "hello".into(),
        };
        let s = serde_json::to_string(&r).unwrap();
        assert!(s.contains("\"transcript\":\"hello\""));
    }
}
