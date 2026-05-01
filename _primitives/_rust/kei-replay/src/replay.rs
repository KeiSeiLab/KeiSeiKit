//! Replay — reconstruct a spawn's composed prompt from a DNA string.
//!
//! Pipeline:
//!   1. Parse DNA (validates shape).
//!   2. Resolve ledger hit (agent-id, worktree path, spec_sha).
//!   3. Locate `task.toml` (explicit override OR `<worktree>/tasks/<agent-id>/task.toml`).
//!   4. Load task + kit root, re-run `kei_agent_runtime::compose::compose_prompt`.
//!   5. Recompute body hash from the re-loaded `task.body.text` and compare
//!      to the DNA body segment — mismatch = schema drift.

use anyhow::{anyhow, Context, Result};
use kei_agent_runtime::compose::compose_prompt;
use kei_agent_runtime::dna::Dna;
use kei_agent_runtime::spawn::load_task;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

/// Outcome of a replay pass.
#[derive(Debug, Clone)]
pub struct Replay {
    pub dna: Dna,
    pub task_toml_text: String,
    pub composed_prompt: String,
    pub recomputed_body_hash: String,
    pub body_hash_matches: bool,
}

/// Reconstruct the spawn.
///
/// `db_path`   — ledger SQLite file.
/// `dna_str`   — the DNA string supplied on the CLI.
/// `task_override` — explicit `task.toml` path if caller knows it (bypasses lookup).
/// `kit_root`  — repo root that holds `_roles/` + `_capabilities/`.
pub fn replay(
    db_path: &Path,
    dna_str: &str,
    task_override: Option<&Path>,
    kit_root: &Path,
) -> Result<Replay> {
    let dna = Dna::parse(dna_str).map_err(|e| anyhow!("invalid DNA: {e}"))?;
    let task_path = resolve_task_path(db_path, &dna, dna_str, task_override)?;
    let task_toml_text = std::fs::read_to_string(&task_path)
        .with_context(|| format!("read task {}", task_path.display()))?;
    let task = load_task(&task_path)?;
    let composed_prompt = compose_prompt(&task, kit_root)
        .with_context(|| format!("re-compose prompt for {}", dna_str))?;
    let recomputed_body_hash = short_sha256(&task.body.text);
    let body_hash_matches = recomputed_body_hash.eq_ignore_ascii_case(&dna.body_hash);
    Ok(Replay {
        dna,
        task_toml_text,
        composed_prompt,
        recomputed_body_hash,
        body_hash_matches,
    })
}

/// Prefer explicit override; else derive from ledger worktree_path + agent-id.
fn resolve_task_path(
    db_path: &Path,
    _dna: &Dna,
    dna_str: &str,
    task_override: Option<&Path>,
) -> Result<PathBuf> {
    if let Some(p) = task_override {
        return Ok(p.to_path_buf());
    }
    let hit = crate::ledger_lookup::require_by_dna(db_path, dna_str)?;
    let wt = hit.worktree_path.ok_or_else(|| {
        anyhow!(
            "DNA body hash not resolvable: ledger row `{}` has no worktree_path — \
             task.toml required (pass --task <path>)",
            hit.id
        )
    })?;
    let path = PathBuf::from(wt).join("tasks").join(&hit.id).join("task.toml");
    if !path.is_file() {
        return Err(anyhow!(
            "DNA body hash not resolvable: task.toml not found at {} — \
             pass --task <path> to override",
            path.display()
        ));
    }
    Ok(path)
}

/// 8-hex SHA-256 prefix — mirrors `kei_agent_runtime::dna::short_sha256`.
/// Kept local (that fn is private) so we stay drop-in compatible with the
/// DNA body_hash format without exposing a new API surface.
fn short_sha256(input: &str) -> String {
    let digest = Sha256::digest(input.as_bytes());
    format!(
        "{:02X}{:02X}{:02X}{:02X}",
        digest[0], digest[1], digest[2], digest[3]
    )
}
