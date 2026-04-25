//! SQL schema for the agent ledger.
//!
//! Constructor Pattern: one cube = schema DDL + migration runner.
//! Single source of truth for table shape. Any structural change MUST
//! bump the migration list below; existing rows are preserved.

use crate::error::LedgerError;
use rusqlite::{Connection, Result};

/// Maximum length (chars) accepted for `branch` and `parent_branch` columns.
/// Enforced by SQL CHECK (v3 migration) and CLI `value_parser` length cap.
pub const MAX_BRANCH_LEN: usize = 256;

/// Ordered migrations. Index = schema version. Never reorder; append only.
pub const MIGRATIONS: &[&str] = &[
    // v1 — initial schema (RULE 0.12, 2026-04-21)
    "CREATE TABLE IF NOT EXISTS agents (
        id TEXT PRIMARY KEY,
        branch TEXT NOT NULL,
        parent_branch TEXT,
        spec_sha TEXT NOT NULL,
        status TEXT NOT NULL CHECK (status IN ('running','done','failed','merged','rejected')),
        started_ts INTEGER NOT NULL,
        finished_ts INTEGER,
        summary TEXT,
        worktree_path TEXT
    );
    CREATE INDEX IF NOT EXISTS idx_parent ON agents(parent_branch);
    CREATE INDEX IF NOT EXISTS idx_status ON agents(status);",
    // v2 — Layer G DNA identity column + prefix index (2026-04-23)
    "ALTER TABLE agents ADD COLUMN dna TEXT;
    CREATE INDEX IF NOT EXISTS idx_agents_dna_prefix ON agents(substr(dna, 1, 30));",
    // v3 — length caps on branch/parent_branch (audit L1, 2026-04-23)
    // Enforced via trigger rather than table CHECK because CHECK cannot be
    // added retroactively to an existing table without rebuilding it. The
    // triggers refuse inserts / updates with over-long values.
    "CREATE TRIGGER IF NOT EXISTS trg_agents_branch_len_ins
     BEFORE INSERT ON agents
     BEGIN
        SELECT RAISE(ABORT, 'branch length exceeds 256')
            WHERE length(NEW.branch) > 256;
        SELECT RAISE(ABORT, 'parent_branch length exceeds 256')
            WHERE NEW.parent_branch IS NOT NULL AND length(NEW.parent_branch) > 256;
     END;
     CREATE TRIGGER IF NOT EXISTS trg_agents_branch_len_upd
     BEFORE UPDATE OF branch, parent_branch ON agents
     BEGIN
        SELECT RAISE(ABORT, 'branch length exceeds 256')
            WHERE length(NEW.branch) > 256;
        SELECT RAISE(ABORT, 'parent_branch length exceeds 256')
            WHERE NEW.parent_branch IS NOT NULL AND length(NEW.parent_branch) > 256;
     END;",
    // v4 — creator_id + fork_parent_id lineage columns (RULE 0.12 extension,
    // 2026-04-23). Both nullable for backward-compat with pre-v4 rows.
    // creator_id: DNA or human id of agent that spawned this fork.
    // fork_parent_id: DNA of forked-from agent if forked; NULL otherwise.
    "ALTER TABLE agents ADD COLUMN creator_id TEXT;
    ALTER TABLE agents ADD COLUMN fork_parent_id TEXT;
    CREATE INDEX IF NOT EXISTS idx_agents_creator ON agents(creator_id);
    CREATE INDEX IF NOT EXISTS idx_agents_fork_parent ON agents(fork_parent_id);",
    // v5 — UNIQUE constraint on dna (2026-04-23). Prevents silent duplicate
    // rows when the 32-bit DNA nonce collides (~65k-agent birthday bound per
    // role+caps+scope+body). Multiple NULLs are still accepted — SQLite's
    // default index semantics treat NULLs as distinct, matching the v2
    // "dna optional" contract. Pre-check in `migrate()` aborts with
    // `LedgerError::DnaMigrationBlocked` if existing rows already conflict,
    // so the operator reconciles manually instead of losing data.
    "CREATE UNIQUE INDEX IF NOT EXISTS idx_agents_dna_unique ON agents(dna);",
    // v6 — cost-tracking columns for /usage endpoint (Wave 40, 2026-04-24).
    // `cost_cents` defaults to 0 so existing pre-v6 rows aggregate to a
    // zero contribution rather than NULL (which COALESCE would handle but
    // leaves the row's TOTAL ambiguous). `provider` / `model` default to
    // empty strings — the usage handler treats both as opaque keys for
    // `top_provider_model` grouping.
    //
    // SQLite ALTER TABLE ADD COLUMN is idempotent only via the `migrate()`
    // user_version gate above — re-running the literal DDL would error
    // with "duplicate column name". The runner skips this entry on a
    // already-v6 ledger because `current >= 6` short-circuits the loop.
    "ALTER TABLE agents ADD COLUMN cost_cents INTEGER DEFAULT 0;
    ALTER TABLE agents ADD COLUMN provider TEXT DEFAULT '';
    ALTER TABLE agents ADD COLUMN model TEXT DEFAULT '';",
    // v7 — micro-cents accumulator (Wave 44c, 2026-04-24).
    // 1 cent = 1_000_000 micro-cents. Pre-v7 rows backfill from existing
    // `cost_cents` so summed history stays equivalent (an integer cents
    // value × 1M lifts cleanly into micro-cents). New writes record the
    // EXACT micro-cent value alongside a rounded `cost_cents` mirror so
    // the `/usage` endpoint and external CLI keep cent-resolution.
    //
    // Idempotent under the user_version gate identical to v6: if the
    // column already exists from a prior partial migration, the runner
    // skips because `current >= 7` short-circuits before re-applying.
    "ALTER TABLE agents ADD COLUMN cost_micro_cents INTEGER DEFAULT 0;
    UPDATE agents SET cost_micro_cents = COALESCE(cost_cents, 0) * 1000000
        WHERE cost_micro_cents IS NULL OR cost_micro_cents = 0;",
];

