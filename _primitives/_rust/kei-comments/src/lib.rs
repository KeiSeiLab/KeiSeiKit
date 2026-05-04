//! kei-comments — sovereign threaded comment store for KeiWiki.
//! Replaces Giscus / GitHub Discussions. SQLite-backed, single-process,
//! soft-delete + reactions. Auth is gated upstream by cortex daemon.

use anyhow::{anyhow, Result};
use chrono::Utc;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::Path;

/// Hard cap on comment body in bytes — prevents DOS via large inserts.
pub const MAX_BODY_BYTES: usize = 10 * 1024;

/// Idempotent schema. Created on every `migrate()` call.
pub const SCHEMA_SQL: &str = "
CREATE TABLE IF NOT EXISTS comments (
  id TEXT PRIMARY KEY, page_id TEXT NOT NULL, author TEXT NOT NULL,
  body TEXT NOT NULL, parent_id TEXT, created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL, deleted INTEGER NOT NULL DEFAULT 0);
CREATE INDEX IF NOT EXISTS idx_comments_page ON comments(page_id);
CREATE INDEX IF NOT EXISTS idx_comments_parent ON comments(parent_id);
CREATE TABLE IF NOT EXISTS reactions (
  comment_id TEXT NOT NULL, author TEXT NOT NULL, emoji TEXT NOT NULL,
  PRIMARY KEY (comment_id, author, emoji));
CREATE INDEX IF NOT EXISTS idx_reactions_comment ON reactions(comment_id);
";

const SELECT_COLS: &str =
    "SELECT id, page_id, author, body, parent_id, created_at, updated_at, deleted FROM comments";

/// Public Comment row. Soft-deleted rows present `deleted = true` with body wiped.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Comment {
    pub id: String,
    pub page_id: String,
    pub author: String,
    pub body: String,
    pub parent_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted: bool,
}

/// SQLite-backed comment store.
pub struct CommentStore {
    conn: Connection,
}

impl CommentStore {
    /// Open or create the database file. Caller must invoke `migrate()` once.
    pub fn open(db_path: &Path) -> Result<Self> {
        if let Some(p) = db_path.parent() {
            if !p.as_os_str().is_empty() { std::fs::create_dir_all(p).ok(); }
        }
        Ok(Self { conn: Connection::open(db_path)? })
    }

    /// In-memory store, for tests.
    pub fn open_memory() -> Result<Self> {
        Ok(Self { conn: Connection::open_in_memory()? })
    }

    /// Idempotent — safe to call on every startup.
    pub fn migrate(&self) -> Result<()> {
        self.conn.execute_batch(SCHEMA_SQL)?;
        Ok(())
    }

    /// Insert a new comment. Returns the assigned id (16-hex of sha256).
    pub fn post(
        &self,
        page_id: &str,
        author: &str,
        body: &str,
        parent_id: Option<&str>,
    ) -> Result<String> {
        validate_body(body)?;
        let now = Utc::now().to_rfc3339();
        let id = derive_id(page_id, author, &now, body);
        self.conn.execute(
            "INSERT INTO comments (id, page_id, author, body, parent_id, created_at, updated_at, deleted) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 0)",
            params![id, page_id, author, body, parent_id, now, now],
        )?;
        Ok(id)
    }

    /// List all (incl. soft-deleted) comments for a page, ordered by created_at.
    pub fn list(&self, page_id: &str) -> Result<Vec<Comment>> {
        let sql = format!("{} WHERE page_id = ?1 ORDER BY created_at ASC", SELECT_COLS);
        let mut stmt = self.conn.prepare(&sql)?;
        let rows = stmt.query_map([page_id], row_to_comment)?;
        rows.collect::<rusqlite::Result<Vec<_>>>().map_err(Into::into)
    }

    /// Fetch by id; None if not present.
    pub fn get(&self, comment_id: &str) -> Result<Option<Comment>> {
        let sql = format!("{} WHERE id = ?1", SELECT_COLS);
        let mut stmt = self.conn.prepare(&sql)?;
        let mut rows = stmt.query_map([comment_id], row_to_comment)?;
        match rows.next() {
            Some(r) => Ok(Some(r?)),
            None => Ok(None),
        }
    }

    /// Soft-delete: only the original author may delete. Returns true on success.
    pub fn delete(&self, comment_id: &str, author: &str) -> Result<bool> {
        let Some(c) = self.get(comment_id)? else { return Ok(false) };
        if c.author != author {
            return Ok(false);
        }
        let now = Utc::now().to_rfc3339();
        let n = self.conn.execute(
            "UPDATE comments SET deleted = 1, body = '', updated_at = ?1 WHERE id = ?2",
            params![now, comment_id],
        )?;
        Ok(n > 0)
    }

    /// Add a reaction (idempotent — repeat is no-op via PK conflict).
    pub fn react(&self, comment_id: &str, author: &str, emoji: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO reactions (comment_id, author, emoji) VALUES (?1, ?2, ?3)",
            params![comment_id, author, emoji],
        )?;
        Ok(())
    }

    /// Remove a reaction (idempotent — missing row is no-op).
    pub fn unreact(&self, comment_id: &str, author: &str, emoji: &str) -> Result<()> {
        self.conn.execute(
            "DELETE FROM reactions WHERE comment_id = ?1 AND author = ?2 AND emoji = ?3",
            params![comment_id, author, emoji],
        )?;
        Ok(())
    }

    /// Map of emoji → list of authors who reacted with it.
    pub fn reactions(&self, comment_id: &str) -> Result<HashMap<String, Vec<String>>> {
        let mut stmt = self.conn.prepare(
            "SELECT emoji, author FROM reactions WHERE comment_id = ?1 ORDER BY emoji, author",
        )?;
        let rows = stmt.query_map([comment_id], |r| {
            Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))
        })?;
        let mut out: HashMap<String, Vec<String>> = HashMap::new();
        for r in rows {
            let (emoji, author) = r?;
            out.entry(emoji).or_default().push(author);
        }
        Ok(out)
    }
}

fn validate_body(body: &str) -> Result<()> {
    if body.len() > MAX_BODY_BYTES {
        return Err(anyhow!("body length {} exceeds cap {}", body.len(), MAX_BODY_BYTES));
    }
    if body.trim().is_empty() {
        return Err(anyhow!("body must not be empty"));
    }
    Ok(())
}

fn row_to_comment(r: &rusqlite::Row<'_>) -> rusqlite::Result<Comment> {
    Ok(Comment {
        id: r.get(0)?,
        page_id: r.get(1)?,
        author: r.get(2)?,
        body: r.get(3)?,
        parent_id: r.get(4)?,
        created_at: r.get(5)?,
        updated_at: r.get(6)?,
        deleted: r.get::<_, i64>(7)? != 0,
    })
}

fn derive_id(page_id: &str, author: &str, ts: &str, body: &str) -> String {
    let mut h = Sha256::new();
    for part in [page_id, author, ts, body] {
        h.update(part.as_bytes());
        h.update(b"\0");
    }
    let digest = h.finalize();
    let mut out = String::with_capacity(16);
    for b in &digest[..8] {
        out.push_str(&format!("{:02x}", b));
    }
    out
}
