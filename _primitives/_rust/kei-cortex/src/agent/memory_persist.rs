//! Disk-backed memory write for review outcomes.
//!
//! Constructor Pattern: this cube owns ONE responsibility — persisting
//! a successful review reply to the `pet_conversations` SQLite table
//! used by the rest of kei-cortex (`handlers/memory.rs`,
//! `kei_pet::memory`). Empty / short-circuit / error replies are
//! filtered out so the table only accrues genuine memory entries.
//!
//! Threading model: `record_review_blocking` is synchronous — callers
//! wrap it in `tokio::task::spawn_blocking` (matching the
//! `handlers/memory.rs` pattern). The file does not own a tokio
//! runtime so it stays cheap to import in non-async contexts (tests).

use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use kei_pet::memory::{ensure_schema, record_interaction, MemoryError, MemoryTag};
use rusqlite::Connection;

use super::anthropic_memory_invoker::is_error_reply;
use super::memory_review_prompt::is_nothing_to_save;

/// Role string the review writes under. Distinct from `"user"` /
/// `"assistant"` so the cortex memory-search handler can filter
/// review-only rows when ranking results.
pub const REVIEW_ROLE: &str = "memory_review";

/// Outcome of a single persist attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PersistOutcome {
    /// Reply was empty after trimming — nothing written.
    Empty,
    /// Reply matched the `Nothing to save.` short-circuit.
    NothingToSave,
    /// Reply was a transport-error placeholder.
    Error(String),
    /// Successful write; carries the new rowid.
    Wrote(i64),
    /// Underlying SQLite or schema error; write skipped.
    Failed(String),
}

/// Decide whether the reply is worth persisting. Pure — no I/O.
pub fn classify_reply(reply: &str) -> PersistOutcome {
    let trimmed = reply.trim();
    if trimmed.is_empty() {
        return PersistOutcome::Empty;
    }
    if is_error_reply(trimmed) {
        return PersistOutcome::Error(trimmed.to_string());
    }
    if is_nothing_to_save(trimmed) {
        return PersistOutcome::NothingToSave;
    }
    PersistOutcome::Wrote(0) // placeholder — actual rowid filled by writer
}

/// Synchronous write path. Caller wraps in `spawn_blocking`. Schema is
/// idempotent; the table is created on first call so a brand-new
/// install with no chat history still persists reviews.
pub fn record_review_blocking(
    db_path: &std::path::Path,
    tag: &MemoryTag,
    reply: &str,
) -> PersistOutcome {
    match classify_reply(reply) {
        PersistOutcome::Wrote(_) => {} // proceed to write
        other => return other,
    }
    let trimmed = reply.trim().to_string();
    let conn = match open_conn(db_path) {
        Ok(c) => c,
        Err(e) => return PersistOutcome::Failed(format!("open: {e}")),
    };
    if let Err(e) = ensure_schema(&conn) {
        return PersistOutcome::Failed(format!("schema: {e}"));
    }
    let ts = unix_now();
    match record_interaction(&conn, tag, REVIEW_ROLE, &trimmed, ts) {
        Ok(rowid) => PersistOutcome::Wrote(rowid),
        Err(MemoryError::Blocked(b)) => PersistOutcome::Failed(format!("injection: {b}")),
        Err(MemoryError::Sql(e)) => PersistOutcome::Failed(format!("sql: {e}")),
    }
}

/// Open the sqlite db, creating the parent directory if absent. The
/// memory handler creates the file on first read; this mirror keeps
/// review writes from depending on read-path ordering.
fn open_conn(db_path: &std::path::Path) -> rusqlite::Result<Connection> {
    if let Some(parent) = db_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    Connection::open(db_path)
}

/// Carry-around bundle for async contexts (chat handler) — avoids
/// passing 3 args plus a path through the `spawn_blocking` boundary.
#[derive(Debug, Clone)]
pub struct PersistRequest {
    pub db_path: PathBuf,
    pub tag: MemoryTag,
    pub reply: String,
}

impl PersistRequest {
    /// Run the blocking write. Designed to be the body of a
    /// `tokio::task::spawn_blocking(move || req.run())` call.
    pub fn run(&self) -> PersistOutcome {
        record_review_blocking(&self.db_path, &self.tag, &self.reply)
    }
}

fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
#[path = "memory_persist_test.rs"]
mod tests;
