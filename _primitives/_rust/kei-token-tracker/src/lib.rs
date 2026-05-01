//! kei-token-tracker — per-LLM-call token + cost observability store.
//!
//! Records [`TokenEvent`] rows after each LLM turn (cortex chat handlers,
//! agent loops, etc). Phase D sleep-report aggregates by model + day for
//! nightly markdown output.
//!
//! Constructor Pattern: file ≤200 LOC, function ≤30 LOC. Each cube is a
//! single responsibility — `event` (data shape), `schema` (DDL), `store`
//! (CRUD), `aggregate` (rollup), `sleep_report` (markdown), `cli` (clap
//! dispatch). The bin (`src/bin/kei-token-tracker.rs`) is a thin shim.

pub mod aggregate;
pub mod cli;
pub mod error;
pub mod event;
pub mod schema;
pub mod sleep_report;
pub mod store;

pub use aggregate::ModelAggregate;
pub use error::Error;
pub use event::TokenEvent;
pub use store::Store;
