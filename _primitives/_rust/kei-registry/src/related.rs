//! Related-block discovery via naive substring match.
//!
//! Constructor Pattern: this cube owns the dependency-graph approximation
//! ONLY. It re-reads body bytes from disk for each candidate (cheap on a
//! few-thousand-block kit) and looks for the root block's name as a
//! substring. Depth > 1 unrolls iteratively over the active set.
//!
//! Limitations: substring match is intentionally lossy — false positives
//! (one block's name appears in unrelated prose) cost the user nothing
//! beyond an extra row in JSON output. False negatives (a block referred
//! to by alias) are an open extension, not a bug.

use anyhow::Result;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;

use crate::block::Block;
use crate::registry::list;

/// One related-hit row: the block plus its BFS distance from the root.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedHit {
    pub block: Block,
    pub distance: u32,
}

/// Find blocks whose body references `root` by name. BFS bounded by `depth`.
/// Returns hits in non-decreasing distance order. The root itself is NOT
/// included in the output.
pub fn find_related(conn: &Connection, root: &Block, depth: u32) -> Result<Vec<RelatedHit>> {
    if depth == 0 {
        return Ok(Vec::new());
    }
    let mut visited: HashSet<i64> = HashSet::new();
    visited.insert(root.id);
    let mut frontier: Vec<Block> = vec![root.clone()];
    let mut hits: Vec<RelatedHit> = Vec::new();
    let universe = list(conn, false, i64::MAX)?;
    for d in 1..=depth {
        let next = expand_frontier(&frontier, &universe, &mut visited)?;
        for block in &next {
            hits.push(RelatedHit {
                block: block.clone(),
                distance: d,
            });
        }
        if next.is_empty() {
            break;
        }
        frontier = next;
    }
    Ok(hits)
}

/// One BFS step. Read each frontier block's body and find blocks in
/// `universe` whose name appears in it. Skip already-visited rows.
fn expand_frontier(
    frontier: &[Block],
    universe: &[Block],
    visited: &mut HashSet<i64>,
) -> Result<Vec<Block>> {
    let mut next: Vec<Block> = Vec::new();
    for src in frontier {
        let body = match fs::read(&src.path) {
            Ok(b) => b,
            Err(_) => continue,
        };
        let body_text = String::from_utf8_lossy(&body);
        for cand in universe {
            if visited.contains(&cand.id) || cand.id == src.id {
                continue;
            }
            if body_text.contains(&cand.name) {
                visited.insert(cand.id);
                next.push(cand.clone());
            }
        }
    }
    Ok(next)
}
