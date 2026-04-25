//! Read-only SQLite access to the kei-ledger agents table.
//!
//! Constructor Pattern: one file = one responsibility (DB row loading).

use crate::error::Result;
use crate::parsed::{split_dna, ParsedDna};
use rusqlite::{Connection, OpenFlags};
use std::path::Path;

/// One row of the `agents` table, with its DNA already parsed.
/// Rows where `dna IS NULL` or parse-failed are excluded at load time.
#[derive(Debug, Clone)]
pub struct Row {
    pub agent_id: String,
    pub dna: String,
    pub parsed: ParsedDna,
    pub started_ts: i64,
    pub status: String,
}

/// Open ledger in read-only mode. No schema mutation.
pub fn open_read_only<P: AsRef<Path>>(path: P) -> Result<Connection> {
    let conn = Connection::open_with_flags(
        path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_URI,
    )?;
    Ok(conn)
}

/// Load all rows with non-null DNA. Malformed DNAs are skipped silently.
pub fn load_rows(conn: &Connection) -> Result<Vec<Row>> {
    let mut stmt = conn.prepare(
        "SELECT id, dna, started_ts, COALESCE(status,'unknown') \
         FROM agents WHERE dna IS NOT NULL",
    )?;
    let iter = stmt.query_map([], |r| {
        let id: String = r.get(0)?;
        let dna: String = r.get(1)?;
        let ts: i64 = r.get(2)?;
        let status: String = r.get(3)?;
        Ok((id, dna, ts, status))
    })?;

    let mut rows: Vec<Row> = Vec::new();
    for rec in iter {
        let (agent_id, dna, started_ts, status) = rec?;
        if let Ok(parsed) = split_dna(&dna) {
            rows.push(Row {
                agent_id,
                dna,
                parsed,
                started_ts,
                status,
            });
        }
    }
    Ok(rows)
}

/// Find the row matching a given DNA string exactly.
pub fn find_target<'a>(rows: &'a [Row], target_dna: &str) -> Option<&'a Row> {
    rows.iter().find(|r| r.dna == target_dna)
}
