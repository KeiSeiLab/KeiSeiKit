//! Pricing helpers — registry-backed cost computation.
//!
//! Source of truth: `models.toml` via `registry::Registry`.
//! All prices are in microcents per 1M tokens (u64) to avoid float drift.
//! 1 microcent = 1e-6 USD = 1e-4 cents.
//!
//! `cost_micro_cents(model_id, tokens_in, tokens_out, registry)` is the
//! primary entry point; returns None if model_id is unknown.
//!
//! Legacy `Model` enum is kept for `posterior.rs` / `calibrate.rs` which
//! still use model slugs for SQL ledger queries. New code should use model
//! id strings from the registry directly.
//!
//! Constructor Pattern: pricing is one cube. Decision rule (`select.rs`)
//! reads from here and never duplicates cost arithmetic.

use crate::registry::Registry;

/// Compute cost in microcents for one (input, output) token pair.
///
/// Returns `None` if `model_id` is not present in the registry.
/// Does NOT account for cache hits / batch discounts — those are applied
/// by callers as orthogonal multipliers.
pub fn cost_micro_cents(
    model_id: &str,
    tokens_in: u64,
    tokens_out: u64,
    registry: &Registry,
) -> Option<u64> {
    let m = registry.model_by_id(model_id)?;
    let input = tokens_in.saturating_mul(m.cost_input_per_mtok_micro) / 1_000_000;
    let output = tokens_out.saturating_mul(m.cost_output_per_mtok_micro) / 1_000_000;
    Some(input.saturating_add(output))
}

/// Tokenizer density of Opus 4.7 relative to Sonnet/Haiku baseline.
/// Multiply expected token count by this when comparing Opus 4.7 to
/// other models on identical text input.
pub const OPUS_47_TOKENIZER_OVERHEAD: f64 = 1.35;

// ──────────────────────────────────────────────────────────────────────────────
// Legacy Model enum — kept for posterior.rs + calibrate.rs SQL lookup by slug.
// Do NOT use in new code; reference registry model ids directly.
// ──────────────────────────────────────────────────────────────────────────────

/// Discrete Claude model identifier (legacy). Order = escalation ladder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Model {
    Haiku45,
    Sonnet46,
    Opus47,
}

impl Model {
    pub fn slug(&self) -> &'static str {
        match self {
            Self::Haiku45 => "claude-haiku-4-5",
            Self::Sonnet46 => "claude-sonnet-4-6",
            Self::Opus47 => "claude-opus-4-7",
        }
    }

    /// Legacy short slug used in ledger rows written before 2026-05.
    /// Used for backward-compat SQL queries (`WHERE model = slug OR model = legacy_slug`).
    pub fn legacy_slug(&self) -> &'static str {
        match self {
            Self::Haiku45 => "haiku",
            Self::Sonnet46 => "sonnet",
            Self::Opus47 => "opus",
        }
    }

    pub fn from_slug(s: &str) -> Option<Model> {
        match s {
            "haiku" | "haiku-4.5" | "claude-haiku-4-5" => Some(Self::Haiku45),
            "sonnet" | "sonnet-4.6" | "claude-sonnet-4-6" => Some(Self::Sonnet46),
            "opus" | "opus-4.7" | "claude-opus-4-7" => Some(Self::Opus47),
            _ => None,
        }
    }

    pub fn all() -> [Model; 3] {
        [Self::Haiku45, Self::Sonnet46, Self::Opus47]
    }

    /// Next escalation tier. Returns None if already at Opus47 (top).
    ///
    /// Finding 10: consolidated here from escalate.rs so all inherent Model
    /// behaviour lives in one impl block. escalate.rs uses pure functions
    /// that take &Model as argument.
    pub fn next_tier(&self) -> Option<Model> {
        match self {
            Self::Haiku45 => Some(Self::Sonnet46),
            Self::Sonnet46 => Some(Self::Opus47),
            Self::Opus47 => None,
        }
    }
}

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
    fn sonnet_mixed_cost_matches_toml() {
        // 100k in + 50k out:
        // input:  100_000 * 300_000_000 / 1_000_000 = 30_000_000
        // output:  50_000 * 1_500_000_000 / 1_000_000 = 75_000_000
        let r = reg();
        let c = cost_micro_cents("claude-sonnet-4-6", 100_000, 50_000, &r).unwrap();
        assert_eq!(c, 30_000_000 + 75_000_000, "got {c}");
    }

    #[test]
    fn opus_input_1m_is_500m_microcents() {
        let r = reg();
        let c = cost_micro_cents("claude-opus-4-7", 1_000_000, 0, &r).unwrap();
        assert_eq!(c, 500_000_000);
    }

    #[test]
    fn haiku_output_1m_is_500m_microcents() {
        let r = reg();
        let c = cost_micro_cents("claude-haiku-4-5", 0, 1_000_000, &r).unwrap();
        assert_eq!(c, 500_000_000);
    }

    #[test]
    fn unknown_model_returns_none() {
        let r = reg();
        assert!(cost_micro_cents("does-not-exist", 1_000, 1_000, &r).is_none());
    }

    #[test]
    fn slug_round_trip() {
        for m in Model::all() {
            assert_eq!(Model::from_slug(m.slug()), Some(m));
        }
    }
}
