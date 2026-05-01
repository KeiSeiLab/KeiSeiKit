//! Source fetcher trait — frozen interface, default impl is a no-op stub.
//!
//! Actual WebFetch/WebSearch integration is out-of-scope for v0.14 part A.
//! Later milestones plug real providers (anthropic-websearch, SerpAPI, etc.).

use crate::types::Source;

/// Implement this trait to integrate a live search provider.
pub trait SourceFetcher {
    /// Fetch sources for `claim`. Returns (source, cost_microcents).
    /// Cost is real — the budget is charged by the pipeline, not by impl.
    fn fetch(&self, claim: &str) -> (Vec<Source>, i64);
}

/// Default stub — returns empty. Frozen interface, no runtime side-effects.
pub struct StubFetcher;

impl SourceFetcher for StubFetcher {
    fn fetch(&self, _claim: &str) -> (Vec<Source>, i64) {
        // TODO(v0.15): wire to real websearch. Kept as stub per v0.14 spec.
        (Vec::new(), 0)
    }
}
