// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! kei-memory-postgres — `MemoryBackend` impl over PostgreSQL via
//! `tokio-postgres`.
//!
//! Suitable for:
//! - shared per-fleet memory store (multi-process, multi-host)
//! - JSONB payloads with GIN indexability on tags
//! - production durability + WAL replication
//!
//! Out of scope:
//! - migrations beyond the single `apply_schema` idempotent bootstrap
//!   (use a dedicated migration tool for richer schema evolution)
//! - `mirror_to_remote` returns `Provider` — git push is the
//!   responsibility of `kei-sleep-sync.sh` per RULE 0.15.
//!
//! Why tokio-postgres instead of sqlx: schema is small (one table,
//! two indexes), no compile-time query macros needed, fewer transitive
//! deps. Keeps the primitive tight.

pub mod backend;
pub mod error;
pub mod query_builder;
pub mod schema;
pub mod store;

pub use backend::PostgresBackend;
pub use error::{Error, Result};
pub use schema::{apply_schema, SCHEMA_SQL};
pub use store::PgStore;
