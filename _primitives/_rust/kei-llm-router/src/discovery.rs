//! Cross-backend model discovery.
//!
//! Constructor Pattern: ONE responsibility — given a `model_id`, ask each
//! viable backend "do you have this model installed?" and return the
//! aggregated answers. Each backend's lookup goes through its own crate
//! (W57 tags / W58 .gguf scan / W59 HF cache).
//!
//! Match grading:
//!   * **exact**: backend's own identifier is byte-equal to `model_id`
//!     (e.g. Ollama tag `qwen3:4b` vs CLI input `qwen3:4b`).
//!   * **fuzzy**: a substring match in either direction on a normalised
//!     base name (drop suffixes like `-mlx-q4`, `-Q4_K_M`, tag `:Xb`).
//!   * **none**: backend doesn't have it — entry omitted from the result.

use serde::{Deserialize, Serialize};

use crate::backend::{from_tier, BackendKind};
use kei_machine_probe::Machine;

/// Per-backend match outcome.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelMatch {
    pub exact: bool,
    /// Populated on a fuzzy match — the backend-side identifier the
    /// router will use if this entry wins. `None` when `exact = true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternative: Option<String>,
}

impl ModelMatch {
    pub fn exact() -> Self {
        Self { exact: true, alternative: None }
    }

    pub fn fuzzy(alt: impl Into<String>) -> Self {
        Self { exact: false, alternative: Some(alt.into()) }
    }
}

/// Walk the machine's viable backends and ask each one whether it has
/// `model_id`. Result is in machine-recommended priority order with
/// non-matches dropped.
pub async fn discover_models(
    machine: &Machine,
    model_id: &str,
) -> Vec<(BackendKind, ModelMatch)> {
    let recs = kei_machine_probe::recommend(machine);
    let mut out = Vec::new();
    for tier in &recs.viable_backends {
        let kind = match from_tier(tier) {
            Some(k) => k,
            None => continue,
        };
        if let Some(m) = match_for(kind, model_id).await {
            out.push((kind, m));
        }
    }
    out
}

async fn match_for(kind: BackendKind, model_id: &str) -> Option<ModelMatch> {
    match kind {
        BackendKind::Ollama => match_ollama(model_id).await,
        BackendKind::LlamaCpp => match_llamacpp(model_id),
        BackendKind::Mlx => match_mlx(model_id),
    }
}

async fn match_ollama(model_id: &str) -> Option<ModelMatch> {
    let client = kei_llm_ollama::Client::default();
    let tags = client.tags().await.ok()?;
    let names: Vec<String> = tags.models.iter().map(|m| m.name.clone()).collect();
    pick_match(model_id, &names)
}

fn match_llamacpp(model_id: &str) -> Option<ModelMatch> {
    let dirs = kei_llm_llamacpp::models::default_dirs();
    let mut all = Vec::new();
    for d in dirs {
        if let Ok(mut found) = kei_llm_llamacpp::list_models(&d) {
            all.append(&mut found);
        }
    }
    let names: Vec<String> = all.iter().map(|m| m.name.clone()).collect();
    pick_match(model_id, &names)
}

fn match_mlx(model_id: &str) -> Option<ModelMatch> {
    let dir = kei_llm_mlx::default_cache_dir()?;
    let entries = kei_llm_mlx::list_models(&dir);
    let names: Vec<String> = entries.iter().map(|m| m.hf_id.clone()).collect();
    pick_match(model_id, &names)
}

/// Public for direct unit-testing — picks an exact then fuzzy match.
pub fn pick_match(model_id: &str, names: &[String]) -> Option<ModelMatch> {
    if names.iter().any(|n| n == model_id) {
        return Some(ModelMatch::exact());
    }
    fuzzy_pick(model_id, names).map(ModelMatch::fuzzy)
}

fn fuzzy_pick(model_id: &str, names: &[String]) -> Option<String> {
    let q = normalise_base(model_id);
    if q.is_empty() {
        return None;
    }
    for n in names {
        let nb = normalise_base(n);
        if nb.is_empty() {
            continue;
        }
        if nb.contains(&q) || q.contains(&nb) {
            return Some(n.clone());
        }
    }
    None
}

/// Strip well-known suffixes / quant tags / version separators and
/// lowercase. The result is alphanumeric-only, with quant / role
/// suffix tokens dropped. Public for unit tests.
///
/// Examples (see `discovery_fuzzy.rs`):
///   * `Llama-3-70B-mlx-q4`  → `llama370b`
///   * `llama-3-70b-local`   → `llama370b`
///   * `llama3:70b`          → `llama370b`
pub fn normalise_base(s: &str) -> String {
    let lower = s.to_lowercase();
    let cleaned: String = lower
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { ' ' })
        .collect();
    let pieces: Vec<&str> = cleaned
        .split_whitespace()
        .filter(|p| !is_quant_token(p))
        .collect();
    pieces.join("")
}

fn is_quant_token(p: &str) -> bool {
    matches!(
        p,
        "q2" | "q3" | "q4" | "q5" | "q6" | "q8" | "f16" | "f32" | "bf16" | "mlx"
            | "4bit" | "8bit" | "instruct" | "chat" | "local"
    )
}
