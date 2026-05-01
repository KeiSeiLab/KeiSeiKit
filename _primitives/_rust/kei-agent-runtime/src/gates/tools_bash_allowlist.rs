//! `tools::bash-allowlist` — PreToolUse:Bash denies commands not matching
//! one of the configured allowlist regexes.
//!
//! Renamed from `tools::cargo-only-bash` in v0.17. Old name still resolves
//! via registry alias.
//!
//! As of v0.18 convergence wave: thin const wrapper over `PatternGate`.

use super::pattern_gate::{GateMode, PatternGate, PatternSource};

pub const BASH_ALLOWLIST: PatternGate = PatternGate {
    name: "tools::bash-allowlist",
    tools: &["Bash"],
    field: "command",
    mode: GateMode::AllowIfMatch,
    patterns: PatternSource::StaticRegex(&[
        r"^\s*cargo(\s|$)",
        r"^\s*rustc(\s|$)",
        r"^\s*rustup(\s|$)",
        r"^\s*mkdir(\s|$)",
        r"^\s*rm\s+-rf\s+/tmp/",
        r"^\s*ls(\s|$)",
        r"^\s*pwd(\s|$)",
    ]),
    bypass_env: None,
    deny_template: "tools::bash-allowlist — `{cmd}` not in allowlist",
};
