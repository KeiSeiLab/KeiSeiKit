//! Direct SQLite read of the kei-ledger DB to resolve DNA → ledger row.
//!
//! kei-ledger ships as a binary-only crate (no lib target), so we query
//! its SQLite file directly. The DB path follows the same fallback order
//! used by the ledger binary itself.

use anyhow::{anyhow, Context, Result};
use rusqlite::{params, Connection, OptionalExtension};
use std::path::PathBuf;

/// Resolved ledger row subset that kei-replay needs.
#[derive(Debug, Clone)]
pub struct LedgerHit {
    pub id: String,
    pub dna: String,
    pub worktree_path: Option<String>,
    pub spec_sha: String,
}

/// DB path fallback: `$KEI_LEDGER_DB` env → `$HOME/.claude/agents/ledger.sqlite`.
pub fn default_db_path() -> PathBuf {
    if let Ok(env) = std::env::var("KEI_LEDGER_DB") {
        return PathBuf::from(env);
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".claude/agents/ledger.sqlite")
}

/// Look up a row whose `dna` column exactly matches the given string.
///
/// Returns `None` if no row matches. Errors on DB access failure.
pub fn find_by_dna(db_path: &std::path::Path, dna: &str) -> Result<Option<LedgerHit>> {
    let conn = Connection::open(db_path)
        .with_context(|| format!("open ledger DB {}", db_path.display()))?;
    let sql = "SELECT id, dna, worktree_path, spec_sha FROM agents WHERE dna = ?1 LIMIT 1";
    let row = conn
        .query_row(sql, params![dna], |r| {
            Ok(LedgerHit {
                id: r.get(0)?,
                dna: r.get::<_, Option<String>>(1)?.unwrap_or_default(),
                worktree_path: r.get(2)?,
                spec_sha: r.get(3)?,
            })
        })
        .optional()
        .with_context(|| format!("query DNA {dna} from {}", db_path.display()))?;
    Ok(row)
}

/// Resolve DNA → hit, or a well-typed error if the DNA isn't in the ledger.
pub fn require_by_dna(db_path: &std::path::Path, dna: &str) -> Result<LedgerHit> {
    find_by_dna(db_path, dna)?
        .ok_or_else(|| anyhow!("DNA `{dna}` not found in ledger {}", db_path.display()))
}
