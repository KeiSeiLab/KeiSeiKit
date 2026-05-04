//! Integration test for the coverage_map scanner.
//!
//! Tmp workspace has one `// derive: foo` marker and one `// covers: bar`
//! marker — so `foo` is missing a test and `bar` is a stale test.

use kei_cleanup::{scanners, Config, FindingKind};
use std::fs;
use tempfile::TempDir;

#[test]
fn coverage_map_flags_missing_test_and_stale_test() {
    let tmp = TempDir::new().expect("tmp");
    let dir = tmp.path();
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::create_dir_all(dir.join("tests")).unwrap();
    fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"f\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .unwrap();
    fs::write(dir.join("src/lib.rs"), "// derive: foo\npub fn z() {}\n").unwrap();
    fs::write(dir.join("tests/t.rs"), "// covers: bar\n#[test]\nfn t() {}\n").unwrap();

    let cfg = Config::default();
    let findings = scanners::coverage_map::scan(dir, &cfg).expect("scan");
    let gaps: Vec<_> = findings
        .iter()
        .filter(|f| f.kind == FindingKind::CoverageGap)
        .collect();
    assert!(
        gaps.iter().any(|f| f.message.contains("foo")),
        "expected derive `foo` missing test, got {findings:?}"
    );
    assert!(
        gaps.iter().any(|f| f.message.contains("bar")),
        "expected stale test `bar`, got {findings:?}"
    );
}

#[test]
fn coverage_map_clean_workspace_has_no_findings() {
    let tmp = TempDir::new().expect("tmp");
    let dir = tmp.path();
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::create_dir_all(dir.join("tests")).unwrap();
    fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"f\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .unwrap();
    fs::write(dir.join("src/lib.rs"), "// derive: alpha\npub fn z() {}\n").unwrap();
    fs::write(dir.join("tests/t.rs"), "// covers: alpha\n#[test]\nfn t() {}\n").unwrap();

    let cfg = Config::default();
    let findings = scanners::coverage_map::scan(dir, &cfg).expect("scan");
    assert!(
        findings.is_empty(),
        "expected zero findings, got {findings:?}"
    );
}
