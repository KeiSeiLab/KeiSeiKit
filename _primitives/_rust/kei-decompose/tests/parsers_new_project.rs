//! New-project adapter — phase heading extraction.

use std::path::Path;

use kei_decompose::parsers::{registry, FormatParser};

fn fixture_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/sample-new-project.md")
}

fn new_project_parser() -> Box<dyn FormatParser> {
    let r = registry();
    r.into_iter().find(|p| p.name() == "new-project").expect("new-project adapter present")
}

#[test]
fn new_project_parser_detects_phases_doc() {
    let body = std::fs::read_to_string(fixture_path()).unwrap();
    let conf = new_project_parser().detect(&body).as_f64();
    assert!(conf >= 0.7, "expected new-project confidence >= 0.7, got {}", conf);
}

#[test]
fn new_project_parser_extracts_one_action_per_phase() {
    let actions = new_project_parser().parse(&fixture_path()).unwrap();
    assert_eq!(actions.len(), 4, "expected 4 phase actions, got {}", actions.len());
    let ids: Vec<_> = actions.iter().map(|a| a.id.as_str()).collect();
    assert_eq!(ids, vec!["1", "2", "3", "4"]);
    assert!(actions[0].title.to_lowercase().contains("scaffold"));
}

#[test]
fn new_project_parser_marks_source_format() {
    let actions = new_project_parser().parse(&fixture_path()).unwrap();
    for a in &actions {
        assert_eq!(a.source_format, "new-project");
    }
}
