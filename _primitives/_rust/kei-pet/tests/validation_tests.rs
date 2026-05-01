//! Integration tests for the R1–R19 validator.
//!
//! Each `reject_*` test asserts a specific rule fires with a specific variant.
//! Accepting cases parse `examples/minimal.toml` and `examples/full.toml`
//! unmodified.

use kei_pet::{parse, validate};
use kei_pet::schema::*;
use kei_pet::validate::ValidationError;

const MINIMAL: &str = include_str!("../examples/minimal.toml");
const FULL:    &str = include_str!("../examples/full.toml");

fn base() -> PetManifest {
    parse(MINIMAL).expect("minimal.toml must validate")
}

#[test]
fn accept_minimal_example() {
    let m = parse(MINIMAL).unwrap();
    assert_eq!(m.schema, 1);
    assert_eq!(m.identity.pet_name, "Kei");
}

#[test]
fn accept_full_example() {
    let m = parse(FULL).unwrap();
    assert_eq!(m.interests.len(), 2);
    assert_eq!(m.routines.len(), 4);
    assert!(m.appearance.is_some());
    assert!(m.room.is_some());
    assert!(m.privacy.is_some());
}

// ─────────────────────────── R1 schema version ───────────────────────────

#[test]
fn r1_wrong_schema() {
    let mut m = base();
    m.schema = 99;
    let errs = validate(&m).unwrap_err();
    assert!(errs.iter().any(|e| matches!(e, ValidationError::SchemaVersion(99))));
}

// ──────────────────────────── R2 name bounds ─────────────────────────────

#[test]
fn r2_empty_pet_name() {
    let mut m = base();
    m.identity.pet_name.clear();
    assert!(validate(&m).unwrap_err().contains(&ValidationError::PetNameInvalid));
}

#[test]
fn r2_pet_name_too_long() {
    let mut m = base();
    m.identity.pet_name = "a".repeat(25);
    assert!(validate(&m).unwrap_err().contains(&ValidationError::PetNameInvalid));
}

#[test]
fn r2_empty_user_name() {
    let mut m = base();
    m.identity.user_name.clear();
    assert!(validate(&m).unwrap_err().contains(&ValidationError::UserNameInvalid));
}

// ───────────────────────────── R4 languages ──────────────────────────────

#[test]
fn r4_empty_languages() {
    let mut m = base();
    m.identity.languages.clear();
    assert!(validate(&m).unwrap_err().contains(&ValidationError::LanguagesEmpty));
}

#[test]
fn r4_non_iso_language() {
    let mut m = base();
    m.identity.languages = vec!["english".to_string()];
    let errs = validate(&m).unwrap_err();
    assert!(errs.iter().any(|e| matches!(e, ValidationError::LanguageNotIso(0, s) if s == "english")));
}

// ─────────────────────────────── R6 tones ────────────────────────────────

#[test]
fn r6_too_many_secondary_tones() {
    let mut m = base();
    m.voice.tone_secondary = vec![Tone::Warm, Tone::Sarcastic, Tone::Supportive];
    let errs = validate(&m).unwrap_err();
    assert!(errs.iter().any(|e| matches!(e, ValidationError::ToneSecondaryTooMany(3))));
}

#[test]
fn r6_primary_duplicated_in_secondary() {
    let mut m = base();
    m.voice.tone_primary = Tone::Warm;
    m.voice.tone_secondary = vec![Tone::Warm];
    let errs = validate(&m).unwrap_err();
    assert!(errs.iter().any(|e| matches!(e, ValidationError::ToneSecondaryDuplicatePrimary(Tone::Warm))));
}

// ───────────────────────── R10 profanity/languages ───────────────────────

#[test]
fn r10_never_with_language_list_populated() {
    let mut m = base();
    m.edge.profanity = Profanity::Never;
    m.edge.profanity_languages = vec!["en".to_string()];
    assert!(validate(&m).unwrap_err().contains(&ValidationError::ProfanityLanguagesWhenNever));
}

#[test]
fn r10_profanity_language_not_in_identity() {
    let mut m = base();
    m.edge.profanity = Profanity::MirrorUser;
    m.edge.profanity_languages = vec!["de".to_string()];
    let errs = validate(&m).unwrap_err();
    assert!(errs.iter().any(|e| matches!(e, ValidationError::ProfanityLanguageNotDeclared(s) if s == "de")));
}

// ──────────────────────────── R12 slug-safe ──────────────────────────────

#[test]
fn r12_non_slug_topic() {
    let mut m = base();
    m.interests = vec![Interest {
        topic: "Distributed Systems".into(),
        depth: Depth::Expert,
        freshness: Freshness::Weekly,
        vault_path: String::new(),
        last_refresh: String::new(),
    }];
    let errs = validate(&m).unwrap_err();
    assert!(errs.iter().any(|e| matches!(e, ValidationError::InterestTopicNotSlug(0, _))));
}

