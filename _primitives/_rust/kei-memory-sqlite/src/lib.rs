// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! kei-memory-sqlite — `MemoryBackend` impl over SQLite (rusqlite bundled).
//!
//! Hosted Sleep Wave 6 atomar. Offline-first, single-process, embedded
//! storage. Suitable for:
//! - per-user VM local memory store (file-backed)
//! - offline-first agents needing structured `MemoryItem` storage with
//!   indexed query (kind / key-prefix / tags / time)
//! - test fixtures (in-memory via `SqliteStore::from_memory`)
//!
//! Constructor Pattern (one file = one responsibility):
//! - [`error`]   : crate-local error type, mappable into `kei_runtime_core::Error`.
//! - [`schema`]  : DDL + idempotent `apply_schema`.
//! - [`store`]   : low-level rusqlite handle + path/in-memory constructors.
//! - [`backend`] : [`backend::SqliteBackend`] glues `SqliteStore` to the
//!   `MemoryBackend` trait + carries a DNA.
//!
//! Out of scope:
//! - cross-process concurrency beyond what SQLite offers (use Redis/sled siblings)
//! - remote mirroring (`mirror_to_remote` returns `Provider` error;
//!   git-push is the responsibility of `kei-sleep-sync.sh` per RULE 0.15)

pub mod backend;
pub mod error;
pub mod schema;
pub mod store;

pub use backend::SqliteBackend;
pub use error::{Error, Result};
pub use store::SqliteStore;
