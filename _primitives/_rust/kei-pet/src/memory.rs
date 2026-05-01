//! Persistent conversation memory indexed by (user_id, pet_name).
//!
//! Each row is a single message exchange turn (role = "user" | "assistant" |
//! caller-defined). Storage is SQLite. No FTS: `search` is a simple LIKE scan
//! scoped by the (user_id, pet_name) tuple.
//!
//! Scope boundary: this module does not open connections — the caller
//! supplies a `rusqlite::Connection` (on-disk or in-memory). That keeps the
//! module hermetically testable and lets the host choose the DB path.

use crate::injection_check;
use rusqlite::{params, Connection};

/// Conversation stream identity: one stream per (user, pet) pair.
#[derive(Debug, Clone)]
pub struct MemoryTag {
    pub user_id: String,
    pub pet_name: String,
}

/// A single recorded interaction row.
#[derive(Debug, Clone)]
pub struct Interaction {
    pub id: i64,
    pub role: String,
    pub text: String,
    pub ts: i64,
}

/// Errors surfaced by this module.
#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    #[error(transparent)]
    Sql(#[from] rusqlite::Error),
    /// P2.1.b — content rejected by the injection check before persistence.
    /// Mirrors `kei-memory::injection_guard` Block-tier rejections so a
    /// malicious pet-conversation entry never lands in the SQLite table.
    #[error("injection check blocked write: {0}")]
    Blocked(String),
}

/// Create the `pet_conversations` table and its (user_id, pet_name, ts DESC)
/// index if they don't exist yet. Idempotent.
pub fn ensure_schema(conn: &Connection) -> Result<(), MemoryError> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS pet_conversations (
             id       INTEGER PRIMARY KEY AUTOINCREMENT,
             user_id  TEXT NOT NULL,
             pet_name TEXT NOT NULL,
             role     TEXT NOT NULL,
             text     TEXT NOT NULL,
             ts       INTEGER NOT NULL
         )",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_pet_conv_tag_ts
             ON pet_conversations (user_id, pet_name, ts DESC)",
        [],
    )?;
    Ok(())
}

/// Insert one interaction row, returning its rowid.
///
/// P2.1.b — `text` is scanned via `injection_check::scan` before
/// persistence. Block-tier hits short-circuit with `MemoryError::Blocked`;
/// the row is NOT inserted and the SQLite autoincrement counter is not
/// advanced. The injected `role` is callee-controlled; we trust the
/// caller's role string and only sanitise the user-supplied `text`.
pub fn record_interaction(
    conn: &Connection,
    tag: &MemoryTag,
    role: &str,
    text: &str,
    ts: i64,
) -> Result<i64, MemoryError> {
    if let Err(finding) = injection_check::scan(text) {
        return Err(MemoryError::Blocked(finding.to_string()));
    }
    conn.execute(
        "INSERT INTO pet_conversations (user_id, pet_name, role, text, ts)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![tag.user_id, tag.pet_name, role, text, ts],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Return up to `limit` most recent interactions for `tag`, newest first.
pub fn recent(
    conn: &Connection,
    tag: &MemoryTag,
    limit: usize,
) -> Result<Vec<Interaction>, MemoryError> {
    let mut stmt = conn.prepare(
        "SELECT id, role, text, ts
           FROM pet_conversations
          WHERE user_id = ?1 AND pet_name = ?2
          ORDER BY ts DESC, id DESC
          LIMIT ?3",
    )?;
    let rows = stmt.query_map(
        params![tag.user_id, tag.pet_name, limit as i64],
        row_to_interaction,
    )?;
    collect_rows(rows)
}

/// Return up to `limit` interactions whose `text` contains `query` as a
/// literal substring (case-insensitive via LIKE), scoped to `tag`,
/// newest first.
pub fn search(
    conn: &Connection,
    tag: &MemoryTag,
    query: &str,
    limit: usize,
) -> Result<Vec<Interaction>, MemoryError> {
    let pattern = format!("%{}%", escape_like(query));
    let mut stmt = conn.prepare(
        "SELECT id, role, text, ts
           FROM pet_conversations
          WHERE user_id = ?1 AND pet_name = ?2
            AND text LIKE ?3 ESCAPE '\\'
          ORDER BY ts DESC, id DESC
          LIMIT ?4",
    )?;
    let rows = stmt.query_map(
        params![tag.user_id, tag.pet_name, pattern, limit as i64],
        row_to_interaction,
    )?;
    collect_rows(rows)
}

fn row_to_interaction(row: &rusqlite::Row<'_>) -> rusqlite::Result<Interaction> {
    Ok(Interaction {
        id: row.get(0)?,
        role: row.get(1)?,
        text: row.get(2)?,
        ts: row.get(3)?,
    })
}

fn collect_rows<I>(rows: I) -> Result<Vec<Interaction>, MemoryError>
where
    I: Iterator<Item = rusqlite::Result<Interaction>>,
{
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

/// Escape LIKE metacharacters (`%`, `_`, `\`) so callers can pass raw text.
fn escape_like(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' | '%' | '_' => {
                out.push('\\');
                out.push(c);
            }
            _ => out.push(c),
        }
    }
    out
}
