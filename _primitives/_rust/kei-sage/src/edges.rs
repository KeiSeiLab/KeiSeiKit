//! Typed-edge CRUD between vault_paths.

use crate::store::Store;
use crate::types::Edge;
use anyhow::Result;
use chrono::Utc;
use rusqlite::params;

pub fn add_edge(store: &Store, src: &str, dst: &str, edge_type: &str, weight: f64) -> Result<i64> {
    let now = Utc::now().timestamp();
    store.conn().execute(
        "INSERT OR IGNORE INTO edges (src_path, dst_path, edge_type, weight, created_at)
         VALUES (?1,?2,?3,?4,?5)",
        params![src, dst, edge_type, weight, now],
    )?;
    Ok(store.conn().last_insert_rowid())
}

pub fn remove_edge(store: &Store, src: &str, dst: &str, edge_type: &str) -> Result<usize> {
    let n = store.conn().execute(
        "DELETE FROM edges WHERE src_path=?1 AND dst_path=?2 AND edge_type=?3",
        params![src, dst, edge_type],
    )?;
    Ok(n)
}

pub fn list_outgoing(store: &Store, src: &str) -> Result<Vec<Edge>> {
    let mut stmt = store.conn().prepare(
        "SELECT id, src_path, dst_path, edge_type, weight, created_at
         FROM edges WHERE src_path=?1",
    )?;
    let rows = stmt.query_map(params![src], row_to_edge)?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

pub fn list_incoming(store: &Store, dst: &str) -> Result<Vec<Edge>> {
    let mut stmt = store.conn().prepare(
        "SELECT id, src_path, dst_path, edge_type, weight, created_at
         FROM edges WHERE dst_path=?1",
    )?;
    let rows = stmt.query_map(params![dst], row_to_edge)?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

fn row_to_edge(r: &rusqlite::Row) -> rusqlite::Result<Edge> {
    Ok(Edge {
        id: r.get(0)?,
        src_path: r.get(1)?,
        dst_path: r.get(2)?,
        edge_type: r.get(3)?,
        weight: r.get(4)?,
        created_at: r.get(5)?,
    })
}
