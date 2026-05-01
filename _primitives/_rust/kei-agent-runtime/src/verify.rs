//! Run every verify-capability declared by the task's role and collect
//! results into a `VerifyReport`.
//!
//! `run-mode` of each capability is not declared in this phase's registry
//! (declarative side is phase 1's `capability.toml`). Runtime defaults to
//! `Worktree`; caller passes `RunMode::Both` to get the simulated-merge
//! pass as well.

use crate::capability::{RunMode, TaskSpec, VerifyContext, VerifyResult};
use crate::registry;
use crate::validate::validate_agent_id;
use anyhow::{anyhow, Result};
use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Clone, Serialize)]
pub struct VerifyReport {
    pub passed: Vec<String>,
    pub failed: Vec<FailedEntry>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FailedEntry {
    pub capability: String,
    pub reason: String,
    pub detail: Option<String>,
}

impl VerifyReport {
    pub fn is_clean(&self) -> bool {
        self.failed.is_empty()
    }
}

/// Run every verify capability listed in the role's required list, in order.
/// `capability_names` is the ordered role manifest (from `_roles/<role>.toml`).
pub fn verify_task(
    task: &TaskSpec,
    agent_id: &str,
    worktree_path: &Path,
    main_repo: &Path,
    run_mode: RunMode,
    capability_names: &[String],
    simulated_merge_path: Option<PathBuf>,
) -> Result<VerifyReport> {
    validate_agent_id(agent_id)
        .map_err(|e| anyhow!("agent_id rejected in verify_task: {e}"))?;
    let mut report = VerifyReport::default();
    for name in capability_names {
        let cap = match registry::get_verify(name) {
            Some(c) => c,
            None => continue,
        };
        let ctx = VerifyContext {
            agent_id,
            task,
            worktree_path,
            main_repo,
            run_mode,
            simulated_merge_path: simulated_merge_path.clone(),
        };
        match cap.verify(&ctx) {
            VerifyResult::Pass => report.passed.push(name.clone()),
            VerifyResult::Fail { reason, detail } => report.failed.push(FailedEntry {
                capability: name.clone(),
                reason,
                detail,
            }),
        }
    }
    Ok(report)
}

/// Extract the ordered capability list from a role.toml file,
/// resolving `extends` chains and `relaxes` subtractions (Layer E).
pub fn load_role_capabilities(kit_root: &Path, role: &str) -> Result<Vec<String>> {
    let resolved = crate::role::resolve_role(kit_root, role)?;
    Ok(resolved.required)
}
