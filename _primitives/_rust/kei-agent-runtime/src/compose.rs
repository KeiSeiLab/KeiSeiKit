//! Compose capability-fragment prompt for an agent invocation.
//!
//! Flow:
//!   1. Parse `task.toml` → `TaskSpec` (caller does this).
//!   2. Resolve `_roles/<task.role>.toml` via `role::resolve_role`
//!      (handles `extends` / `relaxes` / cycle detection).
//!   3. For each capability in the resolved required list, read the
//!      `_capabilities/<category>/<slug>/text.md` fragment.
//!   4. Concatenate fragments with `\n\n---\n\n`.
//!   5. Append `task.body.text`.

use crate::capability::TaskSpec;
use crate::role::{resolve_role, validate_name};
use anyhow::{anyhow, Context, Result};
use std::path::Path;

const SEPARATOR: &str = "\n\n---\n\n";

/// Compose prompt text. `kit_root` is the repo root that holds `_roles/`
/// and `_capabilities/` directories.
///
/// Order: capability fragments → resolved scope block → task body.
/// The scope block makes whitelist/denylist/verification params visible
/// to the agent — capability text references "your task's scope" generically;
/// without this block the agent has no way to know concrete paths.
pub fn compose_prompt(task: &TaskSpec, kit_root: &Path) -> Result<String> {
    if task.task.role.is_empty() {
        return Err(anyhow!("task.role is empty"));
    }
    let resolved = resolve_role(kit_root, &task.task.role)?;
    let mut fragments: Vec<String> = Vec::with_capacity(resolved.required.len() + 2);
    for cap_name in &resolved.required {
        let frag = load_capability_text(kit_root, cap_name)
            .with_context(|| format!("capability {cap_name}"))?;
        fragments.push(frag);
    }
    let scope_block = render_scope_block(task);
    if !scope_block.is_empty() {
        fragments.push(scope_block);
    }
    if !task.body.text.trim().is_empty() {
        fragments.push(task.body.text.clone());
    }
    Ok(fragments.join(SEPARATOR))
}

fn render_scope_block(task: &TaskSpec) -> String {
    let mut lines = vec!["## Your task's scope (resolved from task.toml)".to_string()];
    if !task.scope.files_whitelist.is_empty() {
        lines.push(String::new());
        lines.push("**files-whitelist** (you MAY Edit/Write these):".to_string());
        for p in &task.scope.files_whitelist {
            lines.push(format!("- `{p}`"));
        }
    }
    if !task.scope.files_denylist.is_empty() {
        lines.push(String::new());
        lines.push("**files-denylist** (you MUST NOT Edit/Write these):".to_string());
        for p in &task.scope.files_denylist {
            lines.push(format!("- `{p}`"));
        }
    }
    if !task.verification.cargo_check_crates.is_empty() {
        lines.push(String::new());
        lines.push(format!(
            "**cargo check MUST pass** for: {}",
            task.verification
                .cargo_check_crates
                .iter()
                .map(|c| format!("`{c}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if !task.verification.cargo_test_crates.is_empty() {
        lines.push(format!(
            "**cargo test MUST pass** for: {}",
            task.verification
                .cargo_test_crates
                .iter()
                .map(|c| format!("`{c}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if let Some(n) = task.verification.test_count_min {
        if n > 0 {
            lines.push(format!("**minimum test count:** {n}"));
        }
    }
    if !task.output.report_fields_required.is_empty() {
        lines.push(String::new());
        lines.push(format!(
            "**report MUST include fields:** {}",
            task.output
                .report_fields_required
                .iter()
                .map(|f| format!("`{f}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if lines.len() == 1 {
        return String::new();
    }
    lines.join("\n")
}

fn load_capability_text(kit_root: &Path, cap_name: &str) -> Result<String> {
    let (category, slug) = split_cap_name(cap_name)?;
    let path = kit_root
        .join("_capabilities")
        .join(category)
        .join(slug)
        .join("text.md");
    std::fs::read_to_string(&path)
        .with_context(|| format!("read capability text {}", path.display()))
}

fn split_cap_name(cap: &str) -> Result<(&str, &str)> {
    let (cat, slug) = cap
        .split_once("::")
        .filter(|(c, s)| !c.is_empty() && !s.is_empty())
        .ok_or_else(|| anyhow!("malformed capability name '{cap}' — expected <cat>::<slug>"))?;
    // Block path traversal: both halves are joined into a filesystem path,
    // so any `..`, `/`, `\`, upper-case, etc. is refused at the gate.
    validate_name("capability-category", cat)?;
    validate_name("capability-slug", slug)?;
    Ok((cat, slug))
}
