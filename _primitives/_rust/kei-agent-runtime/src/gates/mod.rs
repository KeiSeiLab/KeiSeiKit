//! PreToolUse gate capabilities.
//!
//! After v0.18 convergence wave: 5 of 6 gates are `PatternGate` consts
//! (pattern-driven, regex or glob). `tools::deny-tools` remains its own
//! impl — mechanism is tool-name match, not pattern match.

pub mod pattern_gate;
pub mod policy_no_git_ops;
pub mod safety_no_dep_bump;
pub mod scope_files_denylist;
pub mod scope_files_whitelist;
pub mod tools_bash_allowlist;
pub mod tools_deny_tools;
