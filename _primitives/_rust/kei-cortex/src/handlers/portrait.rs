//! `POST /api/v1/cortex/pet/:user_id/portrait/stylize` — take an uploaded
//! portrait, run it through fal.ai Flux 2 Pro, clone a bundled Cubism rig,
//! and swap `texture_00.png` with the stylized image.
//!
//! Constructor Pattern: the handler body is a 5-step pipeline and each step
//! is a sibling function <30 LOC. No business logic in the handler body
//! itself beyond orchestration + timing.
//!
//! Concurrency: installs are serialised per `user_id` via `AppState::user_lock`
//! so two concurrent requests for the same user cannot race on the clone /
//! rename. Different users run in parallel.

use crate::error::AppError;
use crate::fal::{self, Style};
use crate::rig_clone;
use crate::state::AppState;
use crate::validate;
use axum::extract::{Multipart, Path, State};
use axum::Json;
use serde::Serialize;
use std::time::Instant;

// TODO: face detect via opencv later — for now we trust the client and only
// validate MIME + byte length.
const MAX_BYTES: usize = 10 * 1024 * 1024;
/// Upper bound on short text fields (`style`, `base_model`) — prevents an
/// attacker from streaming gigabytes in a non-file field.
const MAX_FIELD_BYTES: usize = 256;
const ALLOWED_BASE_MODELS: &[&str] = &["haru", "mao", "hiyori", "mark"];

/// Response body returned to the Setup UI after a successful stylize run.
#[derive(Debug, Serialize)]
pub struct StylizeResponse {
    pub rig_dir: String,
    pub model_json: String,
    pub preview_url: String,
    pub took_ms: u64,
}

/// Parsed, validated multipart form payload.
struct UploadForm {
    bytes: Vec<u8>,
    style: Style,
    base_model: String,
}

/// Handler entry point — wired in `routes.rs`.
pub async fn stylize(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    multipart: Multipart,
) -> Result<Json<StylizeResponse>, AppError> {
    let t0 = Instant::now();
    validate::user_id(&user_id)?;
    let form = parse_form(multipart).await?;
    let stylized = run_fal(&form.bytes, form.style).await?;
    let (rig_dir, model_json, preview_url) =
        install_rig(&state, &user_id, &form.base_model, &stylized).await?;
    Ok(Json(StylizeResponse {
        rig_dir,
        model_json,
        preview_url,
        took_ms: t0.elapsed().as_millis() as u64,
    }))
}

/// Walk the multipart payload, collecting `file` / `style` / `base_model`.
async fn parse_form(mut mp: Multipart) -> Result<UploadForm, AppError> {
    let mut bytes: Option<Vec<u8>> = None;
    let mut style_str = String::from("anime-cute");
    let mut base_model = String::from("haru");
    while let Some(field) = mp
        .next_field()
        .await
        .map_err(|e| AppError::BadRequest(format!("multipart: {e}")))?
    {
        match field.name().unwrap_or("") {
            "file" => bytes = Some(read_file_field(field).await?),
            "style" => style_str = read_short_text(field, "style").await?,
            "base_model" => base_model = read_short_text(field, "base_model").await?,
            _ => {}
        }
    }
    let bytes = bytes.ok_or_else(|| AppError::BadRequest("missing file field".into()))?;
    validate_base_model(&base_model)?;
    Ok(UploadForm {
        bytes,
        style: Style::from_wire(&style_str),
        base_model,
    })
}

/// Read a file field, enforcing MIME prefix and 10 MiB cap.
async fn read_file_field(field: axum::extract::multipart::Field<'_>) -> Result<Vec<u8>, AppError> {
    let ct = field.content_type().unwrap_or("").to_string();
    if !(ct.starts_with("image/png") || ct.starts_with("image/jpeg")) {
        return Err(AppError::BadRequest(format!(
            "unsupported content-type {ct:?}"
        )));
    }
    let bytes = field
        .bytes()
        .await
        .map_err(|e| AppError::BadRequest(format!("body: {e}")))?;
    if bytes.len() > MAX_BYTES {
        return Err(AppError::PayloadTooLarge(format!(
            "{} bytes > 10 MiB",
            bytes.len()
        )));
    }
    Ok(bytes.to_vec())
}

/// Read a short text field, enforcing `MAX_FIELD_BYTES` BEFORE we return
/// the string. The whole payload is materialised in memory because axum
/// does not let us cap per-field on the multipart stream directly; the
/// outer `DefaultBodyLimit` covers the total envelope.
async fn read_short_text(
    field: axum::extract::multipart::Field<'_>,
    name: &'static str,
) -> Result<String, AppError> {
    let bytes = field
        .bytes()
        .await
        .map_err(|e| AppError::BadRequest(format!("{name}: {e}")))?;
    if bytes.len() > MAX_FIELD_BYTES {
        return Err(AppError::PayloadTooLarge(format!(
            "{name} {} bytes > {MAX_FIELD_BYTES}",
            bytes.len()
        )));
    }
    let s = std::str::from_utf8(&bytes)
        .map_err(|e| AppError::BadRequest(format!("{name} not utf-8: {e}")))?;
    Ok(s.to_string())
}

