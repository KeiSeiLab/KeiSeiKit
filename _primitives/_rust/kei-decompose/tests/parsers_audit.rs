//! Audit adapter — Priority Matrix extraction tests.

use std::path::Path;

use kei_decompose::parsers::{registry, FormatParser};

fn fixture_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/sample-audit.md")
}

fn audit_parser() -> Box<dyn FormatParser> {
    let r = registry();
    r.into_iter().find(|p| p.name() == "audit").expect("audit adapter present")
}

#[test]
fn audit_parser_detects_wave_report() {
    let body = std::fs::read_to_string(fixture_path()).unwrap();
    let conf = audit_parser().detect(&body).as_f64();
    assert!(conf >= 0.7, "expected audit confidence >= 0.7, got {}", conf);
}

#[test]
fn audit_parser_extracts_three_findings() {
    let actions = audit_parser().parse(&fixture_path()).unwrap();
    assert_eq!(actions.len(), 3, "expected 3 findings, got {}", actions.len());
    assert!(actions[0].title.contains("research → action gap"));
}

#[test]
fn audit_parser_classifies_severities() {
    let actions = audit_parser().parse(&fixture_path()).unwrap();
    let s: Vec<_> = actions.iter().map(|a| a.severity.as_str()).collect();
    assert_eq!(s, vec!["high", "medium", "low"]);
}

#[test]
fn audit_parser_marks_source_format() {
    let actions = audit_parser().parse(&fixture_path()).unwrap();
    for a in &actions {
        assert_eq!(a.source_format, "audit");
    }
}
