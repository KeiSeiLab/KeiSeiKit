//! `Pricing` struct + `estimate` cost helper.
//!
//! Every pricing row in the SSoT TOML ships with `status = "placeholder"` and
//! a `source_url` per RULE 0.4. Real micro-cents/Mtok come from a follow-up
//! verification commit. Callers should inspect `status` before quoting cost.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Pricing {
    /// Micro-cents per million input tokens.
    pub input_per_mtok_micro: u64,
    /// Micro-cents per million output tokens.
    pub output_per_mtok_micro: u64,
    pub status: PricingStatus,
    #[serde(default)]
    pub source_url: Option<String>,
    /// ISO-8601 date the row was last verified, e.g. "2026-04-27".
    #[serde(default)]
    pub verified_at: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PricingStatus {
    #[serde(rename = "verified")]
    Verified,
    #[serde(rename = "needs-verification")]
    NeedsVerification,
    #[serde(rename = "placeholder")]
    Placeholder,
}

impl PricingStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            PricingStatus::Verified => "verified",
            PricingStatus::NeedsVerification => "needs-verification",
            PricingStatus::Placeholder => "placeholder",
        }
    }
}

/// Estimate total micro-cents for a (`in_tokens`, `out_tokens`) call.
///
/// Returns `0` when both rates are placeholders (the common case until
/// the verification follow-up commit lands).
pub fn estimate(p: &Pricing, in_tokens: u64, out_tokens: u64) -> u64 {
    let input_cost = mul_div_round(in_tokens, p.input_per_mtok_micro, 1_000_000);
    let output_cost = mul_div_round(out_tokens, p.output_per_mtok_micro, 1_000_000);
    input_cost.saturating_add(output_cost)
}

/// `(a * b) / d` with saturating multiply, integer floor.
///
/// Splits the work so that token counts approaching `u64::MAX` cannot
/// silently overflow into a wrong cost.
fn mul_div_round(a: u64, b: u64, d: u64) -> u64 {
    a.saturating_mul(b) / d.max(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn synth_pricing(in_rate: u64, out_rate: u64) -> Pricing {
        Pricing {
            input_per_mtok_micro: in_rate,
            output_per_mtok_micro: out_rate,
            status: PricingStatus::NeedsVerification,
            source_url: None,
            verified_at: None,
        }
    }

    #[test]
    fn placeholder_zero_cost() {
        let p = synth_pricing(0, 0);
        assert_eq!(estimate(&p, 1_000, 500), 0);
    }

    #[test]
    fn synthetic_rate_one_million_tokens() {
        // 1 Mtok @ 100 micro / Mtok input + 0 output → 100 micro.
        let p = synth_pricing(100, 0);
        assert_eq!(estimate(&p, 1_000_000, 0), 100);
    }

    #[test]
    fn synthetic_rate_split() {
        // 10k input @ 1000 micro/Mtok = 10 micro
        // 5k output @ 2000 micro/Mtok = 10 micro
        // total = 20 micro
        let p = synth_pricing(1_000, 2_000);
        assert_eq!(estimate(&p, 10_000, 5_000), 20);
    }
}