fn validate_base_model(base: &str) -> Result<(), AppError> {
    if ALLOWED_BASE_MODELS.iter().any(|m| *m == base) {
        Ok(())
    } else {
        Err(AppError::BadRequest(format!(
            "base_model must be one of {ALLOWED_BASE_MODELS:?}"
        )))
    }
}

/// Call the Flux client, mapping its errors onto 504 / 502 as appropriate.
async fn run_fal(bytes: &[u8], style: Style) -> Result<Vec<u8>, AppError> {
    match fal::stylize(bytes, style).await {
        Ok(b) => Ok(b),
        Err(fal::Error::Timeout) => Err(AppError::GatewayTimeout("fal.ai > 60s".into())),
        Err(fal::Error::NoApiKey) => Err(AppError::Internal("FAL_KEY not set".into())),
        Err(other) => Err(AppError::BadGateway(format!("fal: {other}"))),
    }
}

/// Clone the base rig on a blocking thread and return the three URL strings.
/// Serialised per-user via the `AppState::user_lock` registry so concurrent
/// stylize requests for the same user cannot race on the install.
async fn install_rig(
    state: &AppState,
    user_id: &str,
    base_model: &str,
    stylized: &[u8],
) -> Result<(String, String, String), AppError> {
    let base_dir = state.config().live2d_samples_dir.join(base_model);
    let target_dir = state
        .config()
        .live2d_samples_dir
        .join(format!("custom-{user_id}"));
    let lock = state.user_lock(user_id).await;
    let _guard = lock.lock().await;
    let stylized_owned = stylized.to_vec();
    let base_owned = base_dir.clone();
    let target_owned = target_dir.clone();
    tokio::task::spawn_blocking(move || {
        install_then_preview(&base_owned, &target_owned, &stylized_owned)
    })
    .await
    .map_err(|e| AppError::Internal(format!("rig task join: {e}")))??;
    Ok(build_urls(&target_dir, base_model, user_id))
}

/// Blocking helper: atomically install the rig AND drop a `preview.png`
/// alongside it. The preview write is not inside the atomic swap (fine —
/// worst case the preview is briefly stale; the rig directory itself is
/// coherent).
fn install_then_preview(
    base: &std::path::Path,
    target: &std::path::Path,
    stylized: &[u8],
) -> Result<(), AppError> {
    rig_clone::install_rig(base, target, stylized)
        .map_err(|e| AppError::Internal(format!("rig_clone: {e}")))?;
    std::fs::write(target.join("preview.png"), stylized)
        .map_err(|e| AppError::Internal(format!("preview: {e}")))?;
    Ok(())
}

/// Build the three public URLs. `model_json` is the first `*.model3.json`
/// inside the cloned dir; fall back to a conventional name if none found.
fn build_urls(
    target_dir: &std::path::Path,
    base_model: &str,
    user_id: &str,
) -> (String, String, String) {
    let rig_dir = format!("live2d-models/custom-{user_id}/");
    let model_name =
        find_model_json(target_dir).unwrap_or_else(|| format!("{base_model}.model3.json"));
    let model_json = format!("/live2d-models/custom-{user_id}/{model_name}");
    let preview_url = format!("/live2d-models/custom-{user_id}/preview.png");
    (rig_dir, model_json, preview_url)
}

/// Return the first `*.model3.json` basename inside `dir`, if any.
fn find_model_json(dir: &std::path::Path) -> Option<String> {
    let rd = std::fs::read_dir(dir).ok()?;
    for entry in rd.flatten() {
        let name = entry.file_name();
        let s = name.to_string_lossy();
        if s.ends_with(".model3.json") {
            return Some(s.into_owned());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_base_model_accepts_allowed() {
        for m in ALLOWED_BASE_MODELS {
            assert!(validate_base_model(m).is_ok());
        }
    }

    #[test]
    fn validate_base_model_rejects_unknown() {
        assert!(validate_base_model("evil").is_err());
        assert!(validate_base_model("").is_err());
    }

    #[test]
    fn build_urls_format() {
        let tmp = std::env::temp_dir();
        let (rig_dir, model_json, preview_url) = build_urls(&tmp, "haru", "alice");
        assert_eq!(rig_dir, "live2d-models/custom-alice/");
        assert!(model_json.ends_with(".model3.json"));
        assert_eq!(preview_url, "/live2d-models/custom-alice/preview.png");
    }
}
