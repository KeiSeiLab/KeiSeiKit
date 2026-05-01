//! Cumulative research-graph merger.
//!
//! Walks a research root, picks up each per-topic `graph.json`, merges into
//! a single cumulative graph file. Schema is intentionally permissive — we
//! treat each graph.json as `{ "nodes": [...], "edges": [...] }` and append
//! to a master accumulator, dedup'ing by `id` for nodes and (`from`, `to`)
//! for edges.

use anyhow::{Context, Result};
use serde::Serialize;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize)]
pub struct GraphMergeOutput {
    pub graphs_read: usize,
    pub nodes_total: usize,
    pub edges_total: usize,
    pub out_path: PathBuf,
}

/// Walk `research_dir` for `graph.json` siblings of `MASTER-REPORT.md`,
/// merge into the master at `out`. Existing master is loaded and additive.
pub fn merge_graphs(research_dir: &Path, out: &Path) -> Result<GraphMergeOutput> {
    let mut master = load_master_or_empty(out);
    let mut graphs_read = 0usize;
    for entry in WalkDir::new(research_dir).max_depth(4).into_iter().flatten() {
        let p = entry.path();
        if p.file_name().map(|n| n == "graph.json").unwrap_or(false) {
            if let Ok(g) = load_graph(p) {
                merge_into(&mut master, g);
                graphs_read += 1;
            }
        }
    }
    let json = serde_json::to_string_pretty(&master).context("serialize master graph")?;
    if let Some(parent) = out.parent() {
        std::fs::create_dir_all(parent).with_context(|| format!("mkdir {}", parent.display()))?;
    }
    std::fs::write(out, &json).with_context(|| format!("write {}", out.display()))?;
    let nodes_total = master.get("nodes").and_then(|n| n.as_array()).map(|a| a.len()).unwrap_or(0);
    let edges_total = master.get("edges").and_then(|n| n.as_array()).map(|a| a.len()).unwrap_or(0);
    Ok(GraphMergeOutput { graphs_read, nodes_total, edges_total, out_path: out.to_path_buf() })
}

fn load_master_or_empty(out: &Path) -> serde_json::Value {
    if out.exists() {
        if let Ok(s) = std::fs::read_to_string(out) {
            if let Ok(v) = serde_json::from_str(&s) {
                return v;
            }
        }
    }
    serde_json::json!({ "nodes": [], "edges": [] })
}

fn load_graph(p: &Path) -> Result<serde_json::Value> {
    let s = std::fs::read_to_string(p).with_context(|| format!("read {}", p.display()))?;
    let v = serde_json::from_str(&s).with_context(|| format!("parse {}", p.display()))?;
    Ok(v)
}

fn merge_into(master: &mut serde_json::Value, source: serde_json::Value) {
    merge_nodes(master, &source);
    merge_edges(master, &source);
}

fn merge_nodes(master: &mut serde_json::Value, source: &serde_json::Value) {
    let Some(src_nodes) = source.get("nodes").and_then(|n| n.as_array()) else { return };
    let dst_nodes = master.get_mut("nodes").and_then(|n| n.as_array_mut());
    let Some(dst) = dst_nodes else { return };
    let seen: HashSet<String> = dst.iter()
        .filter_map(|n| n.get("id").and_then(|x| x.as_str()).map(String::from))
        .collect();
    for node in src_nodes {
        let id = node.get("id").and_then(|x| x.as_str()).unwrap_or("");
        if id.is_empty() || !seen.contains(id) {
            dst.push(node.clone());
        }
    }
}

fn merge_edges(master: &mut serde_json::Value, source: &serde_json::Value) {
    let Some(src_edges) = source.get("edges").and_then(|n| n.as_array()) else { return };
    let dst_edges = master.get_mut("edges").and_then(|n| n.as_array_mut());
    let Some(dst) = dst_edges else { return };
    let seen: HashSet<(String, String)> = dst.iter().filter_map(edge_key).collect();
    for edge in src_edges {
        if let Some(k) = edge_key(edge) {
            if !seen.contains(&k) {
                dst.push(edge.clone());
            }
        }
    }
}

fn edge_key(edge: &serde_json::Value) -> Option<(String, String)> {
    let from = edge.get("from").and_then(|x| x.as_str())?.to_string();
    let to = edge.get("to").and_then(|x| x.as_str())?.to_string();
    Some((from, to))
}
