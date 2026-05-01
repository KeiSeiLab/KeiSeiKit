//! kei-memory — offline session analyzer + recurring-pattern detector.
//!
//! Library API: re-exports internal modules so binaries, tests, and
//! external Rust consumers can use ingest/analyze/patterns without
//! the `#[path = ...]` test-time hack.

pub mod analyze;
pub mod backlog;
pub mod classifier;
pub mod coaccess;
pub mod commands;
pub mod dump;
pub mod error;
pub mod extract;
pub mod ingest;
pub mod injection_guard;
pub mod injection_patterns;
pub mod patterns;
pub mod schema;
pub mod similarity;
pub mod stats;
pub mod tfidf;
pub mod trace_line;
