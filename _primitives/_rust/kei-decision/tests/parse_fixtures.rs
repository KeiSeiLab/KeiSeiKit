//! Integration tests for the markdown action-table parser.

use kei_decision::{parse_master_report, ParseError};
use std::path::PathBuf;

fn fixture(name: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("tests/fixtures");
    p.push(name);
    p
}

#[test]
fn valid_master_extracts_five_actions() {
    let actions = parse_master_report(&fixture("valid-master.md")).expect("parse ok");
    assert_eq!(actions.len(), 5, "expected 5 rows in fixture, got {:?}", actions);
    assert_eq!(actions[0].id, "1");
    assert!(actions[0].title.contains("Refactor 4 hooks"));
    assert_eq!(actions[0].severity, "LOW");
    assert_eq!(actions[0].effort, "1-2d");
}

#[test]
fn deps_hint_parsed_when_present() {
    let actions = parse_master_report(&fixture("valid-master.md")).expect("parse ok");
    let migrate = actions.iter().find(|a| a.id == "3").expect("row 3 present");
    assert_eq!(migrate.deps, vec!["1".to_string()]);
    let new_prim = actions.iter().find(|a| a.id == "4").expect("row 4 present");
    assert_eq!(new_prim.deps, vec!["3".to_string()]);
}

#[test]
fn no_actions_returns_no_actions_found() {
    let err = parse_master_report(&fixture("no-actions.md")).expect_err("should fail");
    assert!(matches!(err, ParseError::NoActionsFound), "got {:?}", err);
}

#[test]
fn malformed_table_skips_gracefully() {
    // The malformed fixture has a 1-column table — parser should accept rows
    // (since "Action" header present), defaulting severity/effort to empty.
    let actions = parse_master_report(&fixture("malformed-table.md")).expect("parse ok");
    assert_eq!(actions.len(), 2);
    assert!(actions[0].severity.is_empty());
    assert!(actions[0].effort.is_empty());
}

#[test]
fn missing_file_returns_file_not_found() {
    let err = parse_master_report(&fixture("DOES-NOT-EXIST.md")).expect_err("should fail");
    assert!(matches!(err, ParseError::FileNotFound(_)), "got {:?}", err);
}
