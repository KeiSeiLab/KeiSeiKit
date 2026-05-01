//! kei-projects-watcher — fsevents daemon that keeps `kei-projects-index`
//! fresh by watching `~/Projects/` and debouncing 2 s before re-indexing
//! each touched project root.
//!
//! The library surface (this file) exposes the watcher + debounce types
//! plus CLI subcommand bodies so the binary stays under the
//! Constructor-Pattern file budget.
//!
//! Constructor Pattern: every concrete type lives in its own module
//! (`watcher`, `debounce`, `cli`); this file ONLY declares modules and
//! re-exports the public API.

pub mod cli;
pub mod debounce;
pub mod watcher;

pub use cli::{cmd_run, cmd_status, open_db};
pub use debounce::{project_root_of, Debouncer};
pub use watcher::Watcher;
