//! kei-leak-matrix — single source of truth for content protection patterns.
//!
//! See `security/leak-matrix.toml` for the SSoT data file. This crate
//! parses it once, compiles every regex upfront, and exposes scan +
//! substitute helpers used by hooks (no-github-push, sync-public.sh,
//! secrets-guard, genesis-leak-guard) and by ad-hoc CLI use.

pub mod cli;
pub mod matrix;
pub mod scanner;
pub mod substituter;

pub use matrix::{default_matrix_path, Category, Matrix, Rule, Scope, Severity};
pub use scanner::{emit_json, exit_code, scan_file, scan_string, scan_tree, Violation};
pub use substituter::substitute;
