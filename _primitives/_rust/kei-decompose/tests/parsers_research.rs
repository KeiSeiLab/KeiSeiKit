//! Research adapter — table extraction tests.

use std::path::Path;

use kei_decompose::parsers::{registry, FormatParser};

fn fixture_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/sample-research.md")
}

fn research_parser() -> Box<dyn FormatParser> {
    let r = registry();
    r.into_iter().find(|p| p.name() == "research").expect("research adapter present")
}

#[test]
fn research_parser_detects_master_report() {
    let body = std::fs::read_to_string(fixture_path()).unwrap();
    let conf = research_parser().detect(&body).as_f64();
    assert!(conf >= 0.7, "expected research confidence >= 0.7, got {}", conf);
}

#[test]
fn research_parser_extracts_three_actions() {
    let actions = research_parser().parse(&fixture_path()).unwrap();
    assert_eq!(actions.len(), 3, "expected 3 actions, got {}", actions.len());
    assert!(actions[0].title.contains("Refactor kei-foo"));
    assert!(actions[2].title.contains("Document"));
}

#[test]
fn research_parser_attaches_severity_and_deps() {
    let actions = research_parser().parse(&fixture_path()).unwrap();
    let first = &actions[0];
    assert_eq!(first.severity.as_str(), "high");
    let third = &actions[2];
    assert_eq!(third.deps, vec!["1".to_string(), "2".to_string()]);
    assert_eq!(third.severity.as_str(), "low");
}

#[test]
fn research_parser_marks_source_format() {
    let actions = research_parser().parse(&fixture_path()).unwrap();
    for a in &actions {
        assert_eq!(a.source_format, "research");
    }
}
