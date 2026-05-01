//! Sleep adapter — Patterns + Backlog checklist extraction.

use std::path::Path;

use kei_decompose::parsers::{registry, FormatParser};

fn fixture_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/sample-sleep.md")
}

fn sleep_parser() -> Box<dyn FormatParser> {
    let r = registry();
    r.into_iter().find(|p| p.name() == "sleep").expect("sleep adapter present")
}

#[test]
fn sleep_parser_detects_rem_report() {
    let body = std::fs::read_to_string(fixture_path()).unwrap();
    let conf = sleep_parser().detect(&body).as_f64();
    assert!(conf >= 0.7, "expected sleep confidence >= 0.7, got {}", conf);
}

#[test]
fn sleep_parser_extracts_checklist_and_patterns() {
    let actions = sleep_parser().parse(&fixture_path()).unwrap();
    assert!(actions.len() >= 6, "expected ≥6 actions (3 checklist + 3 patterns), got {}", actions.len());
    let checklist: Vec<_> = actions.iter().filter(|a| a.id.starts_with('c')).collect();
    let patterns: Vec<_> = actions.iter().filter(|a| a.id.starts_with('p')).collect();
    assert_eq!(checklist.len(), 3, "expected 3 checklist items");
    assert_eq!(patterns.len(), 3, "expected 3 pattern rows");
}

#[test]
fn sleep_parser_marks_source_format() {
    let actions = sleep_parser().parse(&fixture_path()).unwrap();
    for a in &actions {
        assert_eq!(a.source_format, "sleep");
    }
}
