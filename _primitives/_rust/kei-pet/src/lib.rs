//! kei-pet — pet persona manifest parse/validate/overlay.
//!
//! Scope boundaries: this crate implements a standard TOML-backed persona
//! manifest. Identity is Ed25519 (RFC 8032). Cache/projection patterns are
//! standard CQRS. NO imports, references, or conceptual mentions of sibling
//! research-grade projects are permitted in this crate.

pub mod schema;
pub mod validate;
pub mod overlay;
pub mod identity;
pub mod memory;
pub mod evolution;
pub mod bridge;
pub mod fleet;
pub mod reflect;
pub mod recall;
pub mod templates;
pub mod injection_check;
mod injection_check_binary;
mod injection_check_textual;

pub use schema::PetManifest;
pub use validate::{ValidationError, validate};
pub use overlay::system_prompt;
pub use identity::{generate_keypair, user_id_from_pubkey, Keypair};
pub use bridge::{AgentSpawnRequest, compose_prompt_with_pet};
pub use templates::{load_template, list_templates, PetTemplate};

/// Current schema version written by this crate.
pub const SCHEMA_VERSION: u32 = 1;

/// Parse TOML text → `PetManifest`, running full validation.
///
/// Returns the manifest on success, or the accumulated validation errors.
pub fn parse(toml_text: &str) -> Result<PetManifest, anyhow::Error> {
    let manifest: PetManifest = toml::from_str(toml_text)?;
    validate(&manifest).map_err(|errs| {
        anyhow::anyhow!(
            "pet.toml validation failed ({} error{}):\n{}",
            errs.len(),
            if errs.len() == 1 { "" } else { "s" },
            errs.iter().map(|e| format!("  - {e}")).collect::<Vec<_>>().join("\n")
        )
    })?;
    Ok(manifest)
}

/// Serialize a validated manifest back to TOML text.
pub fn to_toml(manifest: &PetManifest) -> Result<String, toml::ser::Error> {
    toml::to_string_pretty(manifest)
}
