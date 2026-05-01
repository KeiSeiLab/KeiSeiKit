//! Integration tests for map_cmd: build_map, render_markdown, render_json.
//!
//! Uses tempdir fixtures and sibling crates as real fixtures.

use kei_import_project::map_cmd::{self, MapEntry};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

const BASE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");

/// Helper: create a minimal Rust crate in `dir/name`.
fn mk_rust_crate(parent: &Path, name: &str, src: &str) {
    let crate_dir = parent.join(name);
    let src_dir = crate_dir.join("src");
    fs::create_dir_all(&src_dir).unwrap();
    fs::write(
        crate_dir.join("Cargo.toml"),
        format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n"),
    )
    .unwrap();
    fs::write(src_dir.join("lib.rs"), src).unwrap();
}

// ─── test 1: build_map on a single known-trait crate ─────────────────────────

#[test]
fn build_map_single_crate_has_entry() {
    let tmp = TempDir::new().unwrap();
    mk_rust_crate(
        tmp.path(),
        "my-notifier",
        r#"
        pub struct Tg;
        impl NotifyChannel for Tg {
            fn send(&self) {}
            fn channel_name(&self) -> &str { "telegram" }
            fn supports_batching(&self) -> bool { false }
        }
        "#,
    );
    let entries = map_cmd::build_map(tmp.path(), 0.3).unwrap();
    assert!(!entries.is_empty(), "expected ≥1 entry");
    let e = entries.iter().find(|e| e.module == "my-notifier");
    assert!(e.is_some(), "my-notifier not found in entries");
}

// ─── test 2: non-Rust dir produces no match but still appears ────────────────

#[test]
fn build_map_non_rust_dir_included_without_match() {
    let tmp = TempDir::new().unwrap();
    mk_rust_crate(
        tmp.path(),
        "rust-crate",
        "fn send() {} fn channel_name() -> &'static str { \"x\" } fn supports_batching() -> bool { false }",
    );
    // A directory with only a package.json (NpmPackage) — no Rust source.
    let npm_dir = tmp.path().join("my-js-pkg");
    fs::create_dir_all(&npm_dir).unwrap();
    fs::write(npm_dir.join("package.json"), r#"{"name":"my-js-pkg","version":"1.0.0"}"#).unwrap();

    let entries = map_cmd::build_map(tmp.path(), 0.3).unwrap();
    // Should have both, the JS one has no best_match.
    let js = entries.iter().find(|e| e.module == "my-js-pkg");
    assert!(js.is_some(), "JS package entry expected");
    assert!(js.unwrap().best_match.is_none(), "JS package should have no match");
}

// ─── test 3: real sibling fixtures — 4+ confident matches ────────────────────

#[test]
fn build_map_sibling_crates_has_confident_matches() {
    // Run map against the full siblings dir and expect ≥4 confident matches.
    let base = Path::new(BASE);
    if !base.exists() {
        return; // not in this checkout
    }
    let entries = map_cmd::build_map(base, 0.5).unwrap();
    let confident: Vec<&MapEntry> = entries.iter().filter(|e| e.best_match.is_some()).collect();
    assert!(
        confident.len() >= 4,
        "expected ≥4 confident matches, got {}: {:?}",
        confident.len(),
        confident.iter().map(|e| &e.module).collect::<Vec<_>>()
    );
}

// ─── test 4: render_markdown produces table header ────────────────────────────

#[test]
fn render_markdown_contains_table_header() {
    let tmp = TempDir::new().unwrap();
    mk_rust_crate(tmp.path(), "test-crate", "fn foo() {}");
    let entries = map_cmd::build_map(tmp.path(), 0.3).unwrap();
    let md = map_cmd::render_markdown(&entries, 0.3, "test-repo");
    assert!(md.contains("# test-repo — architecture map"), "missing header");
    assert!(md.contains("| Module |"), "missing table header row");
    assert!(md.contains("|---|"), "missing separator row");
}

// ─── test 5: render_json round-trips via serde_json ──────────────────────────

#[test]
fn render_json_round_trips() {
    let tmp = TempDir::new().unwrap();
    mk_rust_crate(tmp.path(), "round-trip", "fn foo() {}");
    let entries = map_cmd::build_map(tmp.path(), 0.3).unwrap();
    let json = map_cmd::render_json(&entries).unwrap();
    let parsed: Vec<serde_json::Value> = serde_json::from_str(&json).unwrap();
    // Module name preserved across round-trip.
    let found = parsed.iter().any(|v| v["module"].as_str() == Some("round-trip"));
    assert!(found, "module name not found in round-tripped JSON");
}

// ─── test 6: high threshold pushes most modules to "below threshold" ──────────

#[test]
fn high_threshold_filters_most_modules() {
    let base = Path::new(BASE);
    if !base.exists() {
        return;
    }
    let entries = map_cmd::build_map(base, 0.95).unwrap();
    let below = entries.iter().filter(|e| e.best_match.is_none()).count();
    let total = entries.len();
    // At threshold 0.95, most modules should be below threshold.
    assert!(
        below > total / 2,
        "expected majority below threshold=0.95, below={below}/{total}"
    );
}
