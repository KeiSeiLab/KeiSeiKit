//! `safety::no-dep-bump` gate — PreToolUse:Edit|Write denies edits to
//! Cargo.toml / Cargo.lock unless `ALLOW_DEP_BUMP=1` is in the env.
//!
//! As of v0.18 convergence wave: thin const wrapper over `PatternGate`.

use super::pattern_gate::{GateMode, PatternGate, PatternSource};

pub const NO_DEP_BUMP_GATE: PatternGate = PatternGate {
    name: "safety::no-dep-bump",
    tools: &["Edit", "Write", "MultiEdit"],
    field: "file_path",
    mode: GateMode::DenyIfMatch,
    patterns: PatternSource::StaticRegex(&[
        r"(^|[/\\])Cargo\.toml$",
        r"(^|[/\\])Cargo\.lock$",
    ]),
    bypass_env: Some("ALLOW_DEP_BUMP"),
    deny_template: "safety::no-dep-bump — {path} edit blocked (set ALLOW_DEP_BUMP=1 to override)",
};
