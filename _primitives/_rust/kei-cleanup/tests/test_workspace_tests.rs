//! Integration test for the workspace_tests scanner.
//!
//! Tmp crate with a `tests/x.rs` that imports two `kei_*` siblings.

use kei_cleanup::{scanners, Config, FindingKind};
use std::fs;
use tempfile::TempDir;

const CROSS_TEST: &str = r#"
use kei_alpha::Foo;
use kei_beta::Bar;

#[test]
fn t() {
    let _f: Option<Foo> = None;
    let _b: Option<Bar> = None;
}
"#;

#[test]
fn workspace_tests_flags_cross_crate_test() {
    let tmp = TempDir::new().expect("tmp");
    let dir = tmp.path();
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::create_dir_all(dir.join("tests")).unwrap();
    fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"f\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .unwrap();
    fs::write(dir.join("src/lib.rs"), "pub fn z() {}\n").unwrap();
    fs::write(dir.join("tests/x.rs"), CROSS_TEST).unwrap();

    let cfg = Config::default();
    let findings = scanners::workspace_tests::scan(dir, &cfg).expect("scan");
    assert!(
        findings
            .iter()
            .any(|f| f.kind == FindingKind::WorkspaceTestNeeded),
        "expected ≥1 WorkspaceTestNeeded, got {findings:?}"
    );
}

#[test]
fn workspace_tests_single_sibling_is_clean() {
    let tmp = TempDir::new().expect("tmp");
    let dir = tmp.path();
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::create_dir_all(dir.join("tests")).unwrap();
    fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"f\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .unwrap();
    fs::write(dir.join("src/lib.rs"), "pub fn z() {}\n").unwrap();
    fs::write(
        dir.join("tests/x.rs"),
        "use kei_alpha::Foo;\n#[test] fn t() { let _: Option<Foo> = None; }\n",
    )
    .unwrap();

    let cfg = Config::default();
    let findings = scanners::workspace_tests::scan(dir, &cfg).expect("scan");
    assert!(findings.is_empty(), "expected clean, got {findings:?}");
}
