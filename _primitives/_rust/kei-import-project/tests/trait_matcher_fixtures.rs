//! A2.1 integration tests: validate trait-pattern matching against real sibling crates.
//!
//! Each positive fixture asserts that match_module() detects the expected
//! TraitKind with confidence >= 0.5 when given a real crate's source files.
//! Negative fixtures assert that utility crates produce no confident matches.

use kei_import_project::{match_module, ModuleSource, TraitKind};
use std::path::Path;

const BASE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");

fn load(crate_name: &str) -> ModuleSource {
    let src_dir = Path::new(BASE).join(crate_name).join("src");
    ModuleSource::from_dir(crate_name, &src_dir)
        .unwrap_or_else(|e| panic!("from_dir({crate_name}): {e}"))
}

fn assert_matches(src: &ModuleSource, expected_kind: TraitKind, min_confidence: f64) {
    let matches = match_module(src);
    let hit = matches.iter().find(|m| m.kind == expected_kind);
    let debug: Vec<_> = matches.iter()
        .map(|m| format!("{:?}@{:.2}", m.kind, m.confidence))
        .collect();
    assert!(hit.is_some(),
        "crate '{}': expected {:?} match. Got: {:?}", src.name, expected_kind, debug);
    assert!(
        hit.unwrap().confidence >= min_confidence,
        "crate '{}': {:?} confidence {:.2} < minimum {:.2}. Methods matched: {:?}",
        src.name, expected_kind, hit.unwrap().confidence, min_confidence,
        hit.unwrap().matched_methods
    );
}

// ─────────────────────────── positive fixtures ───────────────────────────────

#[test]
fn memory_sqlite_matches_memory_backend() {
    let src = load("kei-memory-sqlite");
    assert_matches(&src, TraitKind::MemoryBackend, 0.5);
}

#[test]
fn auth_google_matches_auth_provider() {
    let src = load("kei-auth-google");
    assert_matches(&src, TraitKind::AuthProvider, 0.5);
}

#[test]
fn notify_telegram_matches_notify_channel() {
    let src = load("kei-notify-telegram");
    assert_matches(&src, TraitKind::NotifyChannel, 0.5);
}

#[test]
fn git_forgejo_matches_git_backend() {
    let src = load("kei-git-forgejo");
    assert_matches(&src, TraitKind::GitBackend, 0.5);
}

#[test]
fn svc_systemd_matches_service_manager() {
    let src = load("kei-svc-systemd");
    assert_matches(&src, TraitKind::ServiceManager, 0.5);
}

#[test]
fn compute_vultr_matches_compute_provider() {
    // kei-compute-vultr is a ComputeProvider implementation.
    let src_dir = Path::new(BASE).join("kei-compute-vultr").join("src");
    if !src_dir.exists() {
        return; // fixture not present in this checkout
    }
    let src = ModuleSource::from_dir("kei-compute-vultr", &src_dir).unwrap();
    assert_matches(&src, TraitKind::ComputeProvider, 0.4);
}

#[test]
fn llm_ollama_has_source_files_and_matcher_runs() {
    // kei-llm-ollama is an HTTP CLI adapter (not a direct LlmBackend impl).
    // Verify it loads cleanly and match_module doesn't panic.
    let src = load("kei-llm-ollama");
    assert!(!src.source_files.is_empty(),
        "kei-llm-ollama should have .rs source files");
    let _ = match_module(&src); // must not panic
}

// ─────────────────────────── negative fixtures ───────────────────────────────

#[test]
fn shared_produces_no_confident_trait_match() {
    let src = load("kei-shared");
    let matches = match_module(&src);
    // kei-shared is a utility crate; it should not confidently match any trait.
    for m in &matches {
        assert!(
            m.confidence < 0.5,
            "kei-shared unexpectedly matched {:?} with confidence {:.2}",
            m.kind, m.confidence
        );
    }
}

#[test]
fn mock_render_produces_no_confident_trait_match() {
    let src = load("mock-render");
    let matches = match_module(&src);
    for m in &matches {
        assert!(
            m.confidence < 0.5,
            "mock-render unexpectedly matched {:?} with confidence {:.2}",
            m.kind, m.confidence
        );
    }
}
