//! Integration test for the naming_consistency scanner.
//!
//! Tmp crate contains both `D_INIT` and `DEFAULT_D` constants. cleanup
//! config declares them as a synonym pair → drift should be flagged.

use kei_cleanup::config::{Config, NamingPairsConfig};
use kei_cleanup::{scanners, FindingKind};
use std::fs;
use tempfile::TempDir;

#[test]
fn naming_consistency_flags_drift() {
    let tmp = TempDir::new().expect("tmp");
    let dir = tmp.path();
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"f\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .unwrap();
    fs::write(
        dir.join("src/lib.rs"),
        "pub const D_INIT: u32 = 1;\npub const DEFAULT_D: u32 = 2;\n",
    )
    .unwrap();

    let mut cfg = Config::default();
    cfg.naming_pairs = NamingPairsConfig {
        pairs: vec![vec!["D_INIT".into(), "DEFAULT_D".into()]],
    };
    let findings = scanners::naming_consistency::scan(dir, &cfg).expect("scan");
    assert!(
        findings
            .iter()
            .any(|f| f.kind == FindingKind::NamingDrift),
        "expected NamingDrift, got {findings:?}"
    );
}

#[test]
fn naming_consistency_skips_when_pairs_empty() {
    let tmp = TempDir::new().expect("tmp");
    let dir = tmp.path();
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"f\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .unwrap();
    fs::write(dir.join("src/lib.rs"), "pub const D_INIT: u32 = 1;\n").unwrap();

    let cfg = Config::default();
    let findings = scanners::naming_consistency::scan(dir, &cfg).expect("scan");
    assert!(findings.is_empty(), "expected clean, got {findings:?}");
}

#[test]
fn naming_consistency_only_one_variant_present_is_clean() {
    let tmp = TempDir::new().expect("tmp");
    let dir = tmp.path();
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"f\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .unwrap();
    fs::write(
        dir.join("src/lib.rs"),
        "pub const D_INIT: u32 = 1;\n",
    )
    .unwrap();

    let mut cfg = Config::default();
    cfg.naming_pairs = NamingPairsConfig {
        pairs: vec![vec!["D_INIT".into(), "DEFAULT_D".into()]],
    };
    let findings = scanners::naming_consistency::scan(dir, &cfg).expect("scan");
    assert!(findings.is_empty(), "expected clean, got {findings:?}");
}
