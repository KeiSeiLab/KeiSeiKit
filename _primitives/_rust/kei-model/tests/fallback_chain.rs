//! Walk fallback chains: terminating, cyclic, unknown primary.

use std::path::PathBuf;

use kei_model::chain;
use kei_model::registry::Registry;

fn seed() -> Registry {
    Registry::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/models.toml")).unwrap()
}

#[test]
fn opus_chain_starts_with_opus() {
    let r = seed();
    let walk = chain("claude-opus-4-7", &r).expect("chain must walk");
    assert!(walk.len() >= 3, "opus chain should fall back at least twice");
    assert_eq!(walk[0].id, "claude-opus-4-7");
    assert_eq!(walk[1].id, "claude-sonnet-4-6");
    assert_eq!(walk[2].id, "claude-haiku-4-5");
}

#[test]
fn local_chain_terminates() {
    let r = seed();
    let walk = chain("llama-3-70b-local", &r).unwrap();
    assert_eq!(walk.len(), 1, "local model chain has empty fallback → single-entry walk");
}

#[test]
fn unknown_primary_errors() {
    let r = seed();
    let err = chain("definitely-not-a-real-model", &r);
    assert!(err.is_err(), "unknown primary must error");
    let msg = err.unwrap_err().to_string();
    assert!(msg.starts_with("unknown primary model_id"), "got: {msg}");
}

#[test]
fn cycle_detected_in_synthetic_fixture() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("models.toml");
    std::fs::write(&path, CYCLE_FIXTURE).unwrap();
    let reg = Registry::load(&path).unwrap();
    let err = chain("a", &reg).expect_err("cycle must error");
    let msg = err.to_string();
    assert!(msg.starts_with("cycle in fallback chain"), "got: {msg}");
}

const CYCLE_FIXTURE: &str = r#"
[[models]]
id = "a"
provider = "local"
display_name = "A"
context_tokens = 1000
capabilities = []
status = "active"
role_tags = []
fallback = "b"

[models.pricing]
input_per_mtok_micro = 0
output_per_mtok_micro = 0
status = "placeholder"
source_url = "https://example.test/a"

[[models]]
id = "b"
provider = "local"
display_name = "B"
context_tokens = 1000
capabilities = []
status = "active"
role_tags = []
fallback = "a"

[models.pricing]
input_per_mtok_micro = 0
output_per_mtok_micro = 0
status = "placeholder"
source_url = "https://example.test/b"
"#;
