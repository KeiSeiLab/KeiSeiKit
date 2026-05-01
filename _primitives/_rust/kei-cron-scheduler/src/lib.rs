//! P4.2 — Hermes-equivalent cron / at / interval scheduler.
//!
//! Three-mode schedule parsing (one-shot duration, recurring interval, cron
//! expression, ISO timestamp) on top of a JSON-on-disk job store. Mirrors the
//! Hermes `cron/jobs.py:102-209` parsing surface 1:1 so existing operators can
//! migrate without re-learning the schedule grammar.

#![forbid(unsafe_code)]
#![deny(unused_must_use)]

pub mod job;
pub mod parser;
pub mod runner;
pub mod store;

pub use job::{Job, JobId, Schedule};
pub use parser::{parse_schedule, ParseError};
pub use runner::{JobRunner, RunnerEvent};
pub use store::{JobStore, StoreError};
