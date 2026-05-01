//! Integration tests for `kei_pet::evolution` — diff detection + fork
//! chain linking. Uses `examples/minimal.toml` as the baseline and mutates
//! clones to exercise each `Change` variant.

use kei_pet::evolution::{diff, fork_version, Change, PersonaVersion};
use kei_pet::parse;
use kei_pet::schema::Tone;

const MINIMAL: &str = include_str!("../examples/minimal.toml");

fn base() -> kei_pet::PetManifest {
    parse(MINIMAL).expect("minimal.toml must validate")
}

#[test]
fn diff_detects_tone_change() {
    let old = base();
    let mut new = old.clone();
    new.voice.tone_primary = Tone::Warm;

    let changes = diff(&old, &new);
    assert_eq!(changes.len(), 1, "expected exactly one change, got {changes:?}");
    assert_eq!(
        changes[0],
        Change::VoiceTonePrimaryChanged {
            from: "neutral".to_string(),
            to: "warm".to_string(),
        }
    );
}

#[test]
fn diff_detects_forbidden_added() {
    let old = base();
    let mut new = old.clone();
    new.forbidden.topics.push("diagnosis".to_string());

    let changes = diff(&old, &new);
    assert_eq!(changes.len(), 1, "expected exactly one change, got {changes:?}");
    assert_eq!(
        changes[0],
        Change::ForbiddenTopicAdded("diagnosis".to_string())
    );
}

#[test]
fn fork_version_increments_and_links() {
    let manifest = base();
    let v1 = PersonaVersion {
        version: 1,
        parent_version: None,
        manifest: manifest.clone(),
        created_at: 1_700_000_000,
        reason: "initial".to_string(),
    };

    let v2 = fork_version(&v1, "tune tone", manifest);
    assert_eq!(v2.version, 2);
    assert_eq!(v2.parent_version, Some(1));
    assert_eq!(v2.reason, "tune tone");
}
