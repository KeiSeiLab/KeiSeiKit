//! Decision rule — public API for the router.
//!
//! Two surfaces:
//!   - `pick(profile_id, registry)` — registry-backed profile resolution.
//!     Returns `(provider_id, model_id)` from the profile's `default_model_ref`.
//!   - `select(input, conn)` — empirical posterior + cost argmin.
//!     Implementation lives in `select_posterior.rs`.
//!
//! Constructor Pattern: types + thin delegation cube.

use crate::complexity::ComplexityEstimate;
use crate::kernel::KernelWeights;
use crate::pricing::Model;
use crate::registry::Registry;
use crate::select_posterior;
use rusqlite::{Connection, Result as SqlResult};

// ──────────────────────────────────────────────────────────────────────────────
// Registry-backed pick
// ──────────────────────────────────────────────────────────────────────────────

/// Resolve `(provider_id, model_id)` for a given agent profile.
///
/// Uses `profile.default_model_ref` (format `<provider_id>/<model_id>`).
/// Returns `None` if the profile is unknown or the model is deprecated.
pub fn pick(profile_id: &str, registry: &Registry) -> Option<(String, String)> {
    let profile = registry.profile_by_id(profile_id)?;
    let (provider_id, model_id) = profile.split_model_ref()?;
    if let Some(m) = registry.model_by_id(model_id) {
        if m.is_deprecated() {
            return None;
        }
    }
    Some((provider_id.to_string(), model_id.to_string()))
}

// ──────────────────────────────────────────────────────────────────────────────
// Types
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct DecisionInput {
    pub full_dna: String,
    pub prompt: String,
    pub q_threshold: f64,
    pub delta: f64,
    pub fallback: Model,
    /// Pinned override: if Some, skip routing and use this.
    pub pinned: Option<Model>,
    pub kernel_weights: KernelWeights,
    pub tokens_in: Option<u64>,
    pub tokens_out: Option<u64>,
}

impl DecisionInput {
    pub const DEFAULT_TOKENS_IN: u64 = 4_000;
    pub const DEFAULT_TOKENS_OUT: u64 = 1_500;

    pub fn new(full_dna: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            full_dna: full_dna.into(),
            prompt: prompt.into(),
            q_threshold: 0.70,
            delta: 0.10,
            fallback: Model::Opus47,
            pinned: None,
            kernel_weights: KernelWeights::default(),
            tokens_in: None,
            tokens_out: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Decision {
    pub model: Model,
    pub expected_cost_micro_cents: u64,
    pub quality_lower_bound: f64,
    pub posterior_n: u32,
    pub complexity: ComplexityEstimate,
    pub reason: &'static str,
}

// ──────────────────────────────────────────────────────────────────────────────
// select() — delegates to select_posterior
// ──────────────────────────────────────────────────────────────────────────────

pub fn select(input: &DecisionInput, conn: &Connection) -> SqlResult<Decision> {
    select_posterior::select(input, conn)
}

// ──────────────────────────────────────────────────────────────────────────────
// Tests — pick() only; select() tests live in select_posterior.rs
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn reg() -> Registry {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent().unwrap()
            .parent().unwrap()
            .parent().unwrap()
            .join("_blocks/registries");
        Registry::load_from(&dir).expect("registry load failed")
    }

    #[test]
    fn pick_default_model_for_code_implementer_rust() {
        let r = reg();
        let (prov, model) = pick("code-implementer-rust", &r).unwrap();
        assert_eq!(prov, "anthropic");
        assert_eq!(model, "claude-sonnet-4-6");
    }

    #[test]
    fn pick_codex_reviewer_uses_codex_provider() {
        let r = reg();
        let (prov, model) = pick("codex-reviewer", &r).unwrap();
        assert_eq!(prov, "codex");
        assert_eq!(model, "gpt-5-codex");
    }

    #[test]
    fn pick_unknown_profile_returns_none() {
        let r = reg();
        assert!(pick("does-not-exist", &r).is_none());
    }
}
