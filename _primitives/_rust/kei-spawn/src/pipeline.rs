//! pipeline — derive downstream handoff chain from a writer's role.
//!
//! When a spawn is invoked with `--pipeline`, the orchestrator wants to know:
//!   1. Which downstream roles does the writer's role declare in
//!      `[pipeline].handoff`? (e.g. `edit-local` → `["auditor"]`)
//!   2. What agent-ids should those downstream steps use?
//!   3. Where should the pipeline.json chain artefact be written?
//!   4. What skeleton task.toml should be scaffolded for each step?
//!
//! This module answers all four. It reads the writer's role file, parses
//! the optional `[pipeline]` section, and emits a `PipelineChain` the
//! caller can serialise + use to pre-create per-step task directories.
//!
//! Constructor Pattern: one module = one responsibility (pipeline derivation
//! only). No git, no shell, no ledger. Pure filesystem + TOML parsing.
//! No I/O beyond `std::fs::read_to_string` for role lookup and
//! `std::fs::write` / `create_dir_all` for scaffolding.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// One step in a downstream handoff chain.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PipelineStep {
    pub role: String,
    pub agent_id: String,
}

/// Ordered chain of handoff steps derived from a writer's role.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PipelineChain {
    pub steps: Vec<PipelineStep>,
}

/// Raw on-disk shape of `_roles/<name>.toml`'s `[pipeline]` section.
/// Tolerates absence — default = empty handoff = no downstream steps.
#[derive(Debug, Default, Deserialize)]
struct RolePipelineRaw {
    #[serde(default)]
    pipeline: PipelineSectionRaw,
}

#[derive(Debug, Default, Deserialize)]
struct PipelineSectionRaw {
    #[serde(default)]
    handoff: Vec<String>,
}

/// Read `_roles/<role>.toml` and return its `[pipeline].handoff` list.
/// Missing file → error. Missing `[pipeline]` section → empty Vec (OK).
pub fn pipeline_from_role(kit_root: &Path, role: &str) -> Result<Vec<String>> {
    let path = kit_root.join("_roles").join(format!("{role}.toml"));
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("read role file {}", path.display()))?;
    let raw: RolePipelineRaw = toml::from_str(&text)
        .with_context(|| format!("parse role TOML {}", path.display()))?;
    Ok(raw.pipeline.handoff)
}

/// Derive concrete `PipelineStep`s from a writer agent-id + handoff role
/// list. Each downstream step gets a distinct agent-id of the form
/// `<writer_id>-<role>` so ledger rows remain unique + parent-linked.
pub fn derive_steps(writer_id: &str, handoff_roles: &[String]) -> Vec<PipelineStep> {
    let mut steps = Vec::with_capacity(handoff_roles.len());
    for role in handoff_roles {
        let role_trimmed = role.trim();
        if role_trimmed.is_empty() {
            continue;
        }
        steps.push(PipelineStep {
            role: role_trimmed.to_string(),
            agent_id: format!("{writer_id}-{role_trimmed}"),
        });
    }
    steps
}

/// Serialise `chain` as pretty JSON into `out_path`. Creates parent dirs
/// if missing so callers can point at `tasks/<writer>/pipeline.json`
/// before the parent dir exists (unlikely in practice, but cheap).
pub fn emit_pipeline_json(out_path: &Path, chain: &PipelineChain) -> Result<()> {
    if let Some(parent) = out_path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("create parent dir {}", parent.display()))?;
        }
    }
    let json = serde_json::to_string_pretty(chain).context("serialise pipeline chain")?;
    std::fs::write(out_path, json)
        .with_context(|| format!("write pipeline json {}", out_path.display()))?;
    Ok(())
}

/// For each step in `chain`, scaffold a stub `tasks/<step.agent_id>/task.toml`
/// that the orchestrator can later enrich + hand to `kei-spawn spawn`.
///
/// We deliberately stop short of calling `prepare_agent` / `kei-ledger fork`
/// for downstream steps — those are the orchestrator's responsibility to
/// invoke in order (writer succeeds → auditor spawns → auditor PASS →
/// merger spawns). Scaffolding a stub makes that sequence mechanical.
///
/// Also emits `tasks/<writer_id>/pipeline.json` so the orchestrator can
/// inspect the full chain in one read.
pub fn scaffold_downstream_tasks(
    kit_root: &Path,
    writer_id: &str,
    chain: &PipelineChain,
) -> Result<()> {
    let writer_dir = kit_root.join("tasks").join(writer_id);
    std::fs::create_dir_all(&writer_dir)
        .with_context(|| format!("create writer tasks dir {}", writer_dir.display()))?;
    emit_pipeline_json(&writer_dir.join("pipeline.json"), chain)?;
    for step in &chain.steps {
        scaffold_one_step(kit_root, writer_id, step)?;
    }
    Ok(())
}

fn scaffold_one_step(kit_root: &Path, writer_id: &str, step: &PipelineStep) -> Result<()> {
    let step_dir = kit_root.join("tasks").join(&step.agent_id);
    std::fs::create_dir_all(&step_dir)
        .with_context(|| format!("create step dir {}", step_dir.display()))?;
    let stub_path = step_dir.join("task.stub.toml");
    let stub = build_task_stub(writer_id, step);
    std::fs::write(&stub_path, stub)
        .with_context(|| format!("write task stub {}", stub_path.display()))?;
    Ok(())
}

fn build_task_stub(writer_id: &str, step: &PipelineStep) -> String {
    format!(
        concat!(
            "# Auto-scaffolded pipeline stub. Enrich `[body].text` and run\n",
            "# `kei-spawn spawn <this file>` once the upstream agent returns.\n",
            "\n",
            "[task]\n",
            "role = \"{role}\"\n",
            "agent-id = \"{agent_id}\"\n",
            "parent-agent = \"{parent}\"\n",
            "\n",
            "[body]\n",
            "text = \"TODO: fill handoff body for {role} step\"\n",
        ),
        role = step.role,
        agent_id = step.agent_id,
        parent = writer_id,
    )
}

/// Convenience wrapper: read role, derive steps, return chain. Used by
/// spawn.rs when `--pipeline` is set at the CLI layer.
pub fn derive_chain_from_role(
    kit_root: &Path,
    writer_role: &str,
    writer_id: &str,
) -> Result<PipelineChain> {
    if writer_role.is_empty() {
        return Err(anyhow!("writer_role is empty"));
    }
    let handoff = pipeline_from_role(kit_root, writer_role)?;
    let steps = derive_steps(writer_id, &handoff);
    Ok(PipelineChain { steps })
}

/// Public helper to compute the path where pipeline.json will be written.
/// Exposed so tests + orchestrator can compare without duplicating the
/// layout convention.
pub fn pipeline_json_path(kit_root: &Path, writer_id: &str) -> PathBuf {
    kit_root.join("tasks").join(writer_id).join("pipeline.json")
}
