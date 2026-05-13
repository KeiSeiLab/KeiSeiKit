//! Retry-ladder bookkeeping for the router.
//!
//! Two surfaces:
//!   - `next_model(current_model_id, provider_id, registry)` — registry-backed
//!     escalation: returns the next non-deprecated model in the provider's
//!     cost-output ascending order. Returns None if already at the most
//!     expensive non-deprecated model.
//!   - `next_after_failure(current, depth, failure)` — legacy Claude-only
//!     ladder (kept for backward compatibility with `calibrate.rs`).
//!
//! Constructor Pattern: pure-fn cube, no I/O. Side effects (ledger write)
//! happen in callers.

use crate::pricing::Model;
use crate::registry::Registry;

pub const MAX_ESCALATION_DEPTH: u32 = 2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EscalationDecision {
    /// Retry on the next-tier model.
    Retry { next: Model, depth: u32 },
    /// No more tiers above OR depth ceiling reached.
    Surrender,
}

// ──────────────────────────────────────────────────────────────────────────────
// Registry-backed escalation
// ──────────────────────────────────────────────────────────────────────────────

/// Given `current_model_id` within `provider_id`, return the id of the next
/// more expensive non-deprecated model from the registry (sorted by
/// `cost_output_per_mtok_micro` ascending). Returns `None` if already at top.
pub fn next_model<'r>(
    current_model_id: &str,
    provider_id: &str,
    registry: &'r Registry,
) -> Option<&'r str> {
    let sorted = registry.models_for_provider(provider_id);
    let mut found_current = false;
    for m in &sorted {
        if found_current {
            return Some(&m.id);
        }
        if m.id == current_model_id {
            found_current = true;
        }
    }
    None // current not found, or already at top
}

// ──────────────────────────────────────────────────────────────────────────────
// Legacy ladder (Claude-only)
// ──────────────────────────────────────────────────────────────────────────────

pub fn next_after_failure(
    current: Model,
    depth: u32,
    outcome_is_failure: bool,
) -> EscalationDecision {
    if !outcome_is_failure {
        return EscalationDecision::Surrender;
    }
    if depth >= MAX_ESCALATION_DEPTH {
        return EscalationDecision::Surrender;
    }
    match current.next_tier() {
        Some(next) => EscalationDecision::Retry { next, depth: depth + 1 },
        None => EscalationDecision::Surrender,
    }
}

impl Model {
    /// Next-tier (escalation). Returns None if already at top.
    pub fn next_tier(&self) -> Option<Model> {
        match self {
            Self::Haiku45 => Some(Self::Sonnet46),
            Self::Sonnet46 => Some(Self::Opus47),
            Self::Opus47 => None,
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Tests
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

    // ── next_model() tests ────────────────────────────────────────────────

    #[test]
    fn haiku_escalates_to_sonnet_within_anthropic() {
        let r = reg();
        let next = next_model("claude-haiku-4-5", "anthropic", &r);
        assert_eq!(next, Some("claude-sonnet-4-6"));
    }

    #[test]
    fn sonnet_escalates_to_opus_within_anthropic() {
        let r = reg();
        let next = next_model("claude-sonnet-4-6", "anthropic", &r);
        assert_eq!(next, Some("claude-opus-4-7"));
    }

    #[test]
    fn opus_at_top_returns_none() {
        let r = reg();
        let next = next_model("claude-opus-4-7", "anthropic", &r);
        assert!(next.is_none(), "expected None at top, got {next:?}");
    }

    #[test]
    fn unknown_model_returns_none() {
        let r = reg();
        let next = next_model("does-not-exist", "anthropic", &r);
        assert!(next.is_none());
    }

    #[test]
    fn escalation_skips_deprecated_models() {
        // All current Anthropic models have deprecated_at = "" so this
        // verifies the escalation ladder works without deprecated entries.
        let r = reg();
        let ms = r.models_for_provider("anthropic");
        for m in &ms {
            assert!(!m.is_deprecated(), "{} is deprecated but should not be", m.id);
        }
    }

    // ── legacy next_after_failure() tests ────────────────────────────────

    #[test]
    fn haiku_failure_escalates_to_sonnet() {
        assert_eq!(
            next_after_failure(Model::Haiku45, 0, true),
            EscalationDecision::Retry { next: Model::Sonnet46, depth: 1 }
        );
    }

    #[test]
    fn sonnet_failure_escalates_to_opus() {
        assert_eq!(
            next_after_failure(Model::Sonnet46, 1, true),
            EscalationDecision::Retry { next: Model::Opus47, depth: 2 }
        );
    }

    #[test]
    fn opus_failure_surrenders() {
        assert_eq!(next_after_failure(Model::Opus47, 1, true), EscalationDecision::Surrender);
    }

    #[test]
    fn ceiling_reached_surrenders_even_below_top() {
        assert_eq!(
            next_after_failure(Model::Haiku45, MAX_ESCALATION_DEPTH, true),
            EscalationDecision::Surrender
        );
    }

    #[test]
    fn success_returns_surrender_defensively() {
        assert_eq!(
            next_after_failure(Model::Haiku45, 0, false),
            EscalationDecision::Surrender
        );
    }
}
