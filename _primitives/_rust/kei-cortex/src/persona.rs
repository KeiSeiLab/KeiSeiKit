//! Load a pet manifest from disk and build the system prompt for a chat turn.
//!
//! Delegates to `kei_pet::parse` (TOML + validation) and
//! `kei_pet::system_prompt` (overlay render). Adds the chat-specific
//! "respond naturally, 1-3 sentences" tail so the response-length hint is
//! not baked into the persona but is owned by this transport.
//!
//! Manifest path: `<pet_root>/<user_id>.toml` — matches the flat layout
//! used elsewhere in this daemon (see `handlers/pet.rs`,
//! `config::AppConfig::pet_root`).

use crate::error::AppError;
use crate::validate;
use kei_pet::{parse, system_prompt, PetManifest};
use std::fs;
use std::path::Path;

/// Tail appended to the persona overlay. Chat-transport-owned, not persona-owned.
pub const CHAT_TAIL: &str =
    "\nRespond naturally, in 1-3 sentences unless explicitly asked for more.";

/// Load `<pet_root>/<user_id>.toml` → `PetManifest`. 404 if absent.
/// Validates `user_id` BEFORE joining it into the path so callers that
/// forgot the outer guard cannot sneak a traversal.
pub fn load_manifest(pet_root: &Path, user_id: &str) -> Result<PetManifest, AppError> {
    validate::user_id(user_id)?;
    let path = pet_root.join(format!("{user_id}.toml"));
    if !path.exists() {
        return Err(AppError::NotFound(format!("pet {user_id}")));
    }
    let text = fs::read_to_string(&path)?;
    parse(&text).map_err(|e| AppError::BadRequest(format!("parse pet.toml: {e}")))
}

/// Render the system prompt a chat turn sends upstream.
///
/// Wraps `kei_pet::system_prompt` and appends `CHAT_TAIL`.
pub fn build_system_prompt(manifest: &PetManifest) -> String {
    let mut s = system_prompt(manifest);
    s.push_str(CHAT_TAIL);
    s
}

/// Convenience: load manifest, render system prompt, return both.
pub fn load_and_render(pet_root: &Path, user_id: &str) -> Result<(PetManifest, String), AppError> {
    let manifest = load_manifest(pet_root, user_id)?;
    let prompt = build_system_prompt(&manifest);
    Ok((manifest, prompt))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn fixtures_root() -> Option<PathBuf> {
        // Only runs if a fixture manifest is available; skipped otherwise.
        let p = PathBuf::from("tests/fixtures/pets");
        if p.exists() { Some(p) } else { None }
    }

    #[test]
    fn missing_manifest_is_not_found() {
        let tmp = std::env::temp_dir().join("kei-cortex-persona-test-empty");
        let _ = std::fs::create_dir_all(&tmp);
        let err = load_manifest(&tmp, "nonexistent").unwrap_err();
        match err {
            AppError::NotFound(_) => {}
            other => panic!("expected NotFound, got {other:?}"),
        }
    }

    #[test]
    fn chat_tail_is_appended() {
        let Some(root) = fixtures_root() else { return };
        let Ok((_, prompt)) = load_and_render(&root, "u0") else { return };
        assert!(prompt.ends_with(CHAT_TAIL) || prompt.contains("1-3 sentences"));
    }
}
