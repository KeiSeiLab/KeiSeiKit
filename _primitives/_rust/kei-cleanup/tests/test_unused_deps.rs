//! Integration test for the unused_deps scanner.
//!
//! Builds a tmp crate with `tracing` declared in Cargo.toml but no
//! `use tracing` or `tracing::` reference in src/.

use kei_cleanup::{scanners, Config, FindingKind};
use std::fs;
use tempfile::TempDir;

const MANIFEST: &str = r#"[package]
name = "fixture"
version = "0.1.0"
edition = "2021"

[dependencies]
tracing = "0.1"
serde = "1"
"#;

const SRC_USES_SERDE: &str = r#"
use serde::Serialize;

#[derive(Serialize)]
pub struct A { pub x: u32 }
"#;

#[test]
fn unused_deps_flags_tracing_only() {
    let tmp = TempDir::new().expect("tmp");
    let dir = tmp.path().join("fixture");
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::write(dir.join("Cargo.toml"), MANIFEST).unwrap();
    fs::write(dir.join("src/lib.rs"), SRC_USES_SERDE).unwrap();

    let cfg = Config::default();
    let findings = scanners::unused_deps::scan(tmp.path(), &cfg).expect("scan");

    let unused: Vec<_> = findings
        .iter()
        .filter(|f| f.kind == FindingKind::UnusedDep)
        .collect();
    assert!(
        unused.iter().any(|f| f.message.contains("tracing")),
        "expected `tracing` flagged unused; got {findings:?}"
    );
    assert!(
        !unused.iter().any(|f| f.message.contains("serde")),
        "did NOT expect `serde` flagged; got {findings:?}"
    );
}
