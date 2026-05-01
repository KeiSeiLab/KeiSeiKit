use crate::store::Store;
use anyhow::Result;
use rusqlite::params;
use serde::Serialize;
use std::collections::{HashSet, VecDeque};

const MAX_DEPTH: i64 = 5;

#[derive(Debug, Clone, Serialize)]
pub struct Reached {
    pub uri: String,
    pub edge_type: String,
    pub depth: i64,
}

pub fn bfs(store: &Store, start: &str, depth: i64) -> Result<Vec<Reached>> {
    let d = clamp(depth);
    let mut seen: HashSet<String> = HashSet::new();
    seen.insert(start.into());
    let mut q: VecDeque<(String, i64)> = VecDeque::new();
    q.push_back((start.into(), 0));
    let mut out = Vec::new();
    while let Some((uri, cur)) = q.pop_front() {
        if cur >= d { continue; }
        let mut stmt = store.conn().prepare(
            "SELECT to_uri, edge_type FROM cross_edges WHERE from_uri=?1"
        )?;
        let rows = stmt.query_map(params![uri], |r| {
            Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))
        })?;
        for row in rows {
            let (to, et) = row?;
            if seen.contains(&to) { continue; }
            seen.insert(to.clone());
            out.push(Reached { uri: to.clone(), edge_type: et, depth: cur + 1 });
            q.push_back((to, cur + 1));
        }
    }
    Ok(out)
}

fn clamp(d: i64) -> i64 {
    if d <= 0 { 2 } else if d > MAX_DEPTH { MAX_DEPTH } else { d }
}
