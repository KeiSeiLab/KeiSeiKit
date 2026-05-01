//! Capability trait + context / result types.
//!
//! Per schema §Capability trait contract (Rust). One trait, dispatched by
//! string name via `registry::get()`. Gates return `GateDecision`;
//! verifies return `VerifyResult`. Defaults are no-op so gate-only and
//! verify-only capabilities omit the other half.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Shared Capability trait. Gate + verify methods both default to no-op
/// so impls only override what they implement.
pub trait Capability: Send + Sync {
    /// Namespaced capability name: `<category>::<slug>` (e.g. `policy::no-git-ops`).
    fn name(&self) -> &'static str;

    /// PreToolUse gate; called by `kei-capability check <name>`.
    fn check(&self, _ctx: &GateContext) -> GateDecision {
        GateDecision::NotApplicable
    }

    /// On-return verify; called by `kei-capability verify <name>`.
    fn verify(&self, _ctx: &VerifyContext) -> VerifyResult {
        VerifyResult::Pass
    }
}

/// Context passed to `Capability::check()` — constructed by the hook binary
/// from Claude Code's tool-use JSON payload.
pub struct GateContext<'a> {
    pub tool_name: &'a str,
    pub tool_input: &'a Value,
    pub task: &'a TaskSpec,
    pub env: &'a HashMap<String, String>,
}

/// Gate outcome. `Deny` exits 2 in the hook binary; `Allow`/`NotApplicable` exit 0.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateDecision {
    Allow,
    Deny { reason: String },
    NotApplicable,
}

/// Context passed to `Capability::verify()` — constructed from env vars by the
/// hook binary, or programmatically by `verify::verify_task`.
pub struct VerifyContext<'a> {
    pub agent_id: &'a str,
    pub task: &'a TaskSpec,
    pub worktree_path: &'a Path,
    pub main_repo: &'a Path,
    pub run_mode: RunMode,
    pub simulated_merge_path: Option<PathBuf>,
}

impl<'a> VerifyContext<'a> {
    /// Active run dir: simulated-merge path if present, otherwise the worktree.
    pub fn run_dir(&self) -> PathBuf {
        match (&self.run_mode, &self.simulated_merge_path) {
            (RunMode::SimulatedMerge, Some(p)) => p.clone(),
            _ => self.worktree_path.to_path_buf(),
        }
    }
}

/// Verify result. `Fail` exits non-zero in the hook binary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifyResult {
    Pass,
    Fail {
        reason: String,
        detail: Option<String>,
    },
}

/// Verify execution mode. Orchestrator splits `Both` into two sequential
/// `Worktree` + `SimulatedMerge` calls.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunMode {
    Worktree,
    SimulatedMerge,
    Both,
}

/// Parsed task.toml. Subset used by gates + verifies; parser lives in
/// `spawn.rs`.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct TaskSpec {
    #[serde(default)]
    pub task: TaskMeta,
    #[serde(default)]
    pub scope: TaskScope,
    #[serde(default)]
    pub verification: TaskVerification,
    #[serde(default)]
    pub output: TaskOutput,
    #[serde(default)]
    pub body: TaskBody,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct TaskMeta {
    #[serde(default)]
    pub role: String,
    #[serde(default, rename = "agent-id")]
    pub agent_id: String,
    #[serde(default, rename = "parent-agent")]
    pub parent_agent: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct TaskScope {
    #[serde(default, rename = "files-whitelist")]
    pub files_whitelist: Vec<String>,
    #[serde(default, rename = "files-denylist")]
    pub files_denylist: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct TaskVerification {
    #[serde(default, rename = "cargo-check-crates")]
    pub cargo_check_crates: Vec<String>,
    #[serde(default, rename = "cargo-test-crates")]
    pub cargo_test_crates: Vec<String>,
    #[serde(default, rename = "test-count-min")]
    pub test_count_min: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct TaskOutput {
    #[serde(default, rename = "report-fields-required")]
    pub report_fields_required: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct TaskBody {
    #[serde(default)]
    pub text: String,
}
