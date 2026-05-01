//! kei-prune — retire unused agents / primitives based on kei-ledger
//! usage stats. Mirrors biological pruning: the brain forgets what has
//! not activated for long enough.
//!
//! This is a pure metadata primitive. It does NOT delete anything. It
//! marks retirement in a sidecar table `prune_retirements` (agent_id PK,
//! retired_ts INT). The ledger's `agents.status` CHECK constraint does
//! not include `'retired'`, and we deliberately do not try to extend it
//! from outside kei-ledger — sidecar > schema entanglement.
//!
//! # Public API
//!
//! - [`PruneCandidate`] — row DTO.
//! - [`PruneStats`]     — bucket summary DTO.
//! - [`ensure_schema`]  — idempotent sidecar installer.
//! - [`candidates`]     — list eligible-for-retirement agents.
//! - [`mark_retired`]   — record retirement (idempotent).
//! - [`stats`]          — total / active / idle / retired counts.
//!
//! # Constructor Pattern
//!
//! Six cubes:
//! - `error.rs`     — PruneError enum.
//! - `candidate.rs` — PruneCandidate DTO.
//! - `schema.rs`    — sidecar DDL + ensure_schema.
//! - `prune.rs`     — candidates + mark_retired verbs.
//! - `stats.rs`     — PruneStats + stats verb.
//! - `main.rs`      — clap CLI dispatcher.

pub mod candidate;
pub mod error;
pub mod prune;
pub mod schema;
pub mod stats;

pub use candidate::PruneCandidate;
pub use error::PruneError;
pub use prune::{candidates, mark_retired};
pub use schema::ensure_schema;
pub use stats::{stats, PruneStats};
