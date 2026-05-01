//! Social store — thin shim over `kei_entity_store::Store`.
//!
//! Layer-A convergence (2026-04-23): generic CRUD verbs on `people`
//! (create/get/search/list) run through `kei_entity_store::verbs::*`
//! with `SOCIAL_SCHEMA`. Organization and interaction helpers still
//! use the raw connection against tables declared in
//! `custom_migrations` — they are not generic-CRUD.

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
