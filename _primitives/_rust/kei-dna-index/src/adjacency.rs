//! Adjacency queries over DNAs.
//!
//! Constructor Pattern: one file = one responsibility (adjacency kinds).

use crate::db::{find_target, load_rows, Row};
use crate::error::{Error, Result};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdjacencyKind {
    Scope,
    Body,
    Role,
    Temporal,
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Relationship {
    SameScope,
    SameBody,
    SameRoleCaps,
    TemporalNeighbor,
    Cluster,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdjacencyResult {
    pub dna: String,
    pub agent_id: String,
    pub status: String,
    pub distance: u32,
    pub relationship: Relationship,
}

pub fn adjacent(
    conn: &Connection,
    target_dna: &str,
    kind: AdjacencyKind,
    limit: usize,
) -> Result<Vec<AdjacencyResult>> {
    let rows = load_rows(conn)?;
    let target = find_target(&rows, target_dna)
        .ok_or_else(|| Error::TargetNotFound(target_dna.to_string()))?
        .clone();
    let results = match kind {
        AdjacencyKind::Scope => same_scope(&rows, &target),
        AdjacencyKind::Body => same_body(&rows, &target),
        AdjacencyKind::Role => same_role_caps(&rows, &target),
        AdjacencyKind::Temporal => temporal(&rows, &target),
        AdjacencyKind::All => all_union(&rows, &target),
    };
    Ok(truncate(results, limit))
}

fn same_scope(rows: &[Row], target: &Row) -> Vec<AdjacencyResult> {
    rows.iter()
        .filter(|r| r.dna != target.dna)
        .filter(|r| r.parsed.scope_sha == target.parsed.scope_sha)
        .map(|r| make(r, 0, Relationship::SameScope))
        .collect()
}

fn same_body(rows: &[Row], target: &Row) -> Vec<AdjacencyResult> {
    rows.iter()
        .filter(|r| r.dna != target.dna)
        .filter(|r| r.parsed.body_sha == target.parsed.body_sha)
        .map(|r| make(r, 0, Relationship::SameBody))
        .collect()
}

fn same_role_caps(rows: &[Row], target: &Row) -> Vec<AdjacencyResult> {
    let mut out: Vec<AdjacencyResult> = rows
        .iter()
        .filter(|r| r.dna != target.dna)
        .filter(|r| r.parsed.role == target.parsed.role)
        .map(|r| {
            let d = hamming(&r.parsed.caps, &target.parsed.caps);
            make(r, d, Relationship::SameRoleCaps)
        })
        .collect();
    out.sort_by_key(|r| r.distance);
    out
}

fn temporal(rows: &[Row], target: &Row) -> Vec<AdjacencyResult> {
    let mut out: Vec<AdjacencyResult> = rows
        .iter()
        .filter(|r| r.dna != target.dna)
        .map(|r| {
            let d = (r.started_ts - target.started_ts).unsigned_abs() as u32;
            make(r, d, Relationship::TemporalNeighbor)
        })
        .collect();
    out.sort_by_key(|r| r.distance);
    out
}

fn all_union(rows: &[Row], target: &Row) -> Vec<AdjacencyResult> {
    let mut bag: Vec<AdjacencyResult> = Vec::new();
    bag.extend(same_scope(rows, target));
    bag.extend(same_body(rows, target));
    bag.extend(same_role_caps(rows, target));
    bag.extend(temporal(rows, target));
    dedup_min_distance(bag)
}

fn dedup_min_distance(bag: Vec<AdjacencyResult>) -> Vec<AdjacencyResult> {
    let mut seen: std::collections::HashMap<String, AdjacencyResult> =
        std::collections::HashMap::new();
    for r in bag {
        seen.entry(r.dna.clone())
            .and_modify(|cur| {
                if r.distance < cur.distance {
                    *cur = r.clone();
                }
            })
            .or_insert(r);
    }
    let mut out: Vec<AdjacencyResult> = seen.into_values().collect();
    out.sort_by_key(|r| r.distance);
    out
}

fn make(r: &Row, distance: u32, relationship: Relationship) -> AdjacencyResult {
    AdjacencyResult {
        dna: r.dna.clone(),
        agent_id: r.agent_id.clone(),
        status: r.status.clone(),
        distance,
        relationship,
    }
}

fn truncate(mut v: Vec<AdjacencyResult>, limit: usize) -> Vec<AdjacencyResult> {
    if limit > 0 && v.len() > limit {
        v.truncate(limit);
    }
    v
}

/// Hamming distance over ASCII bytes; differing lengths count extra bytes.
pub(crate) fn hamming(a: &str, b: &str) -> u32 {
    let ab = a.as_bytes();
    let bb = b.as_bytes();
    let n = ab.len().min(bb.len());
    let mut d: u32 = 0;
    for i in 0..n {
        if ab[i] != bb[i] {
            d += 1;
        }
    }
    d + (ab.len().abs_diff(bb.len()) as u32)
}
