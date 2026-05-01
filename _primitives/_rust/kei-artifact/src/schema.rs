//! SQL schema DDL + migrations for the artifact store.
//!
//! Two tables:
//! - `schemas`   — registered JSON Schemas by name (SSoT for validation).
//! - `artifacts` — typed content + metadata + parent pointer for handoff chain.

use rusqlite::{Connection, Result};

/// Ordered migrations. Index = schema version. Append only; never reorder.
pub const MIGRATIONS: &[&str] = &[
    // v1 — initial schema
    "CREATE TABLE IF NOT EXISTS schemas (
        name            TEXT PRIMARY KEY,
        json_schema     TEXT NOT NULL,
        registered_at   INTEGER NOT NULL
     );
     CREATE TABLE IF NOT EXISTS artifacts (
        id                   TEXT PRIMARY KEY,
        schema_name          TEXT NOT NULL,
        source_agent         TEXT NOT NULL,
        content              BLOB NOT NULL,
        meta_json            TEXT,
        parent_artifact_id   TEXT,
        created_at           INTEGER NOT NULL,
        FOREIGN KEY (schema_name) REFERENCES schemas(name),
        FOREIGN KEY (parent_artifact_id) REFERENCES artifacts(id)
     );
     CREATE INDEX IF NOT EXISTS idx_schema  ON artifacts(schema_name);
     CREATE INDEX IF NOT EXISTS idx_source  ON artifacts(source_agent);
     CREATE INDEX IF NOT EXISTS idx_created ON artifacts(created_at);",
];

/// Apply pending migrations. Uses pragma `user_version` as the version cursor.
pub fn migrate(conn: &Connection) -> Result<()> {
    let current: i64 = conn
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .unwrap_or(0);
    for (i, sql) in MIGRATIONS.iter().enumerate() {
        let target = (i + 1) as i64;
        if current < target {
            conn.execute_batch(sql)?;
            conn.pragma_update(None, "user_version", target)?;
        }
    }
    Ok(())
}

/// Canonical list of artifact schema names shipped with this primitive.
/// Also the whitelist the _assembler validator checks.
pub const KNOWN_SCHEMAS: &[&str] = &["spec", "plan", "patch", "review", "research"];
