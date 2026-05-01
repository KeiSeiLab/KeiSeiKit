//! Per-category conflict scanners.
//!
//! Each sub-module exposes `fn scan(root: &Path) -> Vec<Conflict>`.
//! The CLI in `main.rs` calls them based on `--only` or runs all.

pub mod blocks;
pub mod cp;
pub mod hooks;
pub mod orphans;
pub mod rules;
