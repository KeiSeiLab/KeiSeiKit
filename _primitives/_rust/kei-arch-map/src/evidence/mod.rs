//! Per-kind evidence checkers. Each submodule = one Evidence variant.
//!
//! Public surface: each `check(...)` returns `(passed, reason_if_failed)`.

pub mod cargo_check;
pub mod cargo_check_safe;
pub mod file_exists;
pub mod file_size;
pub mod grep_count;
pub mod http_status;
pub mod json_field;
pub mod path_resolve;
pub mod regex_match;
