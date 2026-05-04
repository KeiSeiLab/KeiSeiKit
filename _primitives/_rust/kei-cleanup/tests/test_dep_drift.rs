//! Integration test for the dep_drift scanner.
//!
//! Builds a fixture workspace whose member crate pins `serde = "1.0.180"`
//! while `[workspace.dependencies]` declares `serde = "1.0.190"`.

use kei_cleanup::{scanners, Config, FindingKind};
use std::fs;
use tempfile::TempDir;

const WORKSPACE_TOML: &str = r#"[workspace]
resolver = "2"
members = ["mismatch"]

[workspace.dependencies]
serde = "1.0.190"
"#;

const MEMBER_TOML: &str = r#"[package]
name = "mismatch"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0.180"
"#;

#[test]
fn dep_drift_flags_version_mismatch() {
    let tmp = TempDir::new().expect("tmp");
    fs::write(tmp.path().join("Cargo.toml"), WORKSPACE_TOML).unwrap();
    let member = tmp.path().join("mismatch");
    fs::create_dir_all(member.join("src")).unwrap();
    fs::write(member.join("Cargo.toml"), MEMBER_TOML).unwrap();
    fs::write(member.join("src/lib.rs"), "pub fn x() {}\n").unwrap();

    let cfg = Config::default();
    let findings = scanners::dep_drift::scan(tmp.path(), &cfg).expect("scan");
    let drifts: Vec<_> = findings
        .iter()
        .filter(|f| f.kind == FindingKind::DepDrift)
        .collect();
    assert_eq!(
        drifts.len(),
        1,
        "expected exactly 1 DepDrift finding, got {drifts:?}"
    );
    assert!(drifts[0].message.contains("serde"));
    assert!(drifts[0].message.contains("1.0.180"));
    assert!(drifts[0].message.contains("1.0.190"));
}

#[test]
fn dep_drift_ignores_workspace_inherit() {
    let tmp = TempDir::new().expect("tmp");
    fs::write(tmp.path().join("Cargo.toml"), WORKSPACE_TOML).unwrap();
    let member = tmp.path().join("inheritor");
    fs::create_dir_all(member.join("src")).unwrap();
    let inheritor_toml = r#"[package]
name = "inheritor"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { workspace = true }
"#;
    fs::write(member.join("Cargo.toml"), inheritor_toml).unwrap();
    fs::write(member.join("src/lib.rs"), "pub fn y() {}\n").unwrap();

    let cfg = Config::default();
    let findings = scanners::dep_drift::scan(tmp.path(), &cfg).expect("scan");
    let drifts: Vec<_> = findings
        .iter()
        .filter(|f| f.kind == FindingKind::DepDrift)
        .collect();
    assert!(
        drifts.is_empty(),
        "did NOT expect drift findings for workspace=true; got {drifts:?}"
    );
}
