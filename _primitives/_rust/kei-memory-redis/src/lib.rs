// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! kei-memory-redis — MemoryBackend trait-impl backed by Redis 7+.
//!
//! Hosted Sleep Wave 6 atomar. Async via the `redis` crate (`aio` +
//! `tokio-comp`). Single-class-per-file Constructor Pattern:
//!
//! - [`error`] : crate-local error type, mappable into
//!   `kei_runtime_core::Error`.
//! - [`store`] : low-level Redis client + key-schema helpers (no trait).
//! - [`backend`] : [`backend::RedisBackend`] glues `RedisStore` to the
//!   `MemoryBackend` trait + carries a DNA.
//!
//! Live integration tests live in `tests/redis_smoke.rs` and are gated
//! behind the `live` cargo feature so a default `cargo test` run does
//! not require a running Redis.

pub mod backend;
pub mod error;
pub mod store;

pub use backend::RedisBackend;
pub use error::{Error, Result};
pub use store::RedisStore;
