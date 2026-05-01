//! Chat store — thin shim over `kei_entity_store::Store`.
//!
//! Multi-schema convergence (2026-04-23): BOTH `chat_messages` and
//! `chat_sessions` are now engine-owned. `Store::open` hands the engine
//! `ALL_SCHEMAS` so migrations for both tables run in a single
//! atomic transaction.
//!
//! Verbs dispatch per-schema: callers that act on messages pass
//! `MESSAGES_SCHEMA`, callers that act on sessions pass
//! `SESSIONS_SCHEMA`. The only bespoke SQL left is the per-message
//! session-counter UPDATE (`sessions.rs::bump_session_totals`) — the
//! engine has no "aggregate-on-related-insert" verb.

use crate::schema::ALL_SCHEMAS;
use anyhow::Result;
use kei_entity_store::Store as EntityStore;
use rusqlite::Connection;
use std::path::Path;

pub struct Store {
    inner: EntityStore,
}

impl Store {
    pub fn open(path: &Path) -> Result<Self> {
        let inner = EntityStore::open(path, ALL_SCHEMAS)?;
        Ok(Self { inner })
    }

    pub fn open_memory() -> Result<Self> {
        let inner = EntityStore::open_memory(ALL_SCHEMAS)?;
        Ok(Self { inner })
    }

    pub fn conn(&self) -> &Connection { self.inner.conn() }
}
