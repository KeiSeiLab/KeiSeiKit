//! Crossdomain store — thin shim over `kei_entity_store::Store`.
//!
//! Layer-A convergence (2026-04-23): connection lifecycle + migrations +
//! `PRAGMA user_version` stamping now ride the shared engine via
//! `CROSSDOMAIN_SCHEMA`. Public surface preserved byte-for-byte so
//! `edges.rs`, `bfs.rs`, `auto_link.rs`, and integration tests compile
//! unchanged.
//!
//! Generic CRUD verbs are NOT wired here — kei-crossdomain is an
//! edges-only store with bespoke TextPair+extras columns; see
//! `schema.rs` for the architectural note on why engine's `link`/`rank`
//! verbs cannot serve this crate without a destructive schema rewrite.

use crate::schema::CROSSDOMAIN_SCHEMA;
use anyhow::Result;
use kei_entity_store::Store as EntityStore;
use rusqlite::Connection;
use std::path::Path;

pub struct Store {
    inner: EntityStore,
}

impl Store {
    pub fn open(path: &Path) -> Result<Self> {
        let inner = EntityStore::open(path, &[&CROSSDOMAIN_SCHEMA])?;
        Ok(Self { inner })
    }

    pub fn open_memory() -> Result<Self> {
        let inner = EntityStore::open_memory(&[&CROSSDOMAIN_SCHEMA])?;
        Ok(Self { inner })
    }

    pub fn conn(&self) -> &Connection {
        self.inner.conn()
    }
}
