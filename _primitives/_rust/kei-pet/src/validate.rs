//! Validation rules R1–R19.
//!
//! `validate()` returns `Err(Vec<ValidationError>)` accumulating ALL errors,
//! not just the first. This lets `/pet-setup` and `kei-pet validate` surface
//! the full diagnostic in one pass.

use crate::schema::*;
use crate::SCHEMA_VERSION;
use thiserror::Error;

const PET_NAME_MAX: usize = 24;
const USER_NAME_MAX: usize = 48;

#[derive(Debug, Error, PartialEq)]
pub enum ValidationError {
    #[error("R1: schema version mismatch: found {0}, expected {SCHEMA_VERSION}")]
    SchemaVersion(u32),

    #[error("R2: identity.pet_name empty or exceeds {PET_NAME_MAX} chars")]
    PetNameInvalid,

    #[error("R2: identity.user_name empty or exceeds {USER_NAME_MAX} chars")]
    UserNameInvalid,

    #[error("R4: identity.languages must have ≥1 entry (ISO 639-1)")]
    LanguagesEmpty,

    #[error("R4: identity.languages[{0}] '{1}' not a valid ISO 639-1 2-letter code")]
    LanguageNotIso(usize, String),

    #[error("R6: voice.tone_secondary length {0} exceeds max 2")]
    ToneSecondaryTooMany(usize),

    #[error("R6: voice.tone_primary {0:?} present in tone_secondary (must differ)")]
    ToneSecondaryDuplicatePrimary(Tone),

    #[error("R10: edge.profanity = Never but profanity_languages is non-empty")]
    ProfanityLanguagesWhenNever,

    #[error("R10: edge.profanity_languages['{0}'] not in identity.languages")]
    ProfanityLanguageNotDeclared(String),

    #[error("R12: interests[{0}].topic '{1}' not slug-safe (must match [a-z0-9-]+ with no leading/trailing dash)")]
    InterestTopicNotSlug(usize, String),

    #[error("R14: interests[{0}].topic '{1}' also appears in forbidden.topics (contradiction)")]
    InterestForbiddenContradiction(usize, String),

    #[error("R16: routines[{0}].schedule '{1}' does not parse as known grammar (HH:MM / dow-HH:MM / every-Nh / no-commit-for-Nh / N-errors-in-N-calls)")]
    RoutineScheduleInvalid(usize, String),

    #[error("R18: forbidden.topics[{0}] is empty/whitespace")]
    ForbiddenTopicEmpty(usize),

    #[error("R18: forbidden.tone_patterns[{0}] is empty/whitespace")]
    ForbiddenTonePatternEmpty(usize),

