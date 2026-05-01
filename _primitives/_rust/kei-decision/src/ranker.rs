//! Topological sort + score-based ranker.
//!
//! Inputs: a `Vec<RawAction>` plus a parallel `Vec<ActionKind>` from the
//! classifier. Output: `Vec<RankedAction>` ordered so that:
//!   1. All deps of an action come before it.
//!   2. Within deps-equivalent groups, higher score wins.
//!
//! Score = severity_weight × (1 / max(effort_hours, 0.5)) × deps_factor
//!   severity_weight: HIGH=10, MEDIUM=5, LOW=2 (default 5)
//!   effort_hours:   parsed from "1-2h" / "30min" / "2-3h" / "1-2d" etc.
//!   deps_factor:    1.0 for no deps, 0.5 per upstream dep (penalises chains)

use serde::{Deserialize, Serialize};

use crate::classifier::ActionKind;
use crate::parser::RawAction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankedAction {
    pub raw: RawAction,
    pub kind: ActionKind,
    pub score: f64,
    pub rank: usize,
}

/// Topo-sort by deps, then score-rank within levels.
///
/// `actions` and `kinds` MUST be parallel slices. If a dep id refers to an
/// unknown action it is dropped from the dep set (best-effort).
pub fn rank_actions(actions: Vec<RawAction>, kinds: Vec<ActionKind>) -> Vec<RankedAction> {
    assert_eq!(actions.len(), kinds.len(), "rank_actions: parallel slice length mismatch");
    let scored: Vec<(RawAction, ActionKind, f64)> = actions
        .into_iter()
        .zip(kinds.into_iter())
        .map(|(a, k)| {
            let s = compute_score(&a);
            (a, k, s)
        })
        .collect();
    let order = topo_order(&scored);
    order
        .into_iter()
        .enumerate()
        .map(|(i, idx)| {
            let (raw, kind, score) = scored[idx].clone();
            RankedAction { raw, kind, score, rank: i + 1 }
        })
        .collect()
}

/// Deterministic topo order: Kahn's algorithm with score-then-id tie-break.
fn topo_order(scored: &[(RawAction, ActionKind, f64)]) -> Vec<usize> {
    let n = scored.len();
    let id_to_idx = build_id_map(scored);
    let (mut indeg, dependents) = build_dep_graph(scored, &id_to_idx);
    let mut ready: Vec<usize> = (0..n).filter(|i| indeg[*i] == 0).collect();
    sort_ready(&mut ready, scored);
    let mut order = Vec::with_capacity(n);
    while let Some(idx) = ready.pop() {
        order.push(idx);
        for &child in &dependents[idx] {
            indeg[child] -= 1;
            if indeg[child] == 0 {
                ready.push(child);
            }
        }
        sort_ready(&mut ready, scored);
    }
    if order.len() < n {
        // Cycle / orphan deps — dump remaining in original order so we never lose actions.
        for i in 0..n {
            if !order.contains(&i) {
                order.push(i);
            }
        }
    }
    order
}

fn build_id_map(scored: &[(RawAction, ActionKind, f64)]) -> std::collections::HashMap<String, usize> {
    scored.iter().enumerate().map(|(i, (a, _, _))| (a.id.clone(), i)).collect()
}

fn build_dep_graph(
    scored: &[(RawAction, ActionKind, f64)],
    id_to_idx: &std::collections::HashMap<String, usize>,
) -> (Vec<usize>, Vec<Vec<usize>>) {
    let n = scored.len();
    let mut indeg = vec![0usize; n];
    let mut dependents: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (i, (a, _, _)) in scored.iter().enumerate() {
        for d in &a.deps {
            if let Some(&parent_idx) = id_to_idx.get(d) {
                dependents[parent_idx].push(i);
                indeg[i] += 1;
            }
        }
    }
    (indeg, dependents)
}

/// Highest score sorts to the END of the ready queue (since we `pop`).
fn sort_ready(ready: &mut Vec<usize>, scored: &[(RawAction, ActionKind, f64)]) {
    ready.sort_by(|a, b| {
        scored[*a].2
            .partial_cmp(&scored[*b].2)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| scored[*b].0.id.cmp(&scored[*a].0.id))
    });
}

fn compute_score(a: &RawAction) -> f64 {
    let sev = severity_weight(&a.severity);
    let eff = parse_effort_hours(&a.effort).max(0.5);
    let deps = (0.5_f64).powi(a.deps.len() as i32);
    sev * (1.0 / eff) * deps
}

fn severity_weight(s: &str) -> f64 {
    let t = s.to_lowercase();
    if t.contains("high") || t.contains("critical") { 10.0 }
    else if t.contains("med") { 5.0 }
    else if t.contains("low") || t.contains("none") { 2.0 }
    else { 5.0 }
}

/// Parse "1-2h" / "30min" / "2-3h" / "1-2d" / "4-6h" → midpoint in hours.
fn parse_effort_hours(s: &str) -> f64 {
    let t = s.to_lowercase().replace(' ', "");
    if t.is_empty() { return 4.0; }
    let unit_hours = if t.contains('d') { 8.0 } else if t.contains("min") { 1.0 / 60.0 } else { 1.0 };
    let nums: Vec<f64> = t
        .split(|c: char| !c.is_ascii_digit() && c != '.')
        .filter_map(|p| p.parse::<f64>().ok())
        .collect();
    let raw = match nums.len() {
        0 => 4.0,
        1 => nums[0],
        _ => (nums[0] + nums[1]) / 2.0,
    };
    raw * unit_hours
}
