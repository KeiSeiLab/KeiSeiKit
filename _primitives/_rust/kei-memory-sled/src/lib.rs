// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! kei-memory-sled — `MemoryBackend` impl over sled v0.34.
//!
//! Embedded, single-process key-value store. Suitable for:
//! - per-user VM local memory store
//! - offline-first agents needing structured `MemoryItem` storage
//! - test fixtures (cheap to spin up via `tempfile::tempdir`)
//!
//! Out of scope:
//! - cross-process concurrency beyond what sled itself offers
//! - remote mirroring (`mirror_to_remote` returns `Provider` error;
//!   git-push is the responsibility of `kei-sleep-sync.sh` per RULE 0.15)

pub mod backend;
pub mod error;
pub mod store;

pub use backend::SledBackend;
pub use error::{Error, Result};
pub use store::SledStore;
