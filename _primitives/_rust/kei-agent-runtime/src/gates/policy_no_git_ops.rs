//! `policy::no-git-ops` — RULE 0.13 orchestrator-owns-git enforcement.
//!
//! Denies any Bash command matching `git`, `gh repo`, `gh api /repos`.
//! Bypass via env `ORCHESTRATOR_META=1` for orchestrator-meta agents.
//!
//! As of v0.18 convergence wave: thin const wrapper over `PatternGate`.

use super::pattern_gate::{GateMode, PatternGate, PatternSource};

pub const NO_GIT_OPS: PatternGate = PatternGate {
    name: "policy::no-git-ops",
    tools: &["Bash"],
    field: "command",
    mode: GateMode::DenyIfMatch,
    patterns: PatternSource::StaticRegex(&[
        r"(?m)(?:^|[;&|]|\s)git(?:\s|$)",
        r"(?m)(?:^|[;&|]|\s)gh\s+repo",
        r"(?m)(?:^|[;&|]|\s)gh\s+api\s+/?repos",
    ]),
    bypass_env: Some("ORCHESTRATOR_META"),
    deny_template: "RULE 0.13 — git operation blocked (pattern {pat})",
};
