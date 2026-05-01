//! Detection smoke — feed each fixture to detect_format(), assert correct
//! winner with confidence ≥ 0.7.

use std::path::Path;

use kei_decompose::parsers::detect_format;

fn fixture(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn check(file: &str, expected: &str) {
    let body = std::fs::read_to_string(fixture(file)).unwrap();
    let r = detect_format(&body);
    assert!(r.confidence >= 0.7, "{}: confidence {}", file, r.confidence);
    let winner = r.winner.as_deref().unwrap_or("<none>");
    assert_eq!(winner, expected, "{}: detected {}, expected {}", file, winner, expected);
}

#[test]
fn detects_research_master() {
    check("sample-research.md", "research");
}

#[test]
fn detects_audit_report() {
    check("sample-audit.md", "audit");
}

#[test]
fn detects_sleep_rem_report() {
    check("sample-sleep.md", "sleep");
}

#[test]
fn detects_architecture_decision() {
    check("sample-arch.md", "architecture");
}

#[test]
fn detects_new_project_phases() {
    check("sample-new-project.md", "new-project");
}

#[test]
fn unknown_text_is_unclaimed() {
    let body = "# Unrelated heading\n\nSome plain text without any kit cues.";
    let r = detect_format(body);
    assert!(r.confidence < 0.5, "expected NONE/AMBIGUOUS, got {}", r.confidence);
}
