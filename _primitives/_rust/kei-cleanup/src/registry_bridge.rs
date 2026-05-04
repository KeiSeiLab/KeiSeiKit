//! Bridge: cleanup findings → kei-registry `cleanup_findings` table.
//!
//! For each [`Finding`] in a [`CleanupReport`], inserts one row in the
//! `cleanup_findings` table of the kei-registry SQLite DB. The schema
//! is owned by kei-registry (migration v5; see Track D); this bridge
//! only writes — it does NOT create or migrate the table.
//!
//! Schema (kei-registry v5):
//! ```sql
//! CREATE TABLE cleanup_findings (
//!     id INTEGER PRIMARY KEY,
//!     workspace_sha TEXT NOT NULL,
//!     timestamp_unix INTEGER NOT NULL,
//!     finding_json TEXT NOT NULL,
//!     severity TEXT NOT NULL,
//!     kind TEXT NOT NULL
//! ) STRICT;
//! ```
//!
//! NOTE: this code returns `Err` cleanly if the `cleanup_findings`
//! table is not yet present (Track D ships the migration). Callers
//! treat that as a soft failure and skip predicate emission.

use crate::report::{Finding, FindingKind, Severity};
use chrono::Utc;
use rusqlite::{params, Connection};
use std::path::Path;
use thiserror::Error;

/// Bridge-specific error type — kept narrow so callers can match.
#[derive(Debug, Error)]
pub enum BridgeError {
    /// SQLite open / prepare / execute error.
    #[error("sqlite: {0}")]
    Sqlite(#[from] rusqlite::Error),

    /// JSON serialisation failure.
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
}

/// Insert one row per finding into `cleanup_findings`. Returns the
/// number of rows inserted. The transaction is atomic; partial
/// progress is rolled back on any error.
pub fn emit_to_registry(
    db_path: &Path,
    findings: &[Finding],
    workspace_sha: &str,
) -> Result<usize, BridgeError> {
    let mut conn = Connection::open(db_path)?;
    let ts = Utc::now().timestamp();
    let tx = conn.transaction()?;
    let mut count = 0usize;
    for f in findings {
        insert_one(&tx, workspace_sha, ts, f)?;
        count += 1;
    }
    tx.commit()?;
    Ok(count)
}

fn insert_one(
    tx: &rusqlite::Transaction<'_>,
    workspace_sha: &str,
    ts: i64,
    f: &Finding,
) -> Result<(), BridgeError> {
    let kind = predicate_kind(&f.kind);
    let sev = severity_str(&f.severity);
    let body = serde_json::to_string(f)?;
    tx.execute(
        "INSERT INTO cleanup_findings \
         (workspace_sha, timestamp_unix, finding_json, severity, kind) \
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![workspace_sha, ts, body, sev, kind],
    )?;
    Ok(())
}

/// Map FindingKind → predicate kind string used by kei-registry.
pub fn predicate_kind(k: &FindingKind) -> &'static str {
    match k {
        FindingKind::DeadCode => "no_dead_code",
        FindingKind::UnusedDep => "no_unused_dep",
        FindingKind::DepDrift => "version_matches_workspace",
        FindingKind::LocFile => "loc_under_limit",
        FindingKind::LocFunction => "fn_loc_under_limit",
        FindingKind::StaleTodo => "todo_age_within",
        FindingKind::CoverageGap => "test_covers_derivation",
        FindingKind::WorkspaceTestNeeded => "no_cross_crate_test",
        FindingKind::DocWarning => "doc_warnings_zero",
        FindingKind::NamingDrift => "naming_consistent",
    }
}

fn severity_str(s: &Severity) -> &'static str {
    match s {
        Severity::High => "high",
        Severity::Medium => "medium",
        Severity::Low => "low",
    }
}
