//! FTS5 search over knowledge_units.

use crate::store::Store;
use crate::types::Unit;
use anyhow::Result;
use rusqlite::params;

const SEARCH_SQL: &str =
    "SELECT k.id, k.unit_type, k.title, k.content, k.evidence_grade,
            k.source_path, k.vault_path, k.category, k.created_at, k.updated_at
     FROM fts_knowledge f
     JOIN knowledge_units k ON k.id = f.unit_id
     WHERE fts_knowledge MATCH ?1
     ORDER BY rank LIMIT ?2";

/// Full-text search. Returns matching Units ordered by SQLite FTS5 rank.
pub fn fts_search(store: &Store, query: &str, limit: i64) -> Result<Vec<Unit>> {
    let lim = if limit <= 0 { 20 } else { limit };
    let mut stmt = store.conn().prepare(SEARCH_SQL)?;
    let rows = stmt.query_map(params![query, lim], row_to_unit)?;
    let mut out = Vec::new();
    for row in rows { out.push(row?); }
    Ok(out)
}

fn row_to_unit(r: &rusqlite::Row) -> rusqlite::Result<Unit> {
    Ok(Unit {
        id: r.get(0)?, unit_type: r.get(1)?, title: r.get(2)?,
        content: r.get(3)?, evidence_grade: r.get(4)?, source_path: r.get(5)?,
        vault_path: r.get(6)?, category: r.get(7)?,
        created_at: r.get(8)?, updated_at: r.get(9)?,
    })
}
