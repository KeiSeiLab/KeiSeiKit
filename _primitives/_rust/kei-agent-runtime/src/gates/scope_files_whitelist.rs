//! `scope::files-whitelist` — PreToolUse:Edit|Write denies paths outside
//! `task.scope.files-whitelist` globs. Empty list = not applicable (allow).
//!
//! As of v0.18 convergence wave: thin const wrapper over `PatternGate`.

use super::pattern_gate::{GateMode, PatternGate, PatternSource};

pub const FILES_WHITELIST: PatternGate = PatternGate {
    name: "scope::files-whitelist",
    tools: &["Edit", "Write", "MultiEdit", "NotebookEdit"],
    field: "file_path",
    mode: GateMode::DenyIfUnmatched,
    patterns: PatternSource::TaskWhitelist,
    bypass_env: None,
    deny_template: "scope violation — {path} not in files-whitelist",
};
