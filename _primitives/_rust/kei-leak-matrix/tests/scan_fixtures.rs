//! Scanner integration tests.
//!
//! Tests use a synthetic in-test matrix written to a temp file. We do NOT
//! echo any real SSoT pattern. Test fixtures use clearly-test-only tokens
//! (TESTONLY_* prefixes) that the SSoT matrix does not list.
//!
//! Where we cross-check the real SSoT matrix, we reference rules by `id`
//! only — never by pattern source.

use kei_leak_matrix::{scan_file, scan_string, Matrix, Scope, Severity};
use std::io::Write;
use tempfile::NamedTempFile;

/// Build a tiny test matrix on disk and return the loaded Matrix.
/// Patterns below are TEST-ONLY tokens, intentionally distinct from any
/// SSoT pattern, so this fixture cannot accidentally leak IP-bearing regex.
fn build_test_matrix() -> (NamedTempFile, Matrix) {
    let toml = r#"
[[rule]]
id = "test-block-alpha"
pattern = "TESTONLY_BLOCK_ALPHA"
category = "secret"
severity = "block"
scope = ["all-writes", "public-mirror"]
rationale = "test-only block rule"
added = "2026-04-26"

[[rule]]
id = "test-warn-beta"
pattern = "TESTONLY_WARN_BETA"
category = "secret"
severity = "warn"
scope = ["public-mirror"]
rationale = "test-only warn rule"
added = "2026-04-26"

[[rule]]
id = "test-sub-gamma"
pattern = "TESTONLY_SUB_GAMMA"
substitute_with = "REDACTED"
category = "personal"
severity = "substitute"
scope = ["public-mirror"]
rationale = "test-only substitute rule"
added = "2026-04-26"

[[rule]]
id = "test-scoped-only-commit"
pattern = "TESTONLY_COMMIT_ONLY"
category = "secret"
severity = "block"
scope = ["commit-msg"]
rationale = "test-only commit-msg rule"
added = "2026-04-26"
"#;
    let mut f = NamedTempFile::new().expect("tmp file");
    f.write_all(toml.as_bytes()).expect("write");
    f.flush().expect("flush");
    let m = Matrix::load(f.path()).expect("matrix loads");
    (f, m)
}

#[test]
fn loads_and_compiles_all_test_rules() {
    let (_f, m) = build_test_matrix();
    assert_eq!(m.rules.len(), 4);
    let ids: Vec<&str> = m.rules.iter().map(|r| r.id.as_str()).collect();
    assert!(ids.contains(&"test-block-alpha"));
    assert!(ids.contains(&"test-warn-beta"));
    assert!(ids.contains(&"test-sub-gamma"));
    assert!(ids.contains(&"test-scoped-only-commit"));
}

#[test]
fn scan_string_finds_block_violation_by_id() {
    let (_f, m) = build_test_matrix();
    // Synthetic content that triggers the block rule.
    let content = "line one\nfoo TESTONLY_BLOCK_ALPHA bar\nline three\n";
    let v = scan_string(&m, content, Scope::AllWrites, None, "syn");
    assert_eq!(v.len(), 1, "exactly one violation");
    assert_eq!(v[0].rule_id, "test-block-alpha");
    assert_eq!(v[0].line, 2);
    assert_eq!(v[0].severity, "block");
}

#[test]
fn scan_string_redacts_match_to_12_chars_plus_ellipsis() {
    let (_f, m) = build_test_matrix();
    let content = "TESTONLY_BLOCK_ALPHA";
    let v = scan_string(&m, content, Scope::AllWrites, None, "syn");
    assert_eq!(v.len(), 1);
    // Exactly 12 chars + ellipsis when source longer than 12.
    let chars: Vec<char> = v[0].matched_redacted.chars().collect();
    assert_eq!(chars.len(), 13, "12 + ellipsis");
    assert_eq!(chars.last(), Some(&'…'));
}

#[test]
fn scope_filter_excludes_non_matching_scope() {
    let (_f, m) = build_test_matrix();
    // test-scoped-only-commit only applies to commit-msg.
    let content = "x TESTONLY_COMMIT_ONLY y";
    let v = scan_string(&m, content, Scope::PublicMirror, None, "syn");
    assert_eq!(v.len(), 0, "commit-msg-only rule must not fire on public-mirror");
    let v2 = scan_string(&m, content, Scope::CommitMsg, None, "syn");
    assert_eq!(v2.len(), 1);
    assert_eq!(v2[0].rule_id, "test-scoped-only-commit");
}

#[test]
fn all_writes_scope_matches_every_request() {
    let (_f, m) = build_test_matrix();
    // test-block-alpha has scope all-writes — matches both PublicMirror and AllWrites.
    let content = "x TESTONLY_BLOCK_ALPHA y";
    let v_pm = scan_string(&m, content, Scope::PublicMirror, None, "syn");
    let v_aw = scan_string(&m, content, Scope::AllWrites, None, "syn");
    assert_eq!(v_pm.len(), 1);
    assert_eq!(v_aw.len(), 1);
}

#[test]
fn severity_filter_narrows_to_block_only() {
    let (_f, m) = build_test_matrix();
    let content = "TESTONLY_BLOCK_ALPHA TESTONLY_WARN_BETA TESTONLY_SUB_GAMMA";
    let v_block = scan_string(&m, content, Scope::PublicMirror, Some(Severity::Block), "syn");
    assert_eq!(v_block.len(), 1);
    assert_eq!(v_block[0].rule_id, "test-block-alpha");
}

#[test]
fn severity_filter_narrows_to_warn_only() {
    let (_f, m) = build_test_matrix();
    let content = "TESTONLY_BLOCK_ALPHA TESTONLY_WARN_BETA TESTONLY_SUB_GAMMA";
    let v_warn = scan_string(&m, content, Scope::PublicMirror, Some(Severity::Warn), "syn");
    assert_eq!(v_warn.len(), 1);
    assert_eq!(v_warn[0].rule_id, "test-warn-beta");
}

#[test]
fn scan_file_reads_path_and_finds_violations() {
    let (_f, m) = build_test_matrix();
    let mut tf = NamedTempFile::new().unwrap();
    writeln!(tf, "first line").unwrap();
    writeln!(tf, "TESTONLY_BLOCK_ALPHA on second").unwrap();
    tf.flush().unwrap();
    let v = scan_file(&m, tf.path(), Scope::AllWrites, None).expect("scan ok");
    assert_eq!(v.len(), 1);
    assert_eq!(v[0].rule_id, "test-block-alpha");
    assert_eq!(v[0].line, 2);
}

#[test]
fn ssot_matrix_has_eleven_secret_rules() {
    // Cross-check the real SSoT matrix: count secret-category rules.
    // We never echo any pattern; we count by category alone.
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../security/leak-matrix.toml");
    if !path.exists() {
        // Skip if the SSoT file isn't reachable from test harness path.
        eprintln!("ssot path not present, skipping: {}", path.display());
        return;
    }
    let m = Matrix::load(&path).expect("ssot matrix loads");
    let secret_count = m.rules.iter()
        .filter(|r| r.category == kei_leak_matrix::Category::Secret).count();
    assert_eq!(secret_count, 11, "SSoT must contain 11 secret rules");
}
