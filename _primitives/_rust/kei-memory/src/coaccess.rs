//! Co-access tracking — files touched within a 5-minute window.
//!
//! Constructor Pattern: one cube, single responsibility.
//! Derived from an in-house implementation, algorithmic spec documented in coaccess.md.
//!
//! Session_id IS used to scope the window query (avoiding cross-session
//! false co-access — we never pair file_a from session X with file_b
//! from session Y), but it isn't part of the coaccess row primary key
//! (the PK is the canonical file pair). This means a file pair seen in
//! 5 sessions has 1 row, not 5 — counts aggregate across sessions so
//! cross-session recurrences surface in `patterns`.

use rusqlite::{params, Connection, Result};

const WINDOW_SECS: i64 = 300;

/// Insert (or increment) pair entries for the new file vs any other file
/// touched in the same session within the last 5 minutes. Pair ordering
/// is canonicalised lexically so (A,B) and (B,A) collapse to one row.
pub fn record_coaccess(
    conn: &Connection,
    session_id: &str,
    file_path: &str,
    ts: i64,
) -> Result<()> {
    let recent = recent_files_in_window(conn, session_id, file_path, ts)?;
    for other in recent {
        let (a, b) = canonical_pair(file_path, &other);
        conn.execute(
            "INSERT INTO coaccess (file_a, file_b, count) VALUES (?1, ?2, 1)
             ON CONFLICT(file_a, file_b) DO UPDATE SET count = count + 1",
            params![a, b],
        )?;
    }
    Ok(())
}

fn canonical_pair<'a>(x: &'a str, y: &'a str) -> (&'a str, &'a str) {
    if x < y {
        (x, y)
    } else {
        (y, x)
    }
}

fn recent_files_in_window(
    conn: &Connection,
    session_id: &str,
    exclude: &str,
    ts: i64,
) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT DISTINCT file_path FROM events
         WHERE session_id = ?1
           AND file_path IS NOT NULL
           AND file_path != ?2
           AND ts >= ?3
         ORDER BY ts DESC LIMIT 10",
    )?;
    let rows = stmt
        .query_map(params![session_id, exclude, ts - WINDOW_SECS], |r| {
            r.get::<_, String>(0)
        })?
        .collect::<Result<Vec<_>>>()?;
    Ok(rows)
}

/// Return (file_a, file_b, count) triples ordered by co-access count DESC.
/// Not yet exposed on the CLI — used by integration tests and reserved
/// for the upcoming `kei-memory coaccess --top` subcommand.
#[allow(dead_code)]
pub fn top_pairs(conn: &Connection, limit: usize) -> Result<Vec<(String, String, i64)>> {
    let mut stmt = conn.prepare(
        "SELECT file_a, file_b, count FROM coaccess
         ORDER BY count DESC LIMIT ?1",
    )?;
    let rows = stmt
        .query_map(params![limit as i64], |r| {
            Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?, r.get::<_, i64>(2)?))
        })?
        .collect::<Result<Vec<_>>>()?;
    Ok(rows)
}
