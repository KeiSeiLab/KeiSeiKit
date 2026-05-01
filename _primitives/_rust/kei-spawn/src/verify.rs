//! verify — orchestrator-side post-return verification + ledger bookkeeping.
//!
//! Given an agent-id and the worktree path the harness returned, this module:
//!   1. Reads `<kit_root>/tasks/<agent-id>/task.toml`
//!   2. Resolves role → ordered capability list
//!   3. Runs `kei_agent_runtime::verify::verify_task` (worktree pass)
//!   4. On pass, marks ledger row `done`; on fail, marks `failed`
//!   5. Emits a `VerifyOutput` JSON (pass/fail + failed-capability list)
//!
//! Simulated-merge pass is orchestrator-scope (needs git) so we stay in
//! `RunMode::Worktree`. A future `kei-spawn verify-merge` flavour can be
//! added once orchestrator-owned git helpers exist.

use anyhow::{anyhow, Context, Result};
use kei_agent_runtime::capability::RunMode;
use kei_agent_runtime::{spawn as runtime_spawn, verify as runtime_verify};
use serde::Serialize;
use std::path::{Path, PathBuf};

use crate::ledger_sh;

/// Outcome of a single verify pass, including failed-capability detail.
#[derive(Debug, Clone, Serialize)]
pub struct VerifyOutput {
    pub agent_id: String,
    pub passed: Vec<String>,
    pub failed: Vec<runtime_verify::FailedEntry>,
    pub is_clean: bool,
    pub worktree: PathBuf,
}

/// Main verify entry. On pass → ledger done; on fail → ledger failed.
pub fn verify_agent(agent_id: &str, worktree: &Path, kit_root: &Path) -> Result<VerifyOutput> {
    let task_path = task_toml_path(kit_root, agent_id)?;
    let task = runtime_spawn::load_task(&task_path)
        .with_context(|| format!("load task {}", task_path.display()))?;
    let caps = runtime_verify::load_role_capabilities(kit_root, &task.task.role)
        .context("resolve role capabilities")?;
    let report = runtime_verify::verify_task(
        &task, agent_id, worktree, kit_root, RunMode::Worktree, &caps, None,
    )
    .context("run verify pipeline")?;
    let is_clean = report.is_clean();
    update_ledger(agent_id, &report)?;
    Ok(VerifyOutput {
        agent_id: agent_id.to_string(),
        passed: report.passed,
        failed: report.failed,
        is_clean,
        worktree: worktree.to_path_buf(),
    })
}

/// Resolve and validate `<kit>/tasks/<agent-id>/task.toml`.
fn task_toml_path(kit_root: &Path, agent_id: &str) -> Result<std::path::PathBuf> {
    let p = kit_root.join("tasks").join(agent_id).join("task.toml");
    if !p.is_file() {
        return Err(anyhow!(
            "task.toml not found at {}: did you run `kei-spawn spawn` first?",
            p.display()
        ));
    }
    Ok(p)
}

fn update_ledger(agent_id: &str, report: &runtime_verify::VerifyReport) -> Result<()> {
    if report.is_clean() {
        let summary = format!("verify passed ({} capabilities)", report.passed.len());
        ledger_sh::done(agent_id, &summary).context("kei-ledger done")?;
    } else {
        let reason = format_failures(&report.failed);
        ledger_sh::fail(agent_id, &reason).context("kei-ledger fail")?;
    }
    Ok(())
}

fn format_failures(failed: &[runtime_verify::FailedEntry]) -> String {
    let mut parts = Vec::with_capacity(failed.len());
    for f in failed {
        parts.push(format!("{}: {}", f.capability, f.reason));
    }
    parts.join("; ")
}
