//! Test 5 — registry has `llama-3-70b-local`, Ollama has `llama3:70b`.
//! `pick_match` must yield a fuzzy match with the alternative populated.

use kei_llm_router::{normalise_base, pick_match};

#[test]
fn fuzzy_matches_when_query_normalises_to_substring() {
    let names = vec!["llama3:70b".to_string()];
    let m = pick_match("llama-3-70b-local", &names).expect("must fuzzy-match");
    assert!(!m.exact);
    assert_eq!(m.alternative.as_deref(), Some("llama3:70b"));
}

#[test]
fn normalise_strips_quant_suffixes() {
    // mlx quant suffix
    assert_eq!(normalise_base("Llama-3-70B-mlx-q4"), "llama370b");
    // local suffix
    assert_eq!(normalise_base("llama-3-70b-local"), "llama370b");
    // Ollama tag
    assert_eq!(normalise_base("llama3:70b"), "llama370b");
}

#[test]
fn first_fuzzy_candidate_wins() {
    let names = vec![
        "llama3:8b".to_string(),
        "llama3:70b".to_string(),
        "qwen3:4b".to_string(),
    ];
    let m = pick_match("llama-3", &names).expect("must match");
    assert!(!m.exact);
    // first fuzzy hit wins — order is preserved by pick_match.
    assert_eq!(m.alternative.as_deref(), Some("llama3:8b"));
}
