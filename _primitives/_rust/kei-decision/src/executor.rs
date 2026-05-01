//! Action executor — emit task.toml + shell out to `kei-spawn spawn`.
//!
//! Captures stdout (kei-spawn emits a JSON `SpawnOutput`), parses the fields
//! we surface in [`ExecuteOutput`]. If the binary cannot be found we look
//! at a hard-coded fallback path under `~/Projects/KeiSeiKit/_primitives/
//! _rust/target/release/kei-spawn` before giving up loud.

use anyhow::{anyhow, Context, Result};
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Serialize)]
pub struct ExecuteOutput {
    pub action_id: String,
    pub agent_id: String,
    pub dna: String,
    pub branch: String,
    pub worktree_path: String,
    pub task_path: PathBuf,
}

/// Invoke `kei-spawn spawn <task.toml>` and parse its JSON stdout.
///
/// Returns parsed agent_id / dna / branch / worktree fields surfaced for the
/// orchestrator. Caller writes the task.toml beforehand via
/// [`crate::emit_task_toml`].
pub fn execute_action(action_id: &str, task_path: &Path) -> Result<ExecuteOutput> {
    let bin = locate_kei_spawn().context("locate kei-spawn binary")?;
    let out = Command::new(&bin)
        .arg("spawn")
        .arg(task_path)
        .output()
        .with_context(|| format!("invoke {} spawn {}", bin.display(), task_path.display()))?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(anyhow!("kei-spawn spawn failed (exit {:?}): {}", out.status.code(), stderr));
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    parse_spawn_json(&stdout, action_id, task_path)
}

/// Parse the JSON `kei-spawn spawn` produces into our [`ExecuteOutput`].
fn parse_spawn_json(stdout: &str, action_id: &str, task_path: &Path) -> Result<ExecuteOutput> {
    let v: serde_json::Value = serde_json::from_str(stdout)
        .with_context(|| format!("parse kei-spawn stdout as JSON; raw={}", stdout))?;
    let agent_id = pull_string(&v, "agent_id")?;
    let dna = pull_string(&v, "dna")?;
    let branch = pull_string(&v, "branch")?;
    // kei-spawn surfaces the worktree via task_path or prompt_path; the
    // canonical home is the parent dir of the prepared prompt file.
    let worktree_path = pull_string(&v, "prompt_path")
        .ok()
        .and_then(|p| Path::new(&p).parent().map(|q| q.display().to_string()))
        .unwrap_or_else(|| "<unknown>".to_string());
    Ok(ExecuteOutput {
        action_id: action_id.to_string(),
        agent_id,
        dna,
        branch,
        worktree_path,
        task_path: task_path.to_path_buf(),
    })
}

fn pull_string(v: &serde_json::Value, key: &str) -> Result<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow!("kei-spawn JSON missing string field `{}`", key))
}

/// Search PATH first, fall back to a known release-build location.
fn locate_kei_spawn() -> Result<PathBuf> {
    if let Ok(found) = which_path("kei-spawn") {
        return Ok(found);
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    let fallback = PathBuf::from(home)
        .join("Projects/KeiSeiKit/_primitives/_rust/target/release/kei-spawn");
    if fallback.exists() {
        return Ok(fallback);
    }
    Err(anyhow!(
        "kei-spawn not on PATH and fallback {} missing — install `cargo install --path _primitives/_rust/kei-spawn` or build the workspace first",
        fallback.display()
    ))
}

/// Tiny `which` clone — checks each PATH entry for an executable file.
fn which_path(bin: &str) -> Result<PathBuf> {
    let path = std::env::var_os("PATH").ok_or_else(|| anyhow!("PATH unset"))?;
    for entry in std::env::split_paths(&path) {
        let candidate = entry.join(bin);
        if candidate.is_file() {
            return Ok(candidate);
        }
    }
    Err(anyhow!("{} not on PATH", bin))
}
