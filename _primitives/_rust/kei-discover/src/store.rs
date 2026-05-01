//! `Store` — thin shim over `kei_entity_store::Store` wired to
//! `DISCOVER_SCHEMA`.
//!
//! Two constructors: `open(path)` for on-disk and `open_memory()` for
//! unit tests. The inner connection is exposed read-only via `.conn()`
//! so the per-verb modules can call `kei_entity_store::verbs::*`
//! directly without taking a mutable borrow on the Store.

use crate::error::DiscoverError;
use crate::schema::DISCOVER_SCHEMA;
use kei_entity_store::Store as EntityStore;
use rusqlite::Connection;
use std::path::Path;

pub struct Store {
    inner: EntityStore,
}

impl Store {
    pub fn conn(&self) -> &Connection {
        self.inner.conn()
    }
}

pub fn open(path: &Path) -> Result<Store, DiscoverError> {
    let inner = EntityStore::open(path, &[&DISCOVER_SCHEMA])
        .map_err(|e| DiscoverError::Storage(e.to_string()))?;
    Ok(Store { inner })
}

pub fn open_memory() -> Result<Store, DiscoverError> {
    let inner = EntityStore::open_memory(&[&DISCOVER_SCHEMA])
        .map_err(|e| DiscoverError::Storage(e.to_string()))?;
    Ok(Store { inner })
}
