//! Retry-ladder bookkeeping for the router.
//!
//! When a model returns `outcome != functional` on first pass, we may
//! want to retry on the next-tier model (Haiku → Sonnet → Opus). The
//! escalation depth is recorded in the ledger row so future posterior
//! aggregation discounts retries.
//!
//! Constructor Pattern: pure-fn cube, no I/O. Side effects (writing the
//! depth back to ledger) happen in caller / hook.

use crate::pricing::Model;

/// Hard ceiling on escalation depth. Two retries (depth 1 and 2) gives
/// Haiku → Sonnet → Opus ladder; beyond that we surrender.
pub const MAX_ESCALATION_DEPTH: u32 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscalationDecision {
    /// Retry on the next-tier model.
    Retry { next: Model, depth: u32 },
    /// No more tiers above OR depth ceiling reached. Caller should
    /// either accept the partial outcome or escalate to a human.
    Surrender,
}

/// Decide whether to retry given (current_model, current_depth, outcome).
pub fn next_after_failure(
    current: Model,
    depth: u32,
    outcome_is_failure: bool,
) -> EscalationDecision {
    if !outcome_is_failure {
        return EscalationDecision::Surrender; // shouldn't happen, defensive
    }
    if depth >= MAX_ESCALATION_DEPTH {
        return EscalationDecision::Surrender;
    }
    match current.next_tier() {
        Some(next) => EscalationDecision::Retry {
            next,
            depth: depth + 1,
        },
        None => EscalationDecision::Surrender,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn haiku_failure_escalates_to_sonnet() {
        assert_eq!(
            next_after_failure(Model::Haiku45, 0, true),
            EscalationDecision::Retry {
                next: Model::Sonnet46,
                depth: 1
            }
        );
    }

    #[test]
    fn sonnet_failure_escalates_to_opus() {
        assert_eq!(
            next_after_failure(Model::Sonnet46, 1, true),
            EscalationDecision::Retry {
                next: Model::Opus47,
                depth: 2
            }
        );
    }

    #[test]
    fn opus_failure_surrenders() {
        assert_eq!(
            next_after_failure(Model::Opus47, 1, true),
            EscalationDecision::Surrender
        );
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
