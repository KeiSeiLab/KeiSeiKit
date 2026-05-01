//! Prepare an agent invocation: write `tasks/<agent-id>/prompt.md`,
//! record the task.toml alongside it. Actual Claude `Agent` tool call is
//! the orchestrator's job per RULE 0.13.

use crate::capability::TaskSpec;
use crate::compose::compose_prompt;
use crate::validate::validate_agent_id;
use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Parse a task.toml file into `TaskSpec`.
///
/// Validates the embedded `task.agent-id` (if non-empty) before returning —
/// a hostile task.toml with `agent-id = "../../../etc/foo"` is rejected at
/// the parse boundary so it never reaches a downstream path sink.
pub fn load_task(path: &Path) -> Result<TaskSpec> {
    let text = fs::read_to_string(path)
        .with_context(|| format!("read task file {}", path.display()))?;
    let spec: TaskSpec = toml::from_str(&text)
        .with_context(|| format!("parse task TOML {}", path.display()))?;
    if !spec.task.agent_id.is_empty() {
        validate_agent_id(&spec.task.agent_id)
            .map_err(|e| anyhow!("task.agent-id rejected: {e}"))?;
    }
    Ok(spec)
}

/// Prepare a spawnable agent directory.
///
/// Returns the `agent-id`. Does NOT invoke the Agent tool — that is the
/// orchestrator's responsibility. Caller is expected to subsequently call
/// `kei-ledger fork <agent-id>` (or the Rust API) with the path returned.
pub fn prepare_agent(task: &TaskSpec, kit_root: &Path) -> Result<PreparedAgent> {
    let agent_id = resolve_agent_id(task)?;
    let prompt = compose_prompt(task, kit_root)?;
    let dir = kit_root.join("tasks").join(&agent_id);
    fs::create_dir_all(&dir)
        .with_context(|| format!("create tasks dir {}", dir.display()))?;
    let prompt_path = dir.join("prompt.md");
    fs::write(&prompt_path, &prompt)
        .with_context(|| format!("write prompt {}", prompt_path.display()))?;
    let task_path = dir.join("task.toml");
    fs::write(&task_path, toml::to_string_pretty(task)?)
        .with_context(|| format!("write task {}", task_path.display()))?;
    Ok(PreparedAgent { agent_id, dir, prompt_path, task_path })
}

/// Outcome of `prepare_agent`.
#[derive(Debug, Clone)]
pub struct PreparedAgent {
    pub agent_id: String,
    pub dir: PathBuf,
    pub prompt_path: PathBuf,
    pub task_path: PathBuf,
}

/// Resolve the effective `agent_id` — validator-checked, never creates
/// files as a side effect.
pub fn resolve_agent_id(task: &TaskSpec) -> Result<String> {
    if task.task.agent_id.is_empty() {
        return Err(anyhow!(
            "task.agent-id is empty — orchestrator must allocate via kei-ledger"
        ));
    }
    validate_agent_id(&task.task.agent_id)
        .map_err(|e| anyhow!("task.agent-id rejected: {e}"))?;
    Ok(task.task.agent_id.clone())
}
