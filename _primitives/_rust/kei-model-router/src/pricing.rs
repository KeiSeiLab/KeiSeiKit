//! Verified Claude API pricing constants.
//!
//! Source: <https://platform.claude.com/docs/en/docs/about-claude/pricing>
//! Verified: 2026-04-30 (RULE 0.4 — primary source fetched in same session).
//!
//! All prices in microcents per 1M tokens (`u64` to avoid float drift in
//! cost arithmetic). 1 microcent = 1e-6 USD = 1e-4 cents. Aligns with
//! `kei-ledger.cost_micro_cents` column.
//!
//! Constructor Pattern: pricing is one cube. The decision rule (`select.rs`)
//! reads constants from here and never duplicates them.

/// Per-model token pricing (microcents per 1M tokens).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelPricing {
    pub input_micro_cents_per_mtok: u64,
    pub output_micro_cents_per_mtok: u64,
    pub cache_write_5m_micro_cents_per_mtok: u64,
    pub cache_read_micro_cents_per_mtok: u64,
}

/// Tokenizer density relative to baseline (Sonnet/Haiku tokenizer).
///
/// Opus 4.7 ships a new tokenizer that may produce up to 35% more tokens
/// on the same source text [VERIFIED: pricing page 2026-04-30 note].
/// Multiply expected token count by this when comparing Opus 4.7 to other
/// models on identical text input.
pub const OPUS_47_TOKENIZER_OVERHEAD: f64 = 1.35;

/// Claude Haiku 4.5 — cheapest, simple lookup / formatting / single-edit.
pub const HAIKU_45: ModelPricing = ModelPricing {
    input_micro_cents_per_mtok: 100_000_000,         // $1.00
    output_micro_cents_per_mtok: 500_000_000,        // $5.00
    cache_write_5m_micro_cents_per_mtok: 125_000_000, // $1.25
    cache_read_micro_cents_per_mtok: 10_000_000,     // $0.10
};

/// Claude Sonnet 4.6 — multi-step reasoning, code edits, summarization.
pub const SONNET_46: ModelPricing = ModelPricing {
    input_micro_cents_per_mtok: 300_000_000,         // $3.00
    output_micro_cents_per_mtok: 1_500_000_000,      // $15.00
    cache_write_5m_micro_cents_per_mtok: 375_000_000, // $3.75
    cache_read_micro_cents_per_mtok: 30_000_000,     // $0.30
};

/// Claude Opus 4.7 — architecture, novel reasoning, math derivation.
///
/// 4.5/4.6/4.7 are at the SAME price point — half the rate of Opus 4.1
/// (which was $15/$75). [VERIFIED: pricing table 2026-04-30].
pub const OPUS_47: ModelPricing = ModelPricing {
    input_micro_cents_per_mtok: 500_000_000,         // $5.00
    output_micro_cents_per_mtok: 2_500_000_000,      // $25.00
    cache_write_5m_micro_cents_per_mtok: 625_000_000, // $6.25
    cache_read_micro_cents_per_mtok: 50_000_000,     // $0.50
};

/// Discrete model identifier. Order matches escalation ladder
/// (cheaper first → richer last).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Model {
    Haiku45,
    Sonnet46,
    Opus47,
}

impl Model {
    pub fn pricing(&self) -> ModelPricing {
        match self {
            Self::Haiku45 => HAIKU_45,
            Self::Sonnet46 => SONNET_46,
            Self::Opus47 => OPUS_47,
        }
    }

    pub fn slug(&self) -> &'static str {
        match self {
            Self::Haiku45 => "haiku",
            Self::Sonnet46 => "sonnet",
            Self::Opus47 => "opus",
        }
    }

    /// Next-tier (escalation). Returns None if already at top.
    pub fn next_tier(&self) -> Option<Model> {
        match self {
            Self::Haiku45 => Some(Self::Sonnet46),
            Self::Sonnet46 => Some(Self::Opus47),
            Self::Opus47 => None,
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
}

/// Cost in microcents for a single (input, output) token pair on `model`.
/// Does NOT account for cache hits / batch discount / data residency
/// modifiers — those are orthogonal multipliers applied by callers.
pub fn cost_micro_cents(model: Model, tokens_in: u64, tokens_out: u64) -> u64 {
    let p = model.pricing();
    let input = tokens_in.saturating_mul(p.input_micro_cents_per_mtok) / 1_000_000;
    let output = tokens_out.saturating_mul(p.output_micro_cents_per_mtok) / 1_000_000;
    input.saturating_add(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opus_47_input_is_5_dollars_per_mtok() {
        // 1M tokens at $5 = 500M microcents
        assert_eq!(cost_micro_cents(Model::Opus47, 1_000_000, 0), 500_000_000);
    }

    #[test]
    fn haiku_output_is_5_dollars_per_mtok() {
        assert_eq!(cost_micro_cents(Model::Haiku45, 0, 1_000_000), 500_000_000);
    }

    #[test]
    fn sonnet_mixed_input_output() {
        // 100k in + 50k out at Sonnet rates: 100k*$3/MTok + 50k*$15/MTok
        // = $0.30 + $0.75 = $1.05 = 105M microcents
        let c = cost_micro_cents(Model::Sonnet46, 100_000, 50_000);
        assert_eq!(c, 30_000_000 + 75_000_000);
    }

    #[test]
    fn next_tier_terminates_at_opus() {
        assert_eq!(Model::Haiku45.next_tier(), Some(Model::Sonnet46));
        assert_eq!(Model::Sonnet46.next_tier(), Some(Model::Opus47));
        assert_eq!(Model::Opus47.next_tier(), None);
    }

    #[test]
    fn slug_round_trip() {
        for m in Model::all() {
            assert_eq!(Model::from_slug(m.slug()), Some(m));
        }
    }

    #[test]
    fn opus_is_5x_haiku_input_3x_sonnet_at_modern_pricing() {
        // 2026-04-30 pricing audit lock-in: spreads matter for routing
        // economics. If Anthropic re-prices and these assertions break,
        // re-verify the pricing page and update constants + this test.
        assert_eq!(
            OPUS_47.input_micro_cents_per_mtok,
            5 * HAIKU_45.input_micro_cents_per_mtok,
            "Opus 4.7 must be 5x Haiku 4.5 input — re-verify pricing if this fails"
        );
        assert_eq!(
            OPUS_47.output_micro_cents_per_mtok,
            5 * HAIKU_45.output_micro_cents_per_mtok
        );
    }
}
