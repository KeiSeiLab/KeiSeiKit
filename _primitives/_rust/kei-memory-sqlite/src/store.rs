// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! SQLite-backed storage layer. The async surface lives in `backend.rs`;
//! this module is sync (rusqlite is sync) and exposes a `SqliteStore`
//! whose Arc-cloned handle is shared by the backend, which wraps the
//! actual blocking calls in `tokio::task::spawn_blocking`.
//!
//! Connection is guarded by `std::sync::Mutex` because rusqlite's
//! `Connection` is not `Sync` on its own. The blocking surface is small
//! (one `lock()` per backend op) and the spawn_blocking thread holds it
//! only for the duration of the SQL.

use crate::error::Result;
use crate::schema::apply_schema;
use rusqlite::Connection;
use std::path::Path;
use std::sync::Mutex;

/// Owned SQLite handle. Cheap to wrap in `Arc` for sharing across
/// `SqliteBackend` clones (see `backend.rs`).
pub struct SqliteStore {
    conn: Mutex<Connection>,
}

impl SqliteStore {
    /// Open or create a SQLite DB at `path`. Schema is applied
    /// idempotently on every open.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let conn = Connection::open(path.as_ref())?;
        apply_schema(&conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// In-memory store for tests / ephemeral fixtures. Schema applied.
    pub fn from_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        apply_schema(&conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Borrow the connection mutex. Backend uses this from inside
    /// `spawn_blocking` so the blocking lock is off the async runtime.
    pub fn lock(&self) -> std::sync::MutexGuard<'_, Connection> {
        // Mutex poisoning aborts here on purpose: a panic mid-transaction
        // means the in-memory state is suspect.
        self.conn.lock().expect("sqlite connection mutex poisoned")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_memory_opens_and_applies_schema() {
        let s = SqliteStore::from_memory().expect("open");
        let conn = s.lock();
        // Probe the schema: the table must exist.
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='memory_items'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(count, 1, "memory_items table must exist");
    }

    #[test]
    fn from_path_creates_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("kei.db");
        let _s = SqliteStore::from_path(&path).expect("open");
        assert!(path.exists(), "DB file must exist after open");
    }

    #[test]
    fn indexes_present() {
        let s = SqliteStore::from_memory().unwrap();
        let conn = s.lock();
        let n: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND name LIKE 'idx_memory_items_%'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert!(n >= 2, "expected idx_memory_items_kind_key and idx_memory_items_created_at");
    }
}
