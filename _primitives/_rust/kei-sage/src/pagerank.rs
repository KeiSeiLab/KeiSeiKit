//! PageRank — power-iteration, 50 iterations, d=0.85. Operates on the edges table.

use crate::store::Store;
use anyhow::Result;
use std::collections::HashMap;

const DAMPING: f64 = 0.85;
const ITERATIONS: usize = 50;

/// Compute PageRank over the edges table. Returns [(path, score)] sorted desc.
pub fn pagerank(store: &Store) -> Result<Vec<(String, f64)>> {
    let (nodes, out_edges) = collect_graph(store)?;
    if nodes.is_empty() {
        return Ok(Vec::new());
    }
    let mut rank: HashMap<String, f64> = nodes.iter()
        .map(|n| (n.clone(), 1.0 / nodes.len() as f64)).collect();
    for _ in 0..ITERATIONS {
        rank = one_iteration(&nodes, &out_edges, &rank);
    }
    let mut out: Vec<(String, f64)> = rank.into_iter().collect();
    out.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    Ok(out)
}

fn collect_graph(store: &Store) -> Result<(Vec<String>, HashMap<String, Vec<String>>)> {
    let mut stmt = store.conn().prepare("SELECT src_path, dst_path FROM edges")?;
    let rows = stmt.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))?;
    let mut nodes: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut out_edges: HashMap<String, Vec<String>> = HashMap::new();
    for row in rows {
        let (src, dst) = row?;
        nodes.insert(src.clone());
        nodes.insert(dst.clone());
        out_edges.entry(src).or_default().push(dst);
    }
    Ok((nodes.into_iter().collect(), out_edges))
}

fn one_iteration(
    nodes: &[String],
    out_edges: &HashMap<String, Vec<String>>,
    prev: &HashMap<String, f64>,
) -> HashMap<String, f64> {
    let n = nodes.len() as f64;
    let base = (1.0 - DAMPING) / n;
    let mut next: HashMap<String, f64> = nodes.iter().map(|k| (k.clone(), base)).collect();
    for (src, dsts) in out_edges {
        if dsts.is_empty() {
            continue;
        }
        let share = DAMPING * prev.get(src).copied().unwrap_or(0.0) / dsts.len() as f64;
        for dst in dsts {
            if let Some(slot) = next.get_mut(dst) {
                *slot += share;
            }
        }
    }
    next
}
