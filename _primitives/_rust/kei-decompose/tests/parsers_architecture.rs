//! Architecture adapter — numbered recommendation extraction.

use std::path::Path;

use kei_decompose::parsers::{registry, FormatParser};

fn fixture_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/sample-arch.md")
}

fn architecture_parser() -> Box<dyn FormatParser> {
    let r = registry();
    r.into_iter().find(|p| p.name() == "architecture").expect("architecture adapter present")
}

#[test]
fn architecture_parser_detects_decision_record() {
    let body = std::fs::read_to_string(fixture_path()).unwrap();
    let conf = architecture_parser().detect(&body).as_f64();
    assert!(conf >= 0.7, "expected architecture confidence >= 0.7, got {}", conf);
}

#[test]
fn architecture_parser_extracts_three_recommendations() {
    let actions = architecture_parser().parse(&fixture_path()).unwrap();
    assert_eq!(actions.len(), 3, "expected 3 recommendations, got {}", actions.len());
    assert_eq!(actions[0].id, "1");
    assert!(actions[0].title.contains("FormatParser trait"));
}

#[test]
fn architecture_parser_marks_source_format() {
    let actions = architecture_parser().parse(&fixture_path()).unwrap();
    for a in &actions {
        assert_eq!(a.source_format, "architecture");
    }
}
