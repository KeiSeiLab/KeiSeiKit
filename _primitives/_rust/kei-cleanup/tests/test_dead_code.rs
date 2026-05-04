//! Integration test for the dead_code scanner shell-out.
//!
//! Skips itself silently when neither `cargo-machete` nor
//! `cargo-udeps` is available on PATH (per CLEANUP-RUNTIME-SPEC.md
//! §8.3 graceful fallback). When `cargo-machete` IS available, runs
//! it on a fixture and asserts the scanner returns Ok or
//! ToolNotFound — never panics.

use kei_cleanup::{scanners, Config};
use std::fs;
use std::process::Command;
use tempfile::TempDir;

fn machete_available() -> bool {
    Command::new("cargo")
        .args(["machete", "--version"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn make_fixture() -> TempDir {
    let tmp = TempDir::new().expect("tmp");
    let manifest = "[package]\nname = \"deadfx\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n";
    fs::write(tmp.path().join("Cargo.toml"), manifest).unwrap();
    fs::create_dir_all(tmp.path().join("src")).unwrap();
    fs::write(tmp.path().join("src/lib.rs"), "pub fn x() {}\n").unwrap();
    tmp
}

#[test]
fn dead_code_either_runs_or_skips_cleanly() {
    if !machete_available() {
        eprintln!("[skip] cargo-machete not on PATH — graceful fallback covered");
        return;
    }
    let tmp = make_fixture();
    let cfg = Config::default();
    // Outcome matrix per CLEANUP-RUNTIME-SPEC.md §3.7: Ok / ToolNotFound /
    // ToolFailure are all acceptable; no panic.
    match scanners::dead_code::scan(tmp.path(), &cfg) {
        Ok(_) => {}
        Err(kei_cleanup::CleanupError::ToolNotFound(_)) => {}
        Err(kei_cleanup::CleanupError::ToolFailure { .. }) => {}
        Err(other) => panic!("unexpected error variant: {other:?}"),
    }
}
