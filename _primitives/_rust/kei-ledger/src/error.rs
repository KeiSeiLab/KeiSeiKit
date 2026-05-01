//! Error type for ledger operations that extend beyond raw SQL.
//!
//! Constructor Pattern: one cube = one error type + its three trait impls.
//! Kept as a separate module so `ledger.rs` stays under the 200-LOC cap.

use crate::schema::MAX_BRANCH_LEN;
use std::fmt;

/// Maximum depth walked by `ledger::tree()` before aborting with
/// `LedgerError::MaxDepthExceeded`. Guards against cyclic or runaway data.
pub const MAX_TREE_DEPTH: usize = 1024;

/// Errors from ledger ops that extend beyond raw SQL (tree walk + input
/// validation + DNA uniqueness). Hot-path SQL calls still return
/// `rusqlite::Error` directly when no typed surface is required.
#[derive(Debug)]
pub enum LedgerError {
    Sql(rusqlite::Error),
    /// BFS in `tree()` exceeded `MAX_TREE_DEPTH` iterations.
    MaxDepthExceeded,
    /// Branch name longer than `MAX_BRANCH_LEN` chars (audit L1 cap).
    BranchTooLong { field: &'static str, len: usize },
    /// Attempted `fork` with a DNA that is already present in the ledger.
    /// Caller decides whether to regenerate DNA with a fresh nonce — the
    /// ledger never silently retries (v5, 2026-04-23).
    DnaCollision { dna: String },
    /// v5 migration detected pre-existing duplicate DNAs in the agents
    /// table. The UNIQUE index cannot be applied without data loss; the
    /// operator must manually reconcile rows. `duplicates` lists each
    /// offending DNA and its occurrence count.
    ///
    /// Resolution (back up the file first): run
    /// `kei-ledger sql 'DELETE FROM agents WHERE rowid NOT IN
    ///  (SELECT MIN(rowid) FROM agents GROUP BY dna)'`
    /// then re-open to retry migration.
    DnaMigrationBlocked { duplicates: Vec<(String, usize)> },
}

impl fmt::Display for LedgerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LedgerError::Sql(e) => write!(f, "sql: {e}"),
            LedgerError::MaxDepthExceeded => write!(
                f,
                "tree walk exceeded {MAX_TREE_DEPTH} iterations (cycle or runaway ledger)"
            ),
            LedgerError::BranchTooLong { field, len } => write!(
                f,
                "{field} length {len} exceeds cap {MAX_BRANCH_LEN}"
            ),
            LedgerError::DnaCollision { dna } => write!(
                f,
                "dna collision: {dna} already present — regenerate nonce and retry"
            ),
            LedgerError::DnaMigrationBlocked { duplicates } => {
                write!(
                    f,
                    "v5 migration blocked: {} duplicate dna(s) in table; reconcile before reopen:",
                    duplicates.len()
                )?;
                for (dna, count) in duplicates {
                    write!(f, "\n  {dna} ({count}x)")?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for LedgerError {}

impl From<rusqlite::Error> for LedgerError {
    fn from(e: rusqlite::Error) -> Self {
        LedgerError::Sql(e)
    }
}
