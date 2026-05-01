//! Substituter smoke test.
//!
//! Uses a synthetic in-test matrix; never echoes SSoT patterns.

use kei_leak_matrix::{substitute, Matrix, Scope};
use std::io::Write;
use tempfile::NamedTempFile;

fn build_test_matrix() -> (NamedTempFile, Matrix) {
    let toml = r#"
[[rule]]
id = "test-sub-username"
pattern = "TESTONLY_USER_NAME"
substitute_with = "USER"
category = "personal"
severity = "substitute"
scope = ["public-mirror"]
rationale = "test-only personal substitute"
added = "2026-04-26"

[[rule]]
id = "test-block-not-substituted"
pattern = "TESTONLY_BLOCK_KEY"
category = "secret"
severity = "block"
scope = ["public-mirror"]
rationale = "test-only block — must NOT be removed by substitute"
added = "2026-04-26"
"#;
    let mut f = NamedTempFile::new().unwrap();
    f.write_all(toml.as_bytes()).unwrap();
    f.flush().unwrap();
    let m = Matrix::load(f.path()).unwrap();
    (f, m)
}

#[test]
fn substitute_replaces_personal_token_with_redacted_value() {
    let (_f, m) = build_test_matrix();
    let input = "before TESTONLY_USER_NAME after";
    let out = substitute(&m, input, Scope::PublicMirror);
    assert_eq!(out, "before USER after");
}

#[test]
fn substitute_skips_block_severity_rules() {
    let (_f, m) = build_test_matrix();
    let input = "alpha TESTONLY_BLOCK_KEY omega";
    let out = substitute(&m, input, Scope::PublicMirror);
    // Block rules are NOT applied by substitute — block check happens later.
    assert_eq!(out, input);
}

#[test]
fn substitute_respects_scope_filter() {
    let (_f, m) = build_test_matrix();
    let input = "carry TESTONLY_USER_NAME on";
    // CommitMsg scope must NOT trigger the public-mirror-scoped substitute.
    let out = substitute(&m, input, Scope::CommitMsg);
    assert_eq!(out, input);
}

#[test]
fn substitute_is_idempotent_on_clean_input() {
    let (_f, m) = build_test_matrix();
    let input = "no triggers here at all";
    let out = substitute(&m, input, Scope::PublicMirror);
    assert_eq!(out, input);
}
