//! BFS traversal over the edges table, depth-limited, deduplicated.

use crate::edges::list_outgoing;
use crate::store::Store;
use crate::types::Related;
use anyhow::Result;
use std::collections::{HashSet, VecDeque};

const MAX_RESULTS: usize = 500;
const MAX_DEPTH: i64 = 5;

pub fn bfs(store: &Store, start: &str, max_depth: i64) -> Result<Vec<Related>> {
    let depth = clamp_depth(max_depth);
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(start.to_string());
    let mut queue: VecDeque<(String, i64)> = VecDeque::new();
    queue.push_back((start.to_string(), 0));
    let mut out: Vec<Related> = Vec::new();
    while let Some((path, d)) = queue.pop_front() {
        if out.len() >= MAX_RESULTS {
            break;
        }
        if d >= depth {
            continue;
        }
        for e in list_outgoing(store, &path)? {
            if visited.contains(&e.dst_path) || out.len() >= MAX_RESULTS {
                continue;
            }
            visited.insert(e.dst_path.clone());
            out.push(Related {
                path: e.dst_path.clone(),
                edge_type: e.edge_type,
                depth: d + 1,
            });
            queue.push_back((e.dst_path, d + 1));
        }
    }
    Ok(out)
}

fn clamp_depth(d: i64) -> i64 {
    if d <= 0 {
        2
    } else if d > MAX_DEPTH {
        MAX_DEPTH
    } else {
        d
    }
}
