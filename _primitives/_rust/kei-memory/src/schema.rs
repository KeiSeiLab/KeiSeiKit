//! SQL schema for the kei-memory offline analyzer.
//!
//! Constructor Pattern: schema + migration runner, no business logic.
//! DB default path: `~/.claude/memory/kei-memory.sqlite`.
//! Any structural change MUST append a new migration; never edit history.

use rusqlite::{Connection, Result};

/// Ordered migrations. Index = schema version. Never reorder.
pub const MIGRATIONS: &[&str] = &[
    // v1 — initial schema (RULE 0.14, 2026-04-22)
    "CREATE TABLE IF NOT EXISTS sessions (
        id TEXT PRIMARY KEY,
        started_ts INTEGER NOT NULL,
        ended_ts INTEGER,
        tool_call_count INTEGER NOT NULL DEFAULT 0,
        error_count INTEGER NOT NULL DEFAULT 0
    );
    CREATE TABLE IF NOT EXISTS events (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        session_id TEXT NOT NULL,
        ts INTEGER NOT NULL,
        kind TEXT NOT NULL,
        tool TEXT,
        file_path TEXT,
        is_error INTEGER NOT NULL DEFAULT 0,
        event_class TEXT,
        message TEXT,
        FOREIGN KEY(session_id) REFERENCES sessions(id)
    );
    CREATE INDEX IF NOT EXISTS idx_events_session ON events(session_id);
    CREATE INDEX IF NOT EXISTS idx_events_class ON events(event_class);
    CREATE TABLE IF NOT EXISTS coaccess (
        file_a TEXT NOT NULL,
        file_b TEXT NOT NULL,
        count INTEGER NOT NULL DEFAULT 1,
        PRIMARY KEY(file_a, file_b)
    );
    CREATE TABLE IF NOT EXISTS tokens (
        session_id TEXT NOT NULL,
        token TEXT NOT NULL,
        tf INTEGER NOT NULL,
        PRIMARY KEY(session_id, token)
    );
    CREATE TABLE IF NOT EXISTS idf (
        token TEXT PRIMARY KEY,
        df INTEGER NOT NULL,
        idf REAL NOT NULL
    );
    CREATE TABLE IF NOT EXISTS patterns (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        event_class TEXT NOT NULL,
        session_id TEXT NOT NULL,
        count INTEGER NOT NULL,
        first_seen_ts INTEGER NOT NULL,
        last_seen_ts INTEGER NOT NULL
    );
    CREATE INDEX IF NOT EXISTS idx_patterns_class ON patterns(event_class);
    CREATE TABLE IF NOT EXISTS backlog (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        ts INTEGER NOT NULL,
        item TEXT NOT NULL,
        processed INTEGER NOT NULL DEFAULT 0
    );",
    // v2 — TF-IDF dedup: mark token rows that need IDF recomputation
    // (RULE 0.16 / Wave C, 2026-05-01). Default 1 so existing rows force
    // a one-time recompute on first stale-check after upgrade.
    "ALTER TABLE tokens ADD COLUMN idf_dirty INTEGER NOT NULL DEFAULT 1;",
    // v3 — Wave A schema fix (2026-05-01):
    //   * `events.cwd` — pulled from real Claude Code trace `cwd` field.
    //     Lets retrospectives bucket by working directory.
    //   * Hot-query indices on tool / file_path / ts.
    //   * UNIQUE index on patterns(event_class, COALESCE(session_id,''))
    //     enables the UPSERT planned for Wave D pattern persistence.
    "ALTER TABLE events ADD COLUMN cwd TEXT;
    CREATE INDEX IF NOT EXISTS idx_events_tool ON events(tool) WHERE tool IS NOT NULL;
    CREATE INDEX IF NOT EXISTS idx_events_file_path ON events(file_path) WHERE file_path IS NOT NULL;
    CREATE INDEX IF NOT EXISTS idx_events_ts ON events(ts);
    CREATE UNIQUE INDEX IF NOT EXISTS idx_patterns_class_session ON patterns(event_class, COALESCE(session_id, ''));",
];

/// Apply all pending migrations. Stores version in `PRAGMA user_version`.
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
