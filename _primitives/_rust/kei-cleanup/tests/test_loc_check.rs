//! Integration test for the loc_check scanner.
//!
//! Builds a temporary workspace with one oversized file and verifies
//! the scanner emits a `LocFile` finding plus an over-30-LOC `LocFunction`
//! finding for the embedded function.

use kei_cleanup::{scanners, Config, FindingKind};
use std::fs;
use tempfile::TempDir;

fn make_oversized_rs() -> String {
    let mut s = String::new();
    s.push_str("// covers: cleanup-loc-check\n");
    s.push_str("pub fn big_one() {\n");
    for i in 0..50 {
        s.push_str(&format!("    let _x{i} = {i};\n"));
    }
    s.push_str("}\n");
    // Pad to push the file over 200 LOC.
    for _ in 0..160 {
        s.push_str("// filler line\n");
    }
    s
}

#[test]
fn loc_check_flags_oversized_file_and_function() {
    let tmp = TempDir::new().expect("tmp");
    let crate_dir = tmp.path().join("dummy");
    fs::create_dir_all(crate_dir.join("src")).unwrap();
    fs::write(
        crate_dir.join("Cargo.toml"),
        "[package]\nname = \"dummy\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .unwrap();
    fs::write(crate_dir.join("src/lib.rs"), make_oversized_rs()).unwrap();

    let cfg = Config::default();
    let findings = scanners::loc_check::scan(tmp.path(), &cfg).expect("scan");
    let file_kind = findings.iter().filter(|f| f.kind == FindingKind::LocFile).count();
    let fn_kind = findings.iter().filter(|f| f.kind == FindingKind::LocFunction).count();
    assert!(file_kind >= 1, "expected ≥1 LocFile finding, got {file_kind}");
    assert!(fn_kind >= 1, "expected ≥1 LocFunction finding, got {fn_kind}");
}

#[test]
fn loc_check_clean_file_emits_nothing() {
    let tmp = TempDir::new().expect("tmp");
    let crate_dir = tmp.path().join("clean");
    fs::create_dir_all(crate_dir.join("src")).unwrap();
    fs::write(
        crate_dir.join("Cargo.toml"),
        "[package]\nname = \"clean\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .unwrap();
    fs::write(
        crate_dir.join("src/lib.rs"),
        "pub fn small() -> u32 { 7 }\n",
    )
    .unwrap();

    let cfg = Config::default();
    let findings = scanners::loc_check::scan(tmp.path(), &cfg).expect("scan");
    assert!(
        findings.is_empty(),
        "expected zero findings, got {findings:?}"
    );
}
