//! SQL schema runner for the agent ledger.
//!
//! Constructor Pattern: this cube is the runner; the DDL list lives in
//! the sibling [`crate::migrations_list`] module. Splitting keeps the
//! file under the 200-LOC ceiling now that v8 (skill_invocations) has
//! landed.

use crate::error::LedgerError;
use rusqlite::{Connection, Result};

/// Maximum length (chars) accepted for `branch` and `parent_branch` columns.
/// Enforced by SQL CHECK (v3 migration) and CLI `value_parser` length cap.
pub const MAX_BRANCH_LEN: usize = 256;

/// Re-export the migration list for backward-compat. Existing callers
/// (`tests/v6_cost.rs`, `tests/v7_micro.rs`, `tests/integration.rs`,
/// `schema_test.rs`) import via `crate::schema::MIGRATIONS`. SSoT lives
/// in `migrations_list.rs`; this `pub use` keeps the import path stable.
pub use crate::migrations_list::MIGRATIONS;

/// Schema version constant — index of the latest migration entry.
/// Callers (CLI / lib / tests) compare against `PRAGMA user_version` to
/// confirm the ledger is up to date. Bumped together with the migration
/// list. v9 (2026-04-30) added kei-model-router posterior columns:
/// tokens_in, tokens_out, stubs_count, outcome, escalation_depth, plus the
/// VIRTUAL `task_class_dna` column (DNA with trailing nonce stripped) for
/// per-task-class empirical posterior aggregation.
pub const SCHEMA_VERSION: u32 = 9;

/// Schema version the v5 pre-check guards. Kept as a named constant so the
/// branch in `migrate()` stays self-documenting when future migrations land.
const V5_TARGET: i64 = 5;

/// Apply all pending migrations atomically (one transaction per version).
///
/// Prior design ran `execute_batch` and bumped `user_version` in a separate
/// call — partial failure left the schema half-applied and wedged restarts.
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
