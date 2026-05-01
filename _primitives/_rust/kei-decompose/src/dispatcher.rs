//! kei-spawn / kei-ledger CLI wrapper.
//!
//! Same shell-out pattern as kei-decision: spawn a child process, capture
//! stdout JSON, parse into `SpawnRecord`. No tokio, no async.
//!
//! Binary lookup order (kei-spawn):
//!   1. `KEI_SPAWN_BIN` env var (absolute path)
//!   2. `kei-spawn` on PATH
//!   3. fallback `~/Projects/KeiSeiKit/_primitives/_rust/target/release/kei-spawn`
//!
//! Same logic for kei-ledger via `KEI_LEDGER_BIN`.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::emitter::EmitOutput;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnRecord {
    pub action_id: String,
    pub source_format: String,
    pub task_path: PathBuf,
    pub agent_id: Option<String>,
    pub dna: Option<String>,
    pub branch: Option<String>,
    pub worktree_path: Option<PathBuf>,
    pub ledger_row: Option<String>,
    pub spawn_status: String,
}

pub struct DispatchOpts {
    pub dry_run: bool,
    pub limit: Option<usize>,
    pub no_ledger: bool,
}

/// Dispatch each emitted task.toml to kei-spawn (and optionally kei-ledger).
///
/// Returns one SpawnRecord per dispatched task (in input order). Tasks beyond
/// `opts.limit` are skipped entirely.
pub fn dispatch_all(emitted: &[EmitOutput], opts: &DispatchOpts) -> Result<Vec<SpawnRecord>> {
    let cap = opts.limit.unwrap_or(emitted.len()).min(emitted.len());
    let slice = &emitted[..cap];
    slice
        .iter()
        .map(|e| dispatch_one(e, opts))
        .collect()
}

fn dispatch_one(emit: &EmitOutput, opts: &DispatchOpts) -> Result<SpawnRecord> {
    let mut rec = SpawnRecord {
        action_id: emit.action_id.clone(),
        source_format: emit.source_format.clone(),
        task_path: emit.path.clone(),
        agent_id: None,
        dna: None,
        branch: None,
        worktree_path: None,
        ledger_row: None,
        spawn_status: "skipped-dry-run".to_string(),
    };
    if opts.dry_run {
        return Ok(rec);
    }
    let spawn_bin = locate_spawn_bin()
        .ok_or_else(|| anyhow!("kei-spawn binary not found (tried env, PATH, fallback)"))?;
    let out = run_spawn(&spawn_bin, &emit.path)?;
    apply_spawn_output(&mut rec, &out);
    if !opts.no_ledger {
        if let Some(row) = pre_fork_ledger(&rec)? {
            rec.ledger_row = Some(row);
        }
    }
    Ok(rec)
}

fn locate_spawn_bin() -> Option<PathBuf> {
    locate_bin("kei-spawn", "KEI_SPAWN_BIN")
}

fn locate_ledger_bin() -> Option<PathBuf> {
    locate_bin("kei-ledger", "KEI_LEDGER_BIN")
}

fn locate_bin(cmd: &str, env_var: &str) -> Option<PathBuf> {
    if let Ok(p) = std::env::var(env_var) {
        let pb = PathBuf::from(p);
        if pb.is_file() {
            return Some(pb);
        }
    }
    if let Ok(p) = which(cmd) {
        return Some(p);
    }
    fallback_bin_path(cmd).filter(|p| p.is_file())
}

fn fallback_bin_path(cmd: &str) -> Option<PathBuf> {
    let home = std::env::var("HOME").ok()?;
    Some(Path::new(&home)
        .join("Projects/KeiSeiKit/_primitives/_rust/target/release")
        .join(cmd))
}

fn which(cmd: &str) -> Result<PathBuf> {
    let out = Command::new("/usr/bin/which")
        .arg(cmd)
        .output()
        .with_context(|| format!("which {}", cmd))?;
    if !out.status.success() {
        return Err(anyhow!("which {} failed", cmd));
    }
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() {
        Err(anyhow!("which returned empty"))
    } else {
        Ok(PathBuf::from(s))
    }
}

#[derive(Debug, Default, Clone)]
struct SpawnRaw {
    agent_id: Option<String>,
    dna: Option<String>,
    branch: Option<String>,
    worktree_path: Option<String>,
    status: String,
}

fn run_spawn(bin: &Path, task_path: &Path) -> Result<SpawnRaw> {
    let out = Command::new(bin)
        .arg("spawn")
        .arg(task_path)
        .output()
        .with_context(|| format!("invoke {}", bin.display()))?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        return Err(anyhow!("kei-spawn exit non-zero: {}", stderr));
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    parse_spawn_json(&stdout)
}

fn parse_spawn_json(stdout: &str) -> Result<SpawnRaw> {
    let json: serde_json::Value = serde_json::from_str(stdout)
        .with_context(|| "kei-spawn stdout was not valid JSON")?;
    Ok(SpawnRaw {
        agent_id: json.get("agent_id").and_then(|v| v.as_str()).map(str::to_string),
        dna: json.get("dna").and_then(|v| v.as_str()).map(str::to_string),
        branch: json.get("branch").and_then(|v| v.as_str()).map(str::to_string),
        worktree_path: json
            .get("worktree_path")
            .and_then(|v| v.as_str())
            .map(str::to_string),
        status: json
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("ok")
            .to_string(),
    })
}

fn apply_spawn_output(rec: &mut SpawnRecord, raw: &SpawnRaw) {
    rec.agent_id = raw.agent_id.clone();
    rec.dna = raw.dna.clone();
    rec.branch = raw.branch.clone();
    rec.worktree_path = raw.worktree_path.as_ref().map(PathBuf::from);
    rec.spawn_status = raw.status.clone();
}

fn pre_fork_ledger(rec: &SpawnRecord) -> Result<Option<String>> {
    let bin = match locate_ledger_bin() {
        Some(b) => b,
        None => return Ok(None),
    };
    let agent_id = match rec.agent_id.as_ref() {
        Some(a) => a,
        None => return Ok(None),
    };
    let out = Command::new(bin)
        .arg("fork")
        .arg(agent_id)
        .arg("--source-format")
        .arg(&rec.source_format)
        .output()
        .with_context(|| "invoke kei-ledger fork")?;
    if !out.status.success() {
        return Ok(None);
    }
    Ok(Some(
        String::from_utf8_lossy(&out.stdout).trim().to_string(),
    ))
}
