use crate::store::Store;
use crate::types::CrossEdge;
use anyhow::Result;
use chrono::Utc;
use rusqlite::params;

pub fn link(store: &Store, from: &str, to: &str, edge_type: &str,
            weight: f64, evidence: &str) -> Result<i64> {
    let now = Utc::now().timestamp();
    store.conn().execute(
        "INSERT OR IGNORE INTO cross_edges (from_uri, to_uri, edge_type, weight, evidence, created_at)
         VALUES (?1,?2,?3,?4,?5,?6)",
        params![from, to, edge_type, weight, evidence, now],
    )?;
    Ok(store.conn().last_insert_rowid())
}

pub fn unlink(store: &Store, from: &str, to: &str, edge_type: &str) -> Result<usize> {
    let n = store.conn().execute(
        "DELETE FROM cross_edges WHERE from_uri=?1 AND to_uri=?2 AND edge_type=?3",
        params![from, to, edge_type],
    )?;
    Ok(n)
}

pub fn query_edges(store: &Store, uri: &str) -> Result<Vec<CrossEdge>> {
    let mut stmt = store.conn().prepare(
        "SELECT edge_id, from_uri, to_uri, edge_type, weight, evidence, metadata, created_at
         FROM cross_edges WHERE from_uri=?1 OR to_uri=?1",
    )?;
    let rows = stmt.query_map(params![uri], |r| {
        Ok(CrossEdge {
            id: r.get(0)?, from_uri: r.get(1)?, to_uri: r.get(2)?,
            edge_type: r.get(3)?, weight: r.get(4)?, evidence: r.get(5)?,
            metadata: r.get(6)?, created_at: r.get(7)?,
        })
    })?;
    let mut out = Vec::new();
    for r in rows { out.push(r?); }
    Ok(out)
}

pub fn count_by_type(store: &Store) -> Result<Vec<(String, i64)>> {
    let mut stmt = store.conn().prepare(
        "SELECT edge_type, COUNT(*) FROM cross_edges GROUP BY edge_type",
    )?;
    let rows = stmt.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?)))?;
    let mut out = Vec::new();
    for r in rows { out.push(r?); }
    Ok(out)
}
