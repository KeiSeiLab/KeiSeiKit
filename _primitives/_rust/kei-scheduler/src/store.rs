//! Scheduler store — thin shim over `kei_entity_store::Store`.
//!
//! Mirrors the kei-chat-store pattern: the engine owns DDL + migration
//! transactions, and this crate adds scheduler-specific SQL helpers
//! (`schedule`, `cancel`, `list_due`, `mark_run`) that live in sibling
//! modules.

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

    pub fn conn(&self) -> &Connection {
        self.inner.conn()
    }
}
