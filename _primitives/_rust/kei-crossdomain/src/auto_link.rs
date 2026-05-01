//! Auto-link heuristic — proposes edges based on URI-name component matching.
//! No-ML: intersect the last path segments (case-insensitive, normalized).

use crate::edges::link;
use crate::store::Store;
use crate::types::extract_domain;
use anyhow::Result;
use rusqlite::params;

/// Scan cross_edges for entities referenced from `uri` domain and propose
/// new edges to entities in other domains that share a trailing name token.
pub fn auto_link(store: &Store, uri: &str) -> Result<usize> {
    let tail = tail_token(uri);
    if tail.is_empty() {
        return Ok(0);
    }
    let src_domain = extract_domain(uri);
    let candidates = collect_candidates(store, uri, src_domain, &tail)?;
    commit_candidates(store, uri, &candidates)
}

fn collect_candidates(store: &Store, uri: &str, src_domain: &str, tail: &str)
    -> Result<Vec<String>>
{
    let mut candidates: Vec<String> = Vec::new();
    let mut stmt = store.conn().prepare(
        "SELECT DISTINCT to_uri FROM cross_edges
         UNION SELECT DISTINCT from_uri FROM cross_edges",
    )?;
    let rows = stmt.query_map([], |r| r.get::<_, String>(0))?;
    for row in rows {
        let u = row?;
        if u == uri || extract_domain(&u) == src_domain { continue; }
        if tail_token(&u).eq_ignore_ascii_case(tail) {
            candidates.push(u);
        }
    }
    Ok(candidates)
}

fn commit_candidates(store: &Store, uri: &str, candidates: &[String]) -> Result<usize> {
    let mut added = 0;
    for c in candidates {
        if edge_exists(store, uri, c)? { continue; }
        link(store, uri, c, "auto_related", 0.5, "E5")?;
        added += 1;
    }
    Ok(added)
}

fn edge_exists(store: &Store, from: &str, to: &str) -> Result<bool> {
    let n: i64 = store.conn().query_row(
        "SELECT COUNT(*) FROM cross_edges WHERE from_uri=?1 AND to_uri=?2",
        params![from, to], |r| r.get(0))?;
    Ok(n > 0)
}

fn tail_token(uri: &str) -> String {
    uri.rsplit('/').next().unwrap_or("").to_lowercase()
}
