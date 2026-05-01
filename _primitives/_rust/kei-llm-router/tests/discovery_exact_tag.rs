//! Test 4 — Ollama with exact tag `llama3:70b` → exact match.
//!
//! Pure unit test against `pick_match` — no daemon contact.

use kei_llm_router::{pick_match, ModelMatch};

#[test]
fn exact_tag_matches_exact() {
    let names = vec![
        "llama3:70b".to_string(),
        "qwen3:4b".to_string(),
        "mistral:7b".to_string(),
    ];
    let m = pick_match("llama3:70b", &names).expect("must match");
    assert_eq!(m, ModelMatch::exact());
    assert!(m.exact);
    assert!(m.alternative.is_none());
}

#[test]
fn unrelated_id_does_not_match() {
    let names = vec!["mistral:7b".to_string()];
    assert!(pick_match("zephyr", &names).is_none());
}

#[test]
fn empty_registry_returns_none() {
    let names: Vec<String> = Vec::new();
    assert!(pick_match("llama3:70b", &names).is_none());
}
