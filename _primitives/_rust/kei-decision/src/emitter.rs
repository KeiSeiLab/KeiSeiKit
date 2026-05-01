//! `RankedAction → task.toml` emitter (kei-spawn-compatible).
//!
//! Each emitted file is a minimal kei-spawn task with three sections:
//!
//!   [task]     role + description + body-from-master-line
//!   [scope]    files-whitelist guessed from kind
//!   [body]     long-form text (the action title + source-line ref)
//!
//! The orchestrator can edit the file before passing to `kei-spawn spawn`.

use anyhow::{Context, Result};
use serde::Serialize;
use std::path::{Path, PathBuf};

use crate::classifier::ActionKind;
use crate::ranker::RankedAction;

#[derive(Debug, Clone, Serialize)]
pub struct EmitOutput {
    pub action_id: String,
    pub path: PathBuf,
    pub bytes: usize,
}

/// Emit one task.toml under `out_dir`. Returns its path + size.
pub fn emit_task_toml(action: &RankedAction, out_dir: &Path, master: &Path) -> Result<EmitOutput> {
    std::fs::create_dir_all(out_dir)
        .with_context(|| format!("create dir {}", out_dir.display()))?;
    let slug = make_slug(&action.raw.title);
    let file_name = format!("action-{}-{}.toml", action.raw.id, slug);
    let path = out_dir.join(&file_name);
    let body = build_body(action, master);
    std::fs::write(&path, &body).with_context(|| format!("write {}", path.display()))?;
    Ok(EmitOutput {
        action_id: action.raw.id.clone(),
        path,
        bytes: body.len(),
    })
}

fn build_body(action: &RankedAction, master: &Path) -> String {
    let role = role_for_kind(action.kind);
    let whitelist = whitelist_for_kind(action.kind, &action.raw.title);
    let mut s = String::new();
    s.push_str(&format!("# Auto-emitted by kei-decision from {}\n", master.display()));
    s.push_str(&format!("# Source line: {}\n", action.raw.source_line));
    s.push_str(&format!("# Score: {:.3}  Rank: {}\n\n", action.score, action.rank));
    s.push_str("[task]\n");
    s.push_str(&format!("role = \"{}\"\n", role));
    s.push_str(&format!("description = {}\n", toml_escape(&action.raw.title)));
    s.push_str("\n[scope]\n");
    s.push_str("files-whitelist = [\n");
    for w in &whitelist {
        s.push_str(&format!("  {},\n", toml_escape(w)));
    }
    s.push_str("]\n\n");
    s.push_str("[body]\n");
    s.push_str(&format!("text = {}\n", toml_escape(&render_body_text(action))));
    s
}

fn render_body_text(action: &RankedAction) -> String {
    let mut t = String::new();
    t.push_str(&format!("Action #{} (kind: {:?})\n\n", action.raw.id, action.kind));
    t.push_str(&format!("Title: {}\n", action.raw.title));
    t.push_str(&format!("Severity: {}\n", action.raw.severity));
    t.push_str(&format!("Effort: {}\n", action.raw.effort));
    if !action.raw.deps.is_empty() {
        t.push_str(&format!("Dependencies: {}\n", action.raw.deps.join(", ")));
    }
    t.push_str("\nThis task was auto-generated from a /research MASTER-REPORT.md actionable-plan row. ");
    t.push_str("The orchestrator should review the [scope] whitelist before spawning.");
    t
}

fn role_for_kind(kind: ActionKind) -> &'static str {
    match kind {
        ActionKind::Refactor => "code-implementer",
        ActionKind::Migrate => "code-implementer",
        ActionKind::NewPrimitive => "code-implementer",
        ActionKind::Decompose => "code-implementer",
        ActionKind::Doc => "doc-writer",
        ActionKind::Unknown => "code-implementer",
    }
}

/// Best-effort whitelist guess. Conservative — orchestrator should verify.
fn whitelist_for_kind(kind: ActionKind, title: &str) -> Vec<String> {
    match kind {
        ActionKind::NewPrimitive => {
            let slug = primitive_slug_from_title(title);
            vec![format!("_primitives/_rust/kei-{}/**", slug)]
        }
        ActionKind::Decompose => {
            // Decompose targets a specific monolith — leave placeholder.
            vec!["<TODO-orchestrator-fill: file or dir to decompose>".to_string()]
        }
        ActionKind::Refactor | ActionKind::Migrate => {
            vec!["<TODO-orchestrator-fill: scope of refactor/migrate>".to_string()]
        }
        ActionKind::Doc => vec!["docs/**".to_string(), "README.md".to_string()],
        ActionKind::Unknown => vec!["<TODO-orchestrator-fill>".to_string()],
    }
}

/// Heuristic: pick the longest run of letters after "kei-" or after "primitive ".
fn primitive_slug_from_title(title: &str) -> String {
    let lower = title.to_lowercase();
    if let Some(after) = lower.split("kei-").nth(1) {
        let s: String = after.chars().take_while(|c| c.is_alphanumeric() || *c == '-').collect();
        if !s.is_empty() {
            return s.trim_end_matches('-').to_string();
        }
    }
    if let Some(after) = lower.split("primitive ").nth(1) {
        let s: String = after.chars().take_while(|c| c.is_alphanumeric() || *c == '-').collect();
        if !s.is_empty() {
            return s.trim_end_matches('-').to_string();
        }
    }
    "TODO-name".to_string()
}

/// `make_slug("Refactor 4 hooks to call kei-leak-matrix") → "refactor-4-hooks-to-call-kei-leak"`.
fn make_slug(title: &str) -> String {
    let lower = title.to_lowercase();
    let cleaned: String = lower
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect();
    let collapsed: String = cleaned
        .split('-')
        .filter(|p| !p.is_empty())
        .take(8)
        .collect::<Vec<&str>>()
        .join("-");
    if collapsed.is_empty() { "action".to_string() } else { collapsed }
}

/// Use serde_json string escape for safety, then wrap in TOML basic-string quotes.
fn toml_escape(s: &str) -> String {
    serde_json::to_string(s).unwrap_or_else(|_| format!("\"{}\"", s.replace('"', "\\\"")))
}
