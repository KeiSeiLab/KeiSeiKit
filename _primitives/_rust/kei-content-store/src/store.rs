//! Content store — thin shim over `kei_entity_store::Store`.
//!
//! Multi-schema convergence (2026-04-23): both `content_units` and
//! `campaigns` are engine-owned. `Store::open` hands the engine
//! `ALL_SCHEMAS` so migrations for both tables run in a single
//! atomic transaction.
//!
//! Verbs dispatch per-schema: callers that act on assets pass
//! `CONTENT_SCHEMA`, callers that act on campaigns pass
//! `CAMPAIGNS_SCHEMA`. Two bespoke SQL paths remain:
//! `prompts.rs::register_prompt` (hash-dedup) and
//! `campaigns.rs::{attach_asset,campaign_assets}` (composite PK).

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