/// Schema version constant — index of the latest migration entry.
/// Callers (CLI / lib / tests) compare against `PRAGMA user_version` to
/// confirm the ledger is up to date. Bumped together with `MIGRATIONS`.
pub const SCHEMA_VERSION: u32 = 7;

/// Schema version the v5 pre-check guards. Kept as a named constant so the
/// branch in `migrate()` stays self-documenting when future migrations land.
const V5_TARGET: i64 = 5;

/// Apply all pending migrations atomically (one transaction per version).
///
/// Prior design ran `execute_batch` and bumped `user_version` in a separate
/// call — a partial failure left the schema half-applied and wedged restarts.
/// Now each version's DDL + the `user_version` bump share a transaction, so
/// any error rolls everything back and the next startup retries cleanly.
///
/// The return type is `LedgerError` (not `rusqlite::Error`) because v5
/// surfaces a typed `DnaMigrationBlocked` when pre-existing duplicates would
/// make the UNIQUE index creation fail — callers deserve a structured error,
/// not an opaque "UNIQUE constraint failed" from raw SQL.
pub fn migrate(conn: &Connection) -> std::result::Result<(), LedgerError> {
    let current: i64 = conn
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .unwrap_or(0);
    for (i, sql) in MIGRATIONS.iter().enumerate() {
        let target = (i + 1) as i64;
        if current < target {
            if target == V5_TARGET {
                precheck_dna_uniqueness(conn)?;
            }
            apply_one(conn, sql, target).map_err(LedgerError::Sql)?;
        }
    }
    Ok(())
}

/// v5 pre-check — scan existing rows for duplicate non-NULL DNAs. If any
/// exist, abort with `DnaMigrationBlocked` listing each offender and its
/// count. NULL DNAs are ignored because SQLite's default UNIQUE semantics
/// treat multiple NULLs as distinct (legacy pre-v2 rows stay valid).
fn precheck_dna_uniqueness(conn: &Connection) -> std::result::Result<(), LedgerError> {
    let mut stmt = conn
        .prepare(
            "SELECT dna, COUNT(*) AS c FROM agents
             WHERE dna IS NOT NULL
             GROUP BY dna HAVING c > 1
             ORDER BY c DESC, dna ASC",
        )
        .map_err(LedgerError::Sql)?;
    let rows = stmt
        .query_map([], |r| {
            let dna: String = r.get(0)?;
            let count: i64 = r.get(1)?;
            Ok((dna, count as usize))
        })
        .map_err(LedgerError::Sql)?;
    let duplicates: Vec<(String, usize)> = rows
        .collect::<Result<Vec<_>>>()
        .map_err(LedgerError::Sql)?;
    if duplicates.is_empty() {
        Ok(())
    } else {
        Err(LedgerError::DnaMigrationBlocked { duplicates })
    }
}

/// Apply a single migration atomically: DDL + user_version bump in one txn.
fn apply_one(conn: &Connection, sql: &str, target: i64) -> Result<()> {
    conn.execute_batch("BEGIN IMMEDIATE")?;
    let step = (|| -> Result<()> {
        conn.execute_batch(sql)?;
        conn.pragma_update(None, "user_version", target)?;
        Ok(())
    })();
    match step {
        Ok(()) => conn.execute_batch("COMMIT"),
        Err(e) => {
            let _ = conn.execute_batch("ROLLBACK");
            Err(e)
        }
    }
}

/// Six required artefacts per agent (RULE 0.12 §completion bundle).
pub const REQUIRED_ARTEFACTS: &[&str] = &[
    "spec.md",
    "plan.md",
    "progress.json",
    "chatlog.md",
    "handoffs.md",
    "review.md",
];

#[cfg(test)]
#[path = "schema_test.rs"]
mod tests;