    #[error("R19: meta.{0} is not a valid ISO-8601 timestamp: '{1}'")]
    MetaTimestampInvalid(&'static str, String),

    #[error("appearance.color_primary '{0}' not a valid hex colour (#RRGGBB)")]
    HexColorInvalid(String),
}

/// Run R1–R19. Returns `Err(Vec<ValidationError>)` on any failure.
pub fn validate(m: &PetManifest) -> Result<(), Vec<ValidationError>> {
    let mut errs = Vec::new();

    // R1 — schema version
    if m.schema != SCHEMA_VERSION {
        errs.push(ValidationError::SchemaVersion(m.schema));
    }

    // R2 — name bounds
    if m.identity.pet_name.is_empty() || m.identity.pet_name.chars().count() > PET_NAME_MAX {
        errs.push(ValidationError::PetNameInvalid);
    }
    if m.identity.user_name.is_empty() || m.identity.user_name.chars().count() > USER_NAME_MAX {
        errs.push(ValidationError::UserNameInvalid);
    }

    // R3 — addressing: enum-checked by serde at parse time; no runtime check needed.

    // R4 — languages: ≥1, each 2 ASCII-lower
    if m.identity.languages.is_empty() {
        errs.push(ValidationError::LanguagesEmpty);
    } else {
        for (i, lang) in m.identity.languages.iter().enumerate() {
            if lang.len() != 2 || !lang.chars().all(|c| c.is_ascii_lowercase()) {
                errs.push(ValidationError::LanguageNotIso(i, lang.clone()));
            }
        }
    }

    // R5, R7, R8, R11, R13, R15, R17 — enum membership guaranteed by serde.

    // R6 — tone_secondary cardinality + no duplicate of primary
    if m.voice.tone_secondary.len() > 2 {
        errs.push(ValidationError::ToneSecondaryTooMany(m.voice.tone_secondary.len()));
    }
    if m.voice.tone_secondary.contains(&m.voice.tone_primary) {
        errs.push(ValidationError::ToneSecondaryDuplicatePrimary(m.voice.tone_primary));
    }

    // R9 — profanity enum: serde-validated.

    // R10 — profanity/language consistency
    if m.edge.profanity == Profanity::Never && !m.edge.profanity_languages.is_empty() {
        errs.push(ValidationError::ProfanityLanguagesWhenNever);
    }
    if m.edge.profanity != Profanity::Never {
        for lang in &m.edge.profanity_languages {
            if !m.identity.languages.contains(lang) {
                errs.push(ValidationError::ProfanityLanguageNotDeclared(lang.clone()));
            }
        }
    }

    // R12 — interests[].topic slug-safe
    for (i, interest) in m.interests.iter().enumerate() {
        if !is_slug_safe(&interest.topic) {
            errs.push(ValidationError::InterestTopicNotSlug(i, interest.topic.clone()));
        }

        // R14 — no overlap with forbidden.topics
        if m.forbidden.topics.contains(&interest.topic) {
            errs.push(ValidationError::InterestForbiddenContradiction(i, interest.topic.clone()));
        }
    }

    // R16 — routine schedule grammar
    for (i, routine) in m.routines.iter().enumerate() {
        if !is_valid_schedule(&routine.schedule) {
            errs.push(ValidationError::RoutineScheduleInvalid(i, routine.schedule.clone()));
        }
    }

    // R17 — routines[].template existence is checked by the runtime (we don't
    // have filesystem context here). Left to /pet-setup verify step.

    // R18 — forbidden entries non-empty strings
    for (i, t) in m.forbidden.topics.iter().enumerate() {
        if t.trim().is_empty() {
            errs.push(ValidationError::ForbiddenTopicEmpty(i));
        }
    }
    for (i, t) in m.forbidden.tone_patterns.iter().enumerate() {
        if t.trim().is_empty() {
            errs.push(ValidationError::ForbiddenTonePatternEmpty(i));
        }
    }

    // R19 — meta ISO-8601
    if !is_iso8601(&m.meta.created_at) {
        errs.push(ValidationError::MetaTimestampInvalid("created_at", m.meta.created_at.clone()));
    }
    if !is_iso8601(&m.meta.last_tuned) {
        errs.push(ValidationError::MetaTimestampInvalid("last_tuned", m.meta.last_tuned.clone()));
    }

    // Bonus — hex colours (appearance is optional; only validate when present)
    if let Some(app) = &m.appearance {
        if !is_hex_colour(&app.color_primary) {
            errs.push(ValidationError::HexColorInvalid(app.color_primary.clone()));
        }
        if !is_hex_colour(&app.color_secondary) {
            errs.push(ValidationError::HexColorInvalid(app.color_secondary.clone()));
        }
    }

    if errs.is_empty() { Ok(()) } else { Err(errs) }
}

fn is_slug_safe(s: &str) -> bool {
    !s.is_empty()
        && s.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && !s.starts_with('-')
        && !s.ends_with('-')
        && !s.contains("--")
}

fn is_hex_colour(s: &str) -> bool {
    s.len() == 7
        && s.starts_with('#')
        && s[1..].chars().all(|c| c.is_ascii_hexdigit())
}

/// Recognised schedule grammar — strings that the runtime scheduler can act on.
///
/// Accepted shapes:
/// - `HH:MM`                                   — fixed daily time
/// - `dow-HH:MM` (e.g. `sun-10:00`)            — fixed weekly time
/// - `every-Nh`                                 — every N hours
/// - `no-commit-for-Nh`                         — idle trigger
/// - `N-errors-in-N-calls` (e.g. `3-errors-in-20-calls`)
fn is_valid_schedule(s: &str) -> bool {
    // HH:MM
    if parse_hhmm(s).is_some() { return true; }

    // dow-HH:MM
    if let Some((dow, rest)) = s.split_once('-') {
        let dows = ["mon","tue","wed","thu","fri","sat","sun"];
        if dows.contains(&dow) && parse_hhmm(rest).is_some() { return true; }
    }

    // every-Nh
    if let Some(rest) = s.strip_prefix("every-") {
        if let Some(n) = rest.strip_suffix('h') {
            if n.parse::<u32>().is_ok() { return true; }
        }
    }

    // no-commit-for-Nh
    if let Some(rest) = s.strip_prefix("no-commit-for-") {
        if let Some(n) = rest.strip_suffix('h') {
            if n.parse::<u32>().is_ok() { return true; }
        }
    }

    // N-errors-in-N-calls
    if let Some((n1, rest)) = s.split_once("-errors-in-") {
        if let Some(n2) = rest.strip_suffix("-calls") {
            if n1.parse::<u32>().is_ok() && n2.parse::<u32>().is_ok() { return true; }
        }
    }

    false
}

fn parse_hhmm(s: &str) -> Option<(u32, u32)> {
    let (h, m) = s.split_once(':')?;
    let h: u32 = h.parse().ok()?;
    let m: u32 = m.parse().ok()?;
    if h <= 23 && m <= 59 { Some((h, m)) } else { None }
}

fn is_iso8601(s: &str) -> bool {
    chrono::DateTime::parse_from_rfc3339(s).is_ok()
}
