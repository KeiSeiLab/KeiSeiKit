//! Hermetic tests for `kei_pet::reflect::propose_tune`.
//!
//! Each test builds an in-memory `PetManifest` (no disk, no TOML parsing)
//! so the logic is tested in isolation from schema serialization.

use kei_pet::reflect::{propose_tune, CorrectionSignal, ProposedChange};
use kei_pet::schema::{
    Addressing, Directness, Edge, Forbidden, HumorFrequency, HumorStyle,
    Identity, Initiative, Meta, PetManifest, Profanity, Tone, Voice,
};

fn base_manifest() -> PetManifest {
    PetManifest {
        schema: 1,
        identity: Identity {
            pet_name: "Kei".into(),
            user_name: "Alex".into(),
            addressing: Addressing::ByName,
            languages: vec!["en".into()],
        },
        voice: Voice {
            tone_primary: Tone::Neutral,
            tone_secondary: vec![],
            humor_style: HumorStyle::None,
            humor_frequency: HumorFrequency::Rare,
        },
        edge: Edge {
            profanity: Profanity::Never,
            profanity_languages: vec![],
            directness: Directness::Balanced,
            initiative: Initiative::Wait,
        },
        appearance: None,
        room: None,
        privacy: None,
        interests: vec![],
        routines: vec![],
        forbidden: Forbidden {
            topics: vec![],
            tone_patterns: vec![],
        },
        meta: Meta {
            schema_version_written_by: "kei-pet 0.1.0".into(),
            created_at: "2026-04-23T12:00:00Z".into(),
            last_tuned: "2026-04-23T12:00:00Z".into(),
            tune_count: 0,
        },
    }
}

fn sig(topic: &str, ts: i64) -> CorrectionSignal {
    CorrectionSignal {
        timestamp: ts,
        topic: topic.into(),
        severity: 5,
        note: None,
    }
}

#[test]
fn propose_tune_empty_signals_returns_empty() {
    let m = base_manifest();
    let out = propose_tune(&m, &[]);
    assert!(out.is_empty(), "empty signals → no proposals, got {out:?}");
}

#[test]
fn propose_tune_threshold_too_verbose_3() {
    let m = base_manifest();
    let signals = vec![
        sig("too_verbose", 100),
        sig("too_verbose", 101),
        sig("too_verbose", 102),
    ];
    let out = propose_tune(&m, &signals);
    assert!(
        out.contains(&ProposedChange::SetDirectness("direct".into())),
        "3× too_verbose on balanced manifest must emit SetDirectness(direct); got {out:?}"
    );
}

#[test]
fn propose_tune_below_threshold_too_verbose_2() {
    let m = base_manifest();
    let signals = vec![
        sig("too_verbose", 100),
        sig("too_verbose", 101),
    ];
    let out = propose_tune(&m, &signals);
    assert!(
        !out.contains(&ProposedChange::SetDirectness("direct".into())),
        "2× too_verbose is below threshold; got {out:?}"
    );
}

#[test]
fn propose_tune_threshold_forbidden_2() {
    let m = base_manifest();
    let signals = vec![
        sig("forbidden_topic:diagnosis", 100),
        sig("forbidden_topic:diagnosis", 101),
    ];
    let out = propose_tune(&m, &signals);
    assert!(
        out.contains(&ProposedChange::AddForbiddenTopic("diagnosis".into())),
        "2× forbidden_topic:diagnosis on clean manifest must emit AddForbiddenTopic(diagnosis); got {out:?}"
    );
}

#[test]
fn propose_tune_idempotent_directness_hard() {
    let mut m = base_manifest();
    m.edge.directness = Directness::Hard;
    let signals = vec![
        sig("too_verbose", 100),
        sig("too_verbose", 101),
        sig("too_verbose", 102),
        sig("too_verbose", 103),
    ];
    let out = propose_tune(&m, &signals);
    assert!(
        !out.iter().any(|c| matches!(c, ProposedChange::SetDirectness(_))),
        "manifest already Hard → no SetDirectness proposal; got {out:?}"
    );
}

#[test]
fn propose_tune_idempotent_forbidden_already_listed() {
    let mut m = base_manifest();
    m.forbidden.topics = vec!["diagnosis".into()];
    let signals = vec![
        sig("forbidden_topic:diagnosis", 100),
        sig("forbidden_topic:diagnosis", 101),
        sig("forbidden_topic:diagnosis", 102),
    ];
    let out = propose_tune(&m, &signals);
    assert!(
        !out.iter().any(|c| matches!(c, ProposedChange::AddForbiddenTopic(_))),
        "diagnosis already in forbidden list → no AddForbiddenTopic proposal; got {out:?}"
    );
}

#[test]
fn propose_tune_initiative_and_tone_thresholds() {
    let m = base_manifest();
    let signals = vec![
        sig("not_proactive_enough", 100),
        sig("not_proactive_enough", 101),
        sig("not_proactive_enough", 102),
        sig("too_formal", 200),
        sig("too_formal", 201),
        sig("too_formal", 202),
    ];
    let out = propose_tune(&m, &signals);
    assert!(
        out.contains(&ProposedChange::SetInitiative("proactive".into())),
        "3× not_proactive_enough must emit SetInitiative(proactive); got {out:?}"
    );
    assert!(
        out.contains(&ProposedChange::SetTonePrimary("warm".into())),
        "3× too_formal must emit SetTonePrimary(warm); got {out:?}"
    );
}
