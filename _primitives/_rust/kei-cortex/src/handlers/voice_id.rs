//! Voice-id resolution — pluck `voice.voice_id` out of a pet.toml manifest.
//!
//! The `kei_pet::Voice` schema does not model `voice_id` as a typed field
//! (the crate tracks tone / humor only). Rather than widen the schema, we
//! re-read the manifest TOML here and pick the optional `voice.voice_id`
//! string directly. Absent / wrong-typed → fall back to the ElevenLabs
//! "Rachel" default.

use crate::error::AppError;
use crate::state::AppState;
use crate::validate;
use std::fs;

/// ElevenLabs "Rachel" — used when the pet manifest has no voice_id.
pub const DEFAULT_VOICE_ID: &str = "21m00Tcm4TlvDq8ikWAM";

/// Allowed shape for a pet's `voice_id` — limits blast radius if the TOML
/// is edited by hand. ElevenLabs ids are alphanumeric, <= 64 chars.
const MAX_VOICE_ID_LEN: usize = 64;

/// Load the pet manifest and pluck `voice.voice_id`. 404 if no such pet.
/// Validates `user_id` before path construction so this function is safe
/// to call from anywhere in the daemon.
pub fn resolve(state: &AppState, user_id: &str) -> Result<String, AppError> {
    validate::user_id(user_id)?;
    let path = state.config().pet_root.join(format!("{user_id}.toml"));
    if !path.exists() {
        return Err(AppError::NotFound(format!("pet {user_id}")));
    }
    let text = fs::read_to_string(&path)?;
    let raw: toml::Value = text
        .parse()
        .map_err(|e| AppError::BadRequest(format!("parse pet.toml: {e}")))?;
    let id = extract(&raw).unwrap_or_else(|| DEFAULT_VOICE_ID.to_string());
    sanity_check(&id)?;
    Ok(id)
}

/// Dig `voice.voice_id` out of a raw TOML value. Returns `None` on absent /
/// wrong-typed — caller falls back to the Rachel default.
pub fn extract(raw: &toml::Value) -> Option<String> {
    raw.get("voice")
        .and_then(|v| v.get("voice_id"))
        .and_then(|v| v.as_str())
        .map(str::to_string)
}

/// Reject voice_ids that look pathological (empty, too long, non-ascii).
pub fn sanity_check(id: &str) -> Result<(), AppError> {
    if id.is_empty() {
        return Err(AppError::BadRequest("voice_id is empty".into()));
    }
    if id.len() > MAX_VOICE_ID_LEN {
        return Err(AppError::BadRequest(format!(
            "voice_id len {} > {MAX_VOICE_ID_LEN}",
            id.len()
        )));
    }
    if !id.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(AppError::BadRequest(
            "voice_id must be ASCII alphanumeric".into(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_voice_id_present() {
        let raw: toml::Value = r#"
            schema = 1
            [voice]
            voice_id = "customABC"
        "#
        .parse()
        .unwrap();
        assert_eq!(extract(&raw), Some("customABC".into()));
    }

    #[test]
    fn extract_voice_id_absent_returns_none() {
        let raw: toml::Value = r#"
            schema = 1
            [voice]
            tone_primary = "warm"
        "#
        .parse()
        .unwrap();
        assert_eq!(extract(&raw), None);
    }

    #[test]
    fn sanity_check_rejects_pathological() {
        assert!(sanity_check("").is_err());
        assert!(sanity_check("has space").is_err());
        assert!(sanity_check("id/with/slash").is_err());
        assert!(sanity_check(&"a".repeat(MAX_VOICE_ID_LEN + 1)).is_err());
    }

    #[test]
    fn sanity_check_accepts_default() {
        assert!(sanity_check(DEFAULT_VOICE_ID).is_ok());
    }
}
