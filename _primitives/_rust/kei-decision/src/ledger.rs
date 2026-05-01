//! Pre-fork ledger row writer — shells out to `kei-ledger fork` BEFORE
//! kei-spawn so each ranked action gets a "queued" row immediately. Useful
//! when /research output is piped straight into kei-decision execute and we
//! want every action visible in `kei-ledger list --status running` before
//! any agent boots.

use anyhow::{anyhow, Context, Result};
use serde::Serialize;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Clone, Serialize)]
pub struct LedgerPreForkOutput {
    pub agent_id: String,
    pub branch: String,
    pub spec_sha: String,
    pub stdout: String,
}

/// Invoke `kei-ledger fork <id> <branch> --spec-sha <sha>` (blocking).
/// `agent_id` is the planned id (matches what `kei-spawn` will use).
/// `spec_sha` is a content-derived hash of the planned task.toml.
pub fn pre_fork_ledger(agent_id: &str, branch: &str, spec_sha: &str) -> Result<LedgerPreForkOutput> {
    let bin = locate_kei_ledger().context("locate kei-ledger binary")?;
    let out = Command::new(&bin)
        .arg("fork")
        .arg(agent_id)
        .arg(branch)
        .arg("--spec-sha")
        .arg(spec_sha)
        .output()
        .with_context(|| format!("invoke {} fork {} {}", bin.display(), agent_id, branch))?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(anyhow!("kei-ledger fork failed (exit {:?}): {}", out.status.code(), stderr));
    }
    Ok(LedgerPreForkOutput {
        agent_id: agent_id.to_string(),
        branch: branch.to_string(),
        spec_sha: spec_sha.to_string(),
        stdout: String::from_utf8_lossy(&out.stdout).to_string(),
    })
}

/// Search PATH, then a known fallback under `~/Projects/KeiSeiKit/...`.
fn locate_kei_ledger() -> Result<PathBuf> {
    if let Ok(found) = which_path("kei-ledger") {
        return Ok(found);
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    let fallback = PathBuf::from(home)
        .join("Projects/KeiSeiKit/_primitives/_rust/target/release/kei-ledger");
    if fallback.exists() {
        return Ok(fallback);
    }
    Err(anyhow!(
        "kei-ledger not on PATH and fallback {} missing — build the workspace or `cargo install --path _primitives/_rust/kei-ledger` first",
        fallback.display()
    ))
}

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
