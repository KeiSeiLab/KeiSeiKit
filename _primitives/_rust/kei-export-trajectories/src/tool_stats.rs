//! Tool-statistic extraction + key-set normalization.
//!
//! Constructor Pattern: pure functions over the in-memory event stream
//! returned by `ledger_reader`. Stats live in a `BTreeMap` so the JSONL
//! output is byte-stable across runs (golden-test friendly).
//!
//! The Hermes spec demands that every JSONL line in an export carry the
//! SAME `tool_stats` key set — the union of all tools observed across all
//! trajectories. Per-trajectory missing tools land as `{count:0,success:0,
//! failure:0}` zero defaults so HuggingFace `datasets` doesn't choke on a
//! ragged Arrow schema.

use crate::sharegpt::{ToolStats, Trajectory};
use std::collections::{BTreeMap, BTreeSet};

/// One observed tool invocation, distilled out of the event stream by
/// `ledger_reader`. `success=false` means the call surfaced an error — the
/// caller decides what counts as failure (we treat any `is_error=1` row in
/// `kei-memory.events` as a failure).
#[derive(Debug, Clone)]
pub struct ToolEvent {
    pub tool: String,
    pub success: bool,
}

/// Aggregate a flat list of `ToolEvent`s into per-tool counters. Returned
/// map preserves only tools that were actually invoked — call
/// `normalize_keys` afterwards to fill in zero defaults for the full
/// export-wide key set.
pub fn aggregate_tool_stats(events: &[ToolEvent]) -> BTreeMap<String, ToolStats> {
    let mut out: BTreeMap<String, ToolStats> = BTreeMap::new();
    for ev in events {
        let entry = out.entry(ev.tool.clone()).or_default();
        entry.count += 1;
        if ev.success {
            entry.success += 1;
        } else {
            entry.failure += 1;
        }
    }
    out
}

/// Walk every trajectory once to build the union key set, then walk again
/// to fill missing keys with zero defaults. Two-pass O(N) is fine — the
/// alternative (single-pass with rolling backfill) corrupts already-emitted
/// trajectories and breaks streaming writers.
///
/// Also refreshes the `tool_error_counts` mirror so the two fields stay
/// consistent — callers MUST NOT mutate `tool_stats` after this returns.
pub fn normalize_keys(trajectories: &mut [Trajectory]) {
    let union = collect_union(trajectories);
    for t in trajectories.iter_mut() {
        fill_zero_defaults(t, &union);
        t.refresh_error_counts();
    }
}

/// First pass — collect every tool name observed across the whole batch.
fn collect_union(trajectories: &[Trajectory]) -> BTreeSet<String> {
    let mut union = BTreeSet::new();
    for t in trajectories {
        for k in t.tool_stats.keys() {
            union.insert(k.clone());
        }
    }
    union
}

/// Second pass — for each trajectory, insert zero-default entries for any
/// tool name in the union that the trajectory did not invoke.
fn fill_zero_defaults(t: &mut Trajectory, union: &BTreeSet<String>) {
    for tool in union {
        t.tool_stats
            .entry(tool.clone())
            .or_insert_with(ToolStats::default);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aggregate_counts_success_and_failure() {
        let events = vec![
            ToolEvent { tool: "Read".into(), success: true },
            ToolEvent { tool: "Read".into(), success: true },
            ToolEvent { tool: "Read".into(), success: false },
            ToolEvent { tool: "Bash".into(), success: true },
        ];
        let stats = aggregate_tool_stats(&events);
        assert_eq!(stats["Read"].count, 3);
        assert_eq!(stats["Read"].success, 2);
        assert_eq!(stats["Read"].failure, 1);
        assert_eq!(stats["Bash"].count, 1);
        assert_eq!(stats["Bash"].failure, 0);
    }

    fn empty_traj(idx: u64, tool: &str, count: u64, success: u64, failure: u64) -> Trajectory {
        Trajectory {
            prompt_index: idx,
            conversations: vec![],
            completed: true,
            tool_stats: BTreeMap::from([(
                tool.to_string(),
                ToolStats { count, success, failure },
            )]),
            tool_error_counts: BTreeMap::new(),
            metadata: serde_json::Map::new(),
        }
    }

    #[test]
    fn normalize_fills_zero_defaults() {
        let mut batch = vec![
            empty_traj(0, "Read", 1, 1, 0),
            empty_traj(1, "Bash", 2, 1, 1),
        ];
        normalize_keys(&mut batch);
        assert!(batch[0].tool_stats.contains_key("Bash"));
        assert!(batch[1].tool_stats.contains_key("Read"));
        assert_eq!(batch[0].tool_stats["Bash"], ToolStats::default());
        assert_eq!(batch[0].tool_error_counts["Bash"], 0);
        assert_eq!(batch[1].tool_error_counts["Bash"], 1);
    }
}
