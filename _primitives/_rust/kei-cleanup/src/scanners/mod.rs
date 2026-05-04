//! Scanner registry — see CLEANUP-RUNTIME-SPEC.md §3.2.
//!
//! Each scanner module exposes one `pub fn scan(workspace, cfg)` that
//! returns `Result<Vec<Finding>, CleanupError>`. The dispatcher in
//! `lib.rs` runs them in a fixed order and aggregates findings.

pub mod dead_code;
pub mod dep_drift;
pub mod fn_extract;
pub mod loc_check;
pub mod unused_deps;

/// Names of scanners shipped in v0.1, in dispatch order.
pub const ALL: &[&str] = &["dead_code", "unused_deps", "dep_drift", "loc_check"];
