//! Bucket counts over the hydrated graph.
//!
//! Constructor Pattern: one cube = `Stats` struct + `compute_stats` fn +
//! text renderer. Deterministic output order (sorted keys) so downstream
//! diffing / snapshot tests stay stable across runs.

use crate::graph::Graph;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Default, Serialize)]
pub struct Stats {
    pub total: usize,
    /// status -> count (e.g. "running" -> 3, "done" -> 12)
    pub by_status: BTreeMap<String, usize>,
    /// has-dna count (non-NULL dna column)
    pub with_dna: usize,
    /// top-level roots (no parent_branch or parent outside the ledger)
    pub roots: usize,
    /// non-root forks
    pub forks: usize,
}

/// Build `Stats` from an in-memory graph — pure, no I/O.
pub fn compute_stats(graph: &Graph) -> Stats {
    let mut s = Stats::default();
    s.total = graph.nodes.len();
    s.roots = graph.roots.len();
    s.forks = s.total.saturating_sub(s.roots);
    for n in &graph.nodes {
        *s.by_status.entry(n.status.clone()).or_insert(0) += 1;
        if n.dna.is_some() {
            s.with_dna += 1;
        }
    }
    s
}

/// Human-readable text report. Ordering: total, roots/forks, with_dna,
/// then each status bucket alphabetically (BTreeMap iter).
pub fn render_stats(stats: &Stats) -> String {
    let mut out = String::new();
    out.push_str(&format!("total:    {}\n", stats.total));
    out.push_str(&format!("roots:    {}\n", stats.roots));
    out.push_str(&format!("forks:    {}\n", stats.forks));
    out.push_str(&format!("with_dna: {}\n", stats.with_dna));
    out.push_str("by_status:\n");
    for (status, count) in &stats.by_status {
        out.push_str(&format!("  {status:<10} {count}\n"));
    }
    out
}
