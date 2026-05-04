//! Scanner registry — see CLEANUP-RUNTIME-SPEC.md §3.2.
//!
//! Each scanner module exposes one `pub fn scan(workspace, cfg)` that
//! returns `Result<Vec<Finding>, CleanupError>`. The dispatcher in
//! `lib.rs` runs them in a fixed order and aggregates findings.

pub mod coverage_map;
pub mod dead_code;
pub mod dep_drift;
pub mod doc_warnings;
pub mod fn_extract;
pub mod loc_check;
pub mod naming_consistency;
pub mod todo_age;
pub mod unused_deps;
pub mod workspace_tests;

/// Names of scanners shipped, in dispatch order.
pub const ALL: &[&str] = &[
    "dead_code",
    "unused_deps",
    "dep_drift",
    "loc_check",
    "todo_age",
    "coverage_map",
    "workspace_tests",
    "doc_warnings",
    "naming_consistency",
];
