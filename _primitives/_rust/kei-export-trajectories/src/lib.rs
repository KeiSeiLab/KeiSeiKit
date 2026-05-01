//! kei-export-trajectories — public library surface.
//!
//! Constructor Pattern: the binary (`main.rs`) and tests share the same
//! module tree by depending on this lib. External callers (e.g. a future
//! `kei-cortex` integration that exports on demand) get a stable Rust API
//! without re-implementing CLI parsing.
//!
//! HERMES-MIGRATION-PLAN P0.2: emits ShareGPT-compatible JSONL so any
//! Hermes-aware trainer / dataset loader / HuggingFace pipeline can ingest
//! KeiSei agent trajectories with zero conversion work.

pub mod builder;
pub mod builder_chatlog_parse;
pub mod ledger_reader;
pub mod memory_events;
pub mod sharegpt;
pub mod tool_stats;
pub mod writer;

#[cfg(test)]
mod builder_chatlog_parse_tests;

#[allow(deprecated)]
pub use builder::{record_to_trajectory, system_prompt, DEFAULT_SYSTEM_PROMPT, SYSTEM_PROMPT};
pub use builder_chatlog_parse::parse_chatlog_turns;
pub use ledger_reader::{LedgerReader, TrajectoryRecord};
pub use sharegpt::{From as ShareGptFrom, ShareGptMessage, ToolStats, Trajectory};
pub use tool_stats::{aggregate_tool_stats, normalize_keys};
pub use writer::write_jsonl;
