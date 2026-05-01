//! SQLite store — schema + open + migrate.
//!
//! Constructor Pattern: this cube owns the DDL, the schema-version pragma,
//! and `open_db`. CRUD lives in `registry.rs`. Schema changes MUST bump
//! `SCHEMA_VERSION` and append to `MIGRATIONS`; never reorder.

use anyhow::{Context, Result};
use rusqlite::Connection;
use std::path::Path;

/// v1 — initial schema. Tracks one row per (path, body_sha) tuple. The DNA
/// is the UNIQUE wire-format key. `superseded_by` points at a NEWER row's
/// DNA when this row is no longer active. `created` and `modified` are
/// Unix epoch seconds; they bracket the row's life from first registration
/// to its most recent re-touch.
pub const SCHEMA_V1: &str = "CREATE TABLE IF NOT EXISTS blocks (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    dna           TEXT NOT NULL UNIQUE,
    block_type    TEXT NOT NULL,
    name          TEXT NOT NULL,
    path          TEXT NOT NULL,
    caps          TEXT NOT NULL,
    scope_sha     TEXT NOT NULL,
    body_sha      TEXT NOT NULL,
    nonce         TEXT NOT NULL,
    created       INTEGER NOT NULL,
    modified      INTEGER NOT NULL,
    superseded_by TEXT
);
CREATE INDEX IF NOT EXISTS idx_blocks_type ON blocks(block_type);
CREATE INDEX IF NOT EXISTS idx_blocks_path ON blocks(path);
CREATE INDEX IF NOT EXISTS idx_blocks_body ON blocks(body_sha);";

/// Schema version. Compared against `PRAGMA user_version`. Bumped together
/// with `MIGRATIONS`. Mismatch (DB is newer than this binary) → exit 3.
pub const SCHEMA_VERSION: u32 = 1;

/// Ordered migrations. Index = target version (1-based). Append only.
pub const MIGRATIONS: &[&str] = &[SCHEMA_V1];

/// Open or create the SQLite store at `path`. Runs all pending migrations
/// transactionally. Returns the connection ready for CRUD use. Schema
/// version mismatch (DB ahead of binary) returns an Err, NOT a silent
/// downgrade — callers should exit 3.
pub fn open_db<P: AsRef<Path>>(path: P) -> Result<Connection> {
    let conn = Connection::open(&path)
        .with_context(|| format!("open registry sqlite at {}", path.as_ref().display()))?;
    migrate(&conn)?;
    Ok(conn)
}

/// Apply pending migrations atomically — DDL + user_version bump in one
/// transaction per version. Mirrors the kei-ledger schema.rs idiom so a
/// crash mid-migration leaves a consistent file.
pub fn migrate(conn: &Connection) -> Result<()> {
    let current: i64 = conn
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .unwrap_or(0);
    if current as u32 > SCHEMA_VERSION {
        anyhow::bail!(
            "registry schema v{} is newer than binary v{}; upgrade kei-registry",
            current,
            SCHEMA_VERSION
        );
    }
    for (i, sql) in MIGRATIONS.iter().enumerate() {
        let target = (i + 1) as i64;
        if current < target {
            apply_one(conn, sql, target)?;
        }
    }
    Ok(())
}

fn apply_one(conn: &Connection, sql: &str, target: i64) -> Result<()> {
    conn.execute_batch("BEGIN IMMEDIATE")?;
    let step: Result<()> = (|| {
        conn.execute_batch(sql)?;
        conn.pragma_update(None, "user_version", target)?;
        Ok(())
    })();
    match step {
        Ok(()) => {
            conn.execute_batch("COMMIT")?;
            Ok(())
        }
        Err(e) => {
            let _ = conn.execute_batch("ROLLBACK");
            Err(e)
        }
    }
}
