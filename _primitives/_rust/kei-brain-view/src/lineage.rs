//! Ancestor + descendant walk for a single DNA.
//!
//! Constructor Pattern: one cube = `Lineage` struct + BFS descent + parent
//! chain walk. Both walks cycle-safe via `visited` set + `MAX_TREE_DEPTH`.

use crate::error::{BrainViewError, Result, MAX_TREE_DEPTH};
use crate::graph::{resolve_dna, Graph, Node};
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Default, Serialize)]
pub struct Lineage {
    pub focus: Option<Node>,
    pub ancestors: Vec<Node>,
    pub descendants: Vec<Node>,
}

/// Resolve a DNA prefix and return its ancestors + descendants.
/// `MAX_TREE_DEPTH` bounds both walks to fail fast on cycles.
pub fn lineage(graph: &Graph, dna_prefix: &str) -> Result<Lineage> {
    let focus = resolve_dna(graph, dna_prefix)?.clone();
    let ancestors = collect_ancestors(graph, &focus)?;
    let descendants = collect_descendants(graph, &focus)?;
    Ok(Lineage {
        focus: Some(focus),
        ancestors,
        descendants,
    })
}

fn collect_ancestors(graph: &Graph, focus: &Node) -> Result<Vec<Node>> {
    let mut out = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    let mut cur = focus.parent_branch.clone();
    let mut steps = 0usize;
    while let Some(pb) = cur {
        steps += 1;
        if steps > MAX_TREE_DEPTH {
            return Err(BrainViewError::MaxDepthExceeded(MAX_TREE_DEPTH));
        }
        if !seen.insert(pb.clone()) {
            break;
        }
        let Some(&idx) = graph.by_branch.get(&pb) else {
            break;
        };
        let n = graph.node(idx).clone();
        cur = n.parent_branch.clone();
        out.push(n);
    }
    out.reverse();
    Ok(out)
}

fn collect_descendants(graph: &Graph, focus: &Node) -> Result<Vec<Node>> {
    let mut out = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    seen.insert(focus.branch.clone());
    let mut frontier: Vec<String> = vec![focus.branch.clone()];
    let mut steps = 0usize;
    while let Some(parent_branch) = frontier.pop() {
        steps += 1;
        if steps > MAX_TREE_DEPTH {
            return Err(BrainViewError::MaxDepthExceeded(MAX_TREE_DEPTH));
        }
        let Some(kids) = graph.children_of.get(&parent_branch) else {
            continue;
        };
        for &k in kids {
            let n = graph.node(k);
            if seen.insert(n.branch.clone()) {
                frontier.push(n.branch.clone());
                out.push(n.clone());
            }
        }
    }
    Ok(out)
}
