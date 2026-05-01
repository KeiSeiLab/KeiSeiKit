//! spawn — orchestrator-driven task → prepared agent + ledger row.
//!
//! One public entry point: `spawn_from_task`. Given a task.toml and a
//! kit_root, it:
//!   1. Parses task.toml via `kei_agent_runtime::spawn::load_task`
//!   2. Composes `AgentInvocation` via `kei_agent_runtime::prepare::prepare`
//!      (auto-generates agent-id if absent)
//!   3. Copies the resolved agent-id back into the task and writes
//!      `tasks/<agent-id>/{prompt.md, task.toml}` via
//!      `kei_agent_runtime::spawn::prepare_agent`
//!   4. Computes spec_sha (SHA-256 of the task TOML content)
//!   5. Registers a running row in the ledger via `kei-ledger fork`
//!   6. Returns `SpawnOutput` — everything orchestrator needs to call
//!      Claude Code's `Agent` tool (serialised as JSON).
//!
//! Never invokes git. Never invokes the Agent tool. Per RULE 0.13.

use anyhow::{Context, Result};
use kei_agent_runtime::{prepare, spawn as runtime_spawn};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

use crate::ledger_sh;
use crate::pipeline::{self, PipelineChain};
use crate::precedent;

/// The bundle orchestrator hands to Claude Code's Agent tool.
#[derive(Debug, Clone, Serialize)]
pub struct SpawnOutput {
    pub agent_id: String,
    pub dna: String,
    pub role: String,
    pub subagent_type: String,
    pub isolation: Option<String>,
    pub description: String,
    pub prompt: String,
    pub prompt_path: PathBuf,
    pub task_path: PathBuf,
    pub spec_sha: String,
    pub branch: String,
    pub verify_command: String,
    pub ledger_row: String,
    pub next_step: String,
}

/// Main spawn entry. See module doc for the 6-step pipeline.
///
/// On `kei-ledger fork` failure, the task directory created at step 3 is
/// removed atomically so callers can retry without a half-created leftover
/// (HIGH fix #4). Ledger-fork failure is the first observable checkpoint —
/// earlier steps are pure composition and do not touch shared state.
pub fn spawn_from_task(task_path: &Path, kit_root: &Path) -> Result<SpawnOutput> {
    let mut task = runtime_spawn::load_task(task_path)
        .with_context(|| format!("load task {}", task_path.display()))?;
    let inv = prepare::prepare(&task, kit_root).context("compose AgentInvocation")?;
    // Propagate auto-generated agent-id back into the task so `prepare_agent`
    // can use it as the directory name and the ledger row keys by it.
    task.task.agent_id = inv.agent_id.clone();
    let prepared = runtime_spawn::prepare_agent(&task, kit_root).context("prepare_agent")?;
    let task_bytes = std::fs::read(&prepared.task_path)
        .with_context(|| format!("read written task {}", prepared.task_path.display()))?;
    let spec_sha = sha256_hex(&task_bytes);
    let branch = format!("agent/{}", inv.agent_id);
    let parent = task.task.parent_agent.as_deref().filter(|s| !s.is_empty());
    // Advisory precedent check — env-gated, never blocks.
    let _ = precedent::run_advisory(&spec_sha);
    register_in_ledger(&inv, &branch, parent, &spec_sha, &prepared)?;
    Ok(build_output(inv, prepared, spec_sha, branch))
}

/// Call `kei-ledger fork`; on failure, remove the prepared task dir so
/// the spawn attempt leaves no half-created state for retry.
fn register_in_ledger(
    inv: &prepare::AgentInvocation,
    branch: &str,
    parent: Option<&str>,
    spec_sha: &str,
    prepared: &runtime_spawn::PreparedAgent,
) -> Result<()> {
    if let Err(e) = ledger_sh::fork(
        &inv.agent_id,
        branch,
        parent,
        spec_sha,
        prepared.dir.to_str(),
        Some(&inv.dna),
    ) {
        rollback_task_dir(&prepared.dir);
        return Err(e.context("kei-ledger fork"));
    }
    Ok(())
}

/// Variant that additionally derives the downstream handoff chain from the
/// writer's role and scaffolds stub task files for each step. Used by the
/// `kei-spawn spawn --pipeline` CLI flag. Returns the main `SpawnOutput`
/// plus the derived chain so the caller can serialise both.
pub fn spawn_with_pipeline(
    task_path: &Path,
    kit_root: &Path,
) -> Result<(SpawnOutput, PipelineChain)> {
    let out = spawn_from_task(task_path, kit_root)?;
    let chain = pipeline::derive_chain_from_role(kit_root, &out.role, &out.agent_id)?;
    pipeline::scaffold_downstream_tasks(kit_root, &out.agent_id, &chain)
        .context("scaffold downstream pipeline tasks")?;
    Ok((out, chain))
}

fn rollback_task_dir(dir: &Path) {
    if dir.exists() {
        let _ = std::fs::remove_dir_all(dir);
    }
}

fn build_output(
    inv: prepare::AgentInvocation,
    prepared: runtime_spawn::PreparedAgent,
    spec_sha: String,
    branch: String,
) -> SpawnOutput {
    let next_step = format!(
        "Invoke Agent tool with subagent_type={}, isolation={}, prompt=<see prompt field or {}>",
        inv.subagent_type,
        inv.isolation.as_deref().unwrap_or("<none>"),
        prepared.prompt_path.display()
    );
    SpawnOutput {
        agent_id: inv.agent_id,
        dna: inv.dna,
        role: inv.role,
        subagent_type: inv.subagent_type,
        isolation: inv.isolation,
        description: inv.description,
        prompt: inv.prompt,
        prompt_path: prepared.prompt_path,
        task_path: prepared.task_path,
        spec_sha,
        branch,
        verify_command: inv.verify_command,
        ledger_row: inv.ledger_row,
        next_step,
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    let digest = h.finalize();
    let mut s = String::with_capacity(64);
    for b in digest {
        s.push_str(&format!("{:02x}", b));
    }
    s
}
