//! Cross-parser normalization — every parser yields the same Action shape.
//!
//! This catches drift: if a new adapter starts emitting half-filled Actions
//! or skips the source_format tag, downstream emit/dispatch breaks silently.

use std::path::Path;

use kei_decompose::normalizer::Action;
use kei_decompose::parsers::{registry, FormatParser};

fn fixture(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn parser(name: &str) -> Box<dyn FormatParser> {
    registry().into_iter().find(|p| p.name() == name).expect("parser present")
}

fn assert_action_shape(a: &Action, expected_format: &str) {
    assert!(!a.id.is_empty(), "{}: id empty", expected_format);
    assert!(!a.title.is_empty(), "{}: title empty", expected_format);
    assert_eq!(a.source_format, expected_format, "source_format mismatch");
    assert!(!a.source_path.is_empty(), "{}: source_path empty", expected_format);
    assert!(a.source_line > 0, "{}: source_line zero", expected_format);
    // Body should reference the source path (defensive marker for downstream).
    assert!(a.body.contains(&a.source_path), "{}: body missing source ref", expected_format);
}

#[test]
fn all_parsers_emit_uniform_action_shape() {
    let cases = [
        ("research", "sample-research.md"),
        ("audit", "sample-audit.md"),
        ("sleep", "sample-sleep.md"),
        ("architecture", "sample-arch.md"),
        ("new-project", "sample-new-project.md"),
    ];
    for (fmt, file) in cases {
        let actions = parser(fmt).parse(&fixture(file)).unwrap();
        assert!(!actions.is_empty(), "{}: yielded zero actions", fmt);
        for a in &actions {
            assert_action_shape(a, fmt);
        }
    }
}

#[test]
fn action_serializes_to_json_round_trip() {
    let actions = parser("research").parse(&fixture("sample-research.md")).unwrap();
    let json = serde_json::to_string(&actions).unwrap();
    let back: Vec<Action> = serde_json::from_str(&json).unwrap();
    assert_eq!(actions.len(), back.len());
    assert_eq!(actions[0].id, back[0].id);
}
