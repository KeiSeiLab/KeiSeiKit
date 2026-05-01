//! Preset pet persona templates.
//!
//! Each template is a bundled, schema-valid TOML seed parsed at runtime
//! via `crate::parse`. The set intentionally covers five distinct
//! personas so `/pet-setup` can offer one-click starting points.

use crate::schema::PetManifest;

/// The five preset persona archetypes shipped with kei-pet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PetTemplate {
    Friend,
    Tutor,
    Coach,
    TherapistCompanion,
    ProductivityPartner,
}

/// Load a preset template and return the fully-validated manifest.
///
/// All five bundled templates are verified to pass `validate()` by the
/// `all_five_templates_pass_validation` integration test.
pub fn load_template(t: PetTemplate) -> Result<PetManifest, anyhow::Error> {
    let toml_str = match t {
        PetTemplate::Friend => include_str!("../templates/friend.toml"),
        PetTemplate::Tutor => include_str!("../templates/tutor.toml"),
        PetTemplate::Coach => include_str!("../templates/coach.toml"),
        PetTemplate::TherapistCompanion => {
            include_str!("../templates/therapist-companion.toml")
        }
        PetTemplate::ProductivityPartner => {
            include_str!("../templates/productivity-partner.toml")
        }
    };
    crate::parse(toml_str)
}

/// Stable-ordered list of templates with short human descriptions.
///
/// Order is the same as enum declaration (Friend → ProductivityPartner).
pub fn list_templates() -> Vec<(PetTemplate, &'static str)> {
    vec![
        (PetTemplate::Friend, "Warm casual companion"),
        (PetTemplate::Tutor, "Precise teaching assistant"),
        (PetTemplate::Coach, "Direct improvement coach"),
        (
            PetTemplate::TherapistCompanion,
            "Gentle listening companion (not a replacement for professional care)",
        ),
        (
            PetTemplate::ProductivityPartner,
            "Focus + routine accountability",
        ),
    ]
}
