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

/// v2 — formula 4-tuple columns on `blocks`. Nullable; no row rewrite needed
/// for existing v1 data. Populated later by `register_formula` (Phase 2 PR-2).
/// `execute_batch` runs multi-statement DDL in one shot, so all four ALTERs
/// land atomically inside the migration's transaction.
pub const SCHEMA_V2: &str = "ALTER TABLE blocks ADD COLUMN type_sig_json TEXT;
ALTER TABLE blocks ADD COLUMN effects_json TEXT;
ALTER TABLE blocks ADD COLUMN formula_source TEXT;
ALTER TABLE blocks ADD COLUMN formula_sha TEXT;";

/// v3 — predicates as separate rows (1:N from blocks). One block carries
/// 0..N invariant predicates; `seq` orders them deterministically inside a
/// block. `args_json` holds the predicate-specific payload.
pub const SCHEMA_V3: &str = "CREATE TABLE block_predicates (
    block_id  INTEGER NOT NULL REFERENCES blocks(id),
    seq       INTEGER NOT NULL,
    kind      TEXT NOT NULL,
    args_json TEXT NOT NULL,
    PRIMARY KEY (block_id, seq)
) STRICT;";

/// v4 — declared deps as separate rows (M:N). `dep_dna` is the wire-format
/// DNA of the depended-on block; `dep_kind` distinguishes call-site classes
/// (e.g. \"runtime\", \"build\", \"sidecar\") so the same target can appear
/// under multiple kinds without collision.
pub const SCHEMA_V4: &str = "CREATE TABLE block_deps (
    block_id INTEGER NOT NULL REFERENCES blocks(id),
    dep_dna  TEXT NOT NULL,
    dep_kind TEXT NOT NULL,
    PRIMARY KEY (block_id, dep_dna, dep_kind)
) STRICT;";

/// v5 — cleanup_findings table for the `kei-cleanup --emit-predicates`
/// bridge (Track B). Each row is one finding produced by a cleanup scan;
/// `severity` ∈ {info, warn, error}, `kind` is a stable scanner-id string,
/// `finding_json` is the full structured payload. `workspace_sha` lets a
/// later run reconcile or replace findings from the same workspace state.
pub const SCHEMA_V5: &str = "CREATE TABLE cleanup_findings (
    id INTEGER PRIMARY KEY,
    workspace_sha TEXT NOT NULL,
    timestamp_unix INTEGER NOT NULL,
    severity TEXT NOT NULL,
    kind TEXT NOT NULL,
    finding_json TEXT NOT NULL
) STRICT;
CREATE INDEX idx_cleanup_workspace_sha ON cleanup_findings(workspace_sha);
CREATE INDEX idx_cleanup_severity ON cleanup_findings(severity);";

/// Schema version. Compared against `PRAGMA user_version`. Bumped together
/// with `MIGRATIONS`. Mismatch (DB is newer than this binary) → exit 3.
pub const SCHEMA_VERSION: u32 = 5;

/// Ordered migrations. Index = target version (1-based). Append only.
pub const MIGRATIONS: &[&str] = &[SCHEMA_V1, SCHEMA_V2, SCHEMA_V3, SCHEMA_V4, SCHEMA_V5];

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