#[test]
fn r12_leading_dash() {
    let mut m = base();
    m.interests = vec![Interest {
        topic: "-bad".into(),
        depth: Depth::Expert,
        freshness: Freshness::Weekly,
        vault_path: String::new(),
        last_refresh: String::new(),
    }];
    assert!(validate(&m).is_err());
}

// ──────────────────────── R14 interest/forbidden overlap ─────────────────

#[test]
fn r14_interest_in_forbidden() {
    let mut m = base();
    m.interests = vec![Interest {
        topic: "ai-hype".into(),
        depth: Depth::Shallow,
        freshness: Freshness::OnDemand,
        vault_path: String::new(),
        last_refresh: String::new(),
    }];
    m.forbidden.topics = vec!["ai-hype".into()];
    let errs = validate(&m).unwrap_err();
    assert!(errs.iter().any(|e| matches!(e, ValidationError::InterestForbiddenContradiction(0, _))));
}

// ────────────────────────────── R16 schedules ────────────────────────────

#[test]
fn r16_valid_schedules_accepted() {
    let mut m = base();
    for sched in &[
        "09:00", "23:59", "00:00",
        "sun-10:00", "mon-08:30",
        "every-4h", "no-commit-for-3h",
        "3-errors-in-20-calls",
    ] {
        m.routines = vec![Routine {
            kind: RoutineKind::Custom,
            schedule: (*sched).to_string(),
            template: "pet-routine-morning".to_string(),
            enabled: true,
        }];
        validate(&m).unwrap_or_else(|e| panic!("schedule '{sched}' rejected: {e:?}"));
    }
}

#[test]
fn r16_invalid_schedule() {
    let mut m = base();
    m.routines = vec![Routine {
        kind: RoutineKind::Custom,
        schedule: "whenever".into(),
        template: "pet-routine-morning".into(),
        enabled: true,
    }];
    let errs = validate(&m).unwrap_err();
    assert!(errs.iter().any(|e| matches!(e, ValidationError::RoutineScheduleInvalid(0, s) if s == "whenever")));
}

#[test]
fn r16_invalid_hour() {
    let mut m = base();
    m.routines = vec![Routine {
        kind: RoutineKind::Custom,
        schedule: "25:00".into(),
        template: "x".into(),
        enabled: true,
    }];
    assert!(validate(&m).is_err());
}

// ────────────────────────────── R18 empty strings ────────────────────────

#[test]
fn r18_empty_forbidden_topic() {
    let mut m = base();
    m.forbidden.topics = vec!["   ".into()];
    let errs = validate(&m).unwrap_err();
    assert!(errs.iter().any(|e| matches!(e, ValidationError::ForbiddenTopicEmpty(0))));
}

// ──────────────────────────────── R19 ISO-8601 ───────────────────────────

#[test]
fn r19_bad_timestamp() {
    let mut m = base();
    m.meta.created_at = "yesterday".into();
    let errs = validate(&m).unwrap_err();
    assert!(errs.iter().any(|e| matches!(e, ValidationError::MetaTimestampInvalid("created_at", _))));
}

// ─────────────────────────────── hex colours ─────────────────────────────

#[test]
fn hex_color_invalid() {
    let mut m = parse(FULL).unwrap();
    if let Some(ref mut app) = m.appearance {
        app.color_primary = "brown".into();
    }
    let errs = validate(&m).unwrap_err();
    assert!(errs.iter().any(|e| matches!(e, ValidationError::HexColorInvalid(s) if s == "brown")));
}

// ─────────────────────── multiple errors accumulate ──────────────────────

#[test]
fn errors_accumulate_not_fail_fast() {
    let mut m = base();
    m.schema = 99;                     // R1
    m.identity.pet_name.clear();       // R2
    m.identity.languages.clear();      // R4
    let errs = validate(&m).unwrap_err();
    // Ensure we got ≥3 distinct errors, proving we accumulated rather than
    // short-circuited on the first.
    assert!(errs.len() >= 3, "expected ≥3 accumulated errors, got {}: {errs:?}", errs.len());
}

// ─────────────────────────────── overlay smoke ───────────────────────────

#[test]
fn overlay_contains_names() {
    let m = parse(FULL).unwrap();
    let overlay = kei_pet::system_prompt(&m);
    assert!(overlay.contains("Kei"));
    assert!(overlay.contains("Denis"));
    assert!(overlay.contains("distributed-systems"));
    assert!(overlay.contains("politics"));
}
