//! `rank` verb — PageRank (power iteration, d=0.85, 50 iter) over the
//! schema's `edge_table`. Returns `{ results: [{id, score}, ...] }`
//! sorted by score descending.
//!
//! Dispatches on `schema.edge_key_kind`: `IntegerPair` emits
//! `{id: i64, score: f64}` rows; `TextPair` and `TextPairWithMetadata`
//! emit `{id: String, score}`. For `TextPairWithMetadata` with
//! `has_weight: true` the rank propagation is proportional to edge
//! weight (weighted PageRank); otherwise each edge contributes equally.

use crate::error::VerbError;
use crate::schema::{EdgeKeyKind, EntitySchema};
use rusqlite::Connection;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

const DAMPING: f64 = 0.85;
const ITERATIONS: usize = 50;

/// Adjacency extracted from an integer-keyed edge table:
/// `(nodes, src → [(dst, weight)])`.
type IntegerAdjacency = (Vec<i64>, HashMap<i64, Vec<(i64, f64)>>);

/// Adjacency extracted from a text-keyed edge table:
/// `(nodes, src → [(dst, weight)])`.
type TextAdjacency = (Vec<String>, HashMap<String, Vec<(String, f64)>>);

pub fn run(
    conn: &Connection,
    schema: &EntitySchema,
    _input: Value,
) -> Result<Value, VerbError> {
    if !schema.verb_enabled("rank") {
        return Err(VerbError::VerbDisabled {
            verb: "rank".into(),
            schema: schema.name.into(),
        });
    }
    let edge = schema.edge_table.ok_or_else(|| {
        VerbError::InvalidInput(format!(
            "rank: schema {} has no edge_table configured",
            schema.name
        ))
    })?;
    match schema.edge_key_kind {
        EdgeKeyKind::IntegerPair => rank_integer(conn, edge),
        EdgeKeyKind::TextPair => rank_text(conn, edge, "src_path", "dst_path", false),
        EdgeKeyKind::TextPairWithMetadata {
            from_col,
            to_col,
            has_weight,
            ..
        } => rank_text(conn, edge, from_col, to_col, has_weight),
    }
}

fn rank_integer(conn: &Connection, edge: &str) -> Result<Value, VerbError> {
    let (nodes, out_edges) = collect_integer(conn, edge)?;
    let rank = pagerank(&nodes, &out_edges);
    let mut out: Vec<(i64, f64)> = rank.into_iter().collect();
    out.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let results: Vec<Value> =
        out.into_iter().map(|(id, score)| json!({ "id": id, "score": score })).collect();
    Ok(json!({ "results": results }))
}

fn rank_text(
    conn: &Connection,
    edge: &str,
    from_col: &str,
    to_col: &str,
    with_weight: bool,
) -> Result<Value, VerbError> {
    let (nodes, out_edges) = collect_text(conn, edge, from_col, to_col, with_weight)?;
    let rank = pagerank(&nodes, &out_edges);
    let mut out: Vec<(String, f64)> = rank.into_iter().collect();
    out.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let results: Vec<Value> =
        out.into_iter().map(|(id, score)| json!({ "id": id, "score": score })).collect();
    Ok(json!({ "results": results }))
}

fn collect_integer(
    conn: &Connection,
    edge: &str,
) -> Result<IntegerAdjacency, VerbError> {
    let sql = format!("SELECT from_id, to_id FROM {edge}");
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([], |r| Ok((r.get::<_, i64>(0)?, r.get::<_, i64>(1)?)))?;
    let mut nodes: HashSet<i64> = HashSet::new();
    let mut out_edges: HashMap<i64, Vec<(i64, f64)>> = HashMap::new();
    for row in rows {
        let (src, dst) = row?;
        nodes.insert(src);
        nodes.insert(dst);
        out_edges.entry(src).or_default().push((dst, 1.0));
    }
    Ok((nodes.into_iter().collect(), out_edges))
}

fn collect_text(
    conn: &Connection,
    edge: &str,
    from_col: &str,
    to_col: &str,
    with_weight: bool,
) -> Result<TextAdjacency, VerbError> {
    let sql = if with_weight {
        format!("SELECT {from_col}, {to_col}, weight FROM {edge}")
    } else {
        format!("SELECT {from_col}, {to_col} FROM {edge}")
    };
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([], |r| {
        let src: String = r.get(0)?;
        let dst: String = r.get(1)?;
        let w: f64 = if with_weight { r.get(2)? } else { 1.0 };
        Ok((src, dst, w))
    })?;
    let mut nodes: HashSet<String> = HashSet::new();
    let mut out_edges: HashMap<String, Vec<(String, f64)>> = HashMap::new();
    for row in rows {
        let (src, dst, w) = row?;
        nodes.insert(src.clone());
        nodes.insert(dst.clone());
        out_edges.entry(src).or_default().push((dst, w));
    }
    Ok((nodes.into_iter().collect(), out_edges))
}

/// Generic weighted PageRank — each edge entry is `(target, weight)`.
/// Unit weights reduce exactly to vanilla PageRank.
fn pagerank<K: Eq + Hash + Clone>(
    nodes: &[K],
    out_edges: &HashMap<K, Vec<(K, f64)>>,
) -> HashMap<K, f64> {
    if nodes.is_empty() {
        return HashMap::new();
    }
    let init = 1.0 / nodes.len() as f64;
    let mut rank: HashMap<K, f64> = nodes.iter().map(|n| (n.clone(), init)).collect();
    for _ in 0..ITERATIONS {
        rank = one_iteration(nodes, out_edges, &rank);
    }
    rank
}

fn one_iteration<K: Eq + Hash + Clone>(
    nodes: &[K],
    out_edges: &HashMap<K, Vec<(K, f64)>>,
    prev: &HashMap<K, f64>,
) -> HashMap<K, f64> {
    let n = nodes.len() as f64;
    let base = (1.0 - DAMPING) / n;
    let mut next: HashMap<K, f64> = nodes.iter().map(|k| (k.clone(), base)).collect();
    for (src, dsts) in out_edges {
        if dsts.is_empty() { continue; }
        let total_w: f64 = dsts.iter().map(|(_, w)| *w).sum();
        if total_w <= 0.0 { continue; }
        let src_rank = prev.get(src).copied().unwrap_or(0.0);
        for (dst, w) in dsts {
            let share = DAMPING * src_rank * (w / total_w);
            if let Some(slot) = next.get_mut(dst) {
                *slot += share;
            }
        }
    }
    next
}
