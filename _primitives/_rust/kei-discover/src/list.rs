//! `list_available` — return entries that have NOT been installed.
//!
//! Runs a direct `SELECT` rather than `kei_entity_store::verbs::list`
//! because the generic verb does not filter by column; we need
//! `WHERE installed = 0` to hide already-installed entries. Ordering is
//! `id DESC` for consistency with the engine's `list` convention.

use crate::entry::Entry;
use crate::error::DiscoverError;
use rusqlite::Connection;

const SQL: &str = "\
    SELECT id, slug, author, source_url, description, \
           installed, last_seen_ts, created_at, updated_at \
    FROM discover_index \
    WHERE installed = 0 \
    ORDER BY id DESC";

pub fn list_available(conn: &Connection) -> Result<Vec<Entry>, DiscoverError> {
    let mut stmt = conn.prepare(SQL)?;
    let rows = stmt.query_map([], row_to_entry)?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

fn row_to_entry(r: &rusqlite::Row<'_>) -> rusqlite::Result<Entry> {
    Ok(Entry {
        id: r.get(0)?,
        slug: r.get(1)?,
        author: r.get(2)?,
        source_url: r.get::<_, Option<String>>(3)?.unwrap_or_default(),
        description: r.get::<_, Option<String>>(4)?.unwrap_or_default(),
        installed: r.get::<_, i64>(5)? != 0,
        last_seen_ts: r.get::<_, Option<i64>>(6)?.unwrap_or(0),
        created_at: r.get(7)?,
        updated_at: r.get(8)?,
    })
}
