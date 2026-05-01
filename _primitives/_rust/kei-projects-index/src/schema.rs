//! SQLite schema for kei-projects-index.
//!
//! Constructor Pattern: one cube = schema DDL + initialiser. No business
//! logic. Single source of truth for the `projects` table consumed by
//! both `kei-projects-watcher` (writer) and `kei-cortex` (reader).

use rusqlite::{Connection, Result};

/// Full schema applied by `init`. Idempotent (`IF NOT EXISTS` everywhere).
/// Any structural change MUST be additive — pre-existing rows must survive
/// re-running `init` against a partially-populated DB.
pub const SCHEMA_DDL: &str = "
CREATE TABLE IF NOT EXISTS projects (
    path TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    has_git INTEGER NOT NULL,
    branch TEXT,
    dirty INTEGER NOT NULL DEFAULT 0,
    ahead INTEGER NOT NULL DEFAULT 0,
    behind INTEGER NOT NULL DEFAULT 0,
    last_commit_sha TEXT,
    last_commit_msg TEXT,
    last_commit_ts INTEGER,
    has_claude_md INTEGER NOT NULL DEFAULT 0,
    has_decisions_md INTEGER NOT NULL DEFAULT 0,
    has_runbook_md INTEGER NOT NULL DEFAULT 0,
    has_readme INTEGER NOT NULL DEFAULT 0,
    sqlite_count INTEGER NOT NULL DEFAULT 0,
    last_indexed_ts INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_projects_dirty ON projects(dirty);
CREATE INDEX IF NOT EXISTS idx_projects_last_commit ON projects(last_commit_ts DESC);
";

/// Apply (or re-apply) the schema. Idempotent — safe to call on every
/// open. Returns rusqlite errors directly; no typed error variant is
/// needed because all DDL is idempotent and a hard failure here is
/// always a fatal disk / corruption issue.
pub fn init(conn: &Connection) -> Result<()> {
    conn.execute_batch(SCHEMA_DDL)
}
