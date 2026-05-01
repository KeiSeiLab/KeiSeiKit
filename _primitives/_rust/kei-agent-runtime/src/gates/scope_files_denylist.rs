//! `scope::files-denylist` — PreToolUse:Edit|Write denies paths matching
//! `task.scope.files-denylist` globs. Overrides whitelist.
//!
//! As of v0.18 convergence wave: thin const wrapper over `PatternGate`.

use super::pattern_gate::{GateMode, PatternGate, PatternSource};

pub const FILES_DENYLIST: PatternGate = PatternGate {
    name: "scope::files-denylist",
    tools: &["Edit", "Write", "MultiEdit", "NotebookEdit"],
    field: "file_path",
    mode: GateMode::DenyIfMatch,
    patterns: PatternSource::TaskDenylist,
    bypass_env: None,
    deny_template: "scope violation — {path} matches files-denylist ({pat})",
};
