//! Orchestrator-facing wrapper: task.toml → everything needed to invoke
//! Claude Code's `Agent` tool in a single copy-paste-ready bundle.
//!
//! Per RULE 0.13, the orchestrator (main session) owns branch creation,
//! `isolation: "worktree"` selection, and the actual Agent-tool call. This
//! module only assembles the arguments — no git, no spawn, no shell.
//!
//! Wire: `prepare()` = role resolution + `compose_prompt()` + role→subagent_type
//! resolution + `Dna::compose`. Deliberately does NOT create `tasks/<id>/` on
//! disk (that is `spawn::prepare_agent`'s job) so orchestrator can inspect
//! before committing. The "ledger row" field is a pretty-printed string, not
//! a DB write — ledger persistence is the orchestrator's step.

use crate::capability::TaskSpec;
use crate::compose::compose_prompt;
use crate::dna::Dna;
use crate::role::resolve_role;
use crate::validate::{autogen_agent_id, validate_agent_id};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Everything the orchestrator needs to hand the Claude `Agent` tool.
#[derive(Debug, Clone, Serialize)]
pub struct AgentInvocation {
    pub agent_id: String,
    pub role: String,
    pub prompt: String,
    pub subagent_type: String,
    pub isolation: Option<String>,
    pub description: String,
    pub verify_command: String,
    pub ledger_row: String,
    /// Layer G — composition fingerprint, `<role>::<caps>::<scope>::<body>-<nonce>`.
    pub dna: String,
}

/// Assemble an `AgentInvocation` from a parsed task.toml.
///
/// Errors if the role is unknown or non-spawnable (points at RULE 0.13).
pub fn prepare(task: &TaskSpec, kit_root: &Path) -> Result<AgentInvocation> {
    if task.task.role.is_empty() {
        return Err(anyhow!("task.role is empty"));
    }
    let agent_id = resolve_agent_id(task)?;
    let role_file = require_spawnable_role(kit_root, &task.task.role)?;
    let resolved = resolve_role(kit_root, &task.task.role)?;
    let prompt = compose_prompt(task, kit_root)?;
    let subagent_type = role_file
        .role
        .claude_subagent_type
        .clone()
        .unwrap_or_else(|| default_subagent_type(&task.task.role));
    let dna = compose_dna(task, &agent_id, &resolved);
    Ok(AgentInvocation {
        agent_id: agent_id.clone(),
        role: task.task.role.clone(),
        prompt,
        subagent_type,
        isolation: default_isolation(&task.task.role),
        description: build_description(&task.task.role, &agent_id),
        verify_command: build_verify_command(&agent_id),
        ledger_row: build_ledger_row_with_id(task, &agent_id),
        dna,
    })
}

/// Auto-generate agent-id if absent, otherwise reuse + validate.
/// Format: `ag-<role-slug>-<unix-ms-hex>-<4hex-rand>`. Orchestrator can still
/// pre-allocate via `kei-ledger fork` for deterministic id.
fn resolve_agent_id(task: &TaskSpec) -> Result<String> {
    let id = if task.task.agent_id.is_empty() {
        autogen_agent_id(&task.task.role)
    } else {
        task.task.agent_id.clone()
    };
    validate_agent_id(&id).map_err(|e| anyhow!("agent-id rejected: {e}"))?;
    Ok(id)
}

/// Load role metadata and assert it is spawnable (RULE 0.13 guard).
fn require_spawnable_role(kit_root: &Path, role: &str) -> Result<RoleFile> {
    let role_file = load_role_meta(kit_root, role)?;
    if !role_file.role.spawnable {
        return Err(anyhow!(
            "role '{}' is NOT spawnable (per RULE 0.13 git-ops is \
             orchestrator-only) — refusing to prepare Agent tool invocation",
            role
        ));
    }
    Ok(role_file)
}

/// Compose Layer G DNA fingerprint using the effective (resolved) agent-id.
fn compose_dna(task: &TaskSpec, agent_id: &str, resolved: &crate::role::ResolvedRole) -> String {
    let mut task_for_dna = task.clone();
    task_for_dna.task.agent_id = agent_id.to_string();
    Dna::compose(&task_for_dna, resolved).render()
}

/// Human-readable block — copy into Claude Code's Agent-tool dialog.
pub fn render_human(inv: &AgentInvocation) -> String {
    let iso = inv.isolation.as_deref().unwrap_or("<none>");
    let mut out = String::new();
    out.push_str("=== AGENT SUBSTRATE v1 — PREPARED SPAWN ===\n");
    out.push_str(&format!("agent-id: {}\n", inv.agent_id));
    out.push_str(&format!("dna: {}\n", inv.dna));
    out.push_str(&format!("subagent_type: {}\n", inv.subagent_type));
    out.push_str(&format!("isolation: {iso}\n"));
    out.push_str(&format!("description: {}\n", inv.description));
    out.push_str("\n--- PROMPT (copy into Agent tool `prompt` param) ---\n");
    out.push_str(&inv.prompt);
    if !inv.prompt.ends_with('\n') {
        out.push('\n');
    }
    out.push_str("--- END PROMPT ---\n\n");
    out.push_str("on return:\n");
    out.push_str(&format!("  {}\n", inv.verify_command));
    out.push_str("  (orchestrator harness returns worktree path in the task-notification)\n\n");
    out.push_str(&format!("ledger: {}\n", inv.ledger_row));
    out
}

pub fn render_json(inv: &AgentInvocation) -> Result<String> {
    serde_json::to_string_pretty(inv).context("serialize AgentInvocation to JSON")
}

pub fn render_toml(inv: &AgentInvocation) -> Result<String> {
    toml::to_string_pretty(inv).context("serialize AgentInvocation to TOML")
}

fn default_isolation(role: &str) -> Option<String> {
    match role {
        "edit-local" | "edit-shared" => Some("worktree".into()),
        _ => None,
    }
}

fn default_subagent_type(role: &str) -> String {
    match role {
        "edit-local" | "edit-shared" => "code-implementer",
        "explorer" => "Explore",
        "read-only" => "critic",
        _ => "critic",
    }
    .into()
}

fn build_description(role: &str, agent_id: &str) -> String {
    let short = agent_id.split('-').take(2).collect::<Vec<_>>().join("-");
    format!("{role} agent {short}")
}

fn build_verify_command(agent_id: &str) -> String {
    format!(
        "kei-agent-runtime verify tasks/{id}/task.toml \
         --worktree <path-from-harness>",
        id = agent_id
    )
}

fn build_ledger_row_with_id(task: &TaskSpec, agent_id: &str) -> String {
    let parent = task
        .task
        .parent_agent
        .as_deref()
        .filter(|s| !s.is_empty())
        .unwrap_or("none");
    format!(
        "running agent-id={} role={} parent={}",
        agent_id, task.task.role, parent
    )
}

fn load_role_meta(kit_root: &Path, role: &str) -> Result<RoleFile> {
    let path = kit_root.join("_roles").join(format!("{role}.toml"));
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("read role file {}", path.display()))?;
    toml::from_str::<RoleFile>(&text)
        .with_context(|| format!("parse role TOML {}", path.display()))
}

#[derive(Debug, Deserialize)]
struct RoleFile {
    #[serde(default)]
    role: RoleMeta,
}

#[derive(Debug, Default, Deserialize)]
struct RoleMeta {
    #[serde(default = "spawnable_default")]
    spawnable: bool,
    #[serde(default, rename = "claude-subagent-type")]
    claude_subagent_type: Option<String>,
}

fn spawnable_default() -> bool {
    true
}
