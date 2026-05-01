//! Action[] → kei-spawn task.toml[] emitter.
//!
//! One file per Action. Filename pattern:
//!   `<source-stem>-<action-id>-<slug>.toml`
//!
//! Schema mirrors what kei-spawn consumes: `[task]`, `[scope]`, `[body]`.
//! Native emitter only — research adapter could shell out to kei-decision
//! if available, but the unified shape removes that need.

use anyhow::{Context, Result};
use serde::Serialize;
use std::path::{Path, PathBuf};

use crate::normalizer::Action;

#[derive(Debug, Clone, Serialize)]
pub struct EmitOutput {
    pub action_id: String,
    pub source_format: String,
    pub path: PathBuf,
    pub bytes: usize,
}

/// Emit a list of Actions, one task.toml per Action, under `out_dir`.
pub fn emit_all(actions: &[Action], out_dir: &Path) -> Result<Vec<EmitOutput>> {
    std::fs::create_dir_all(out_dir)
        .with_context(|| format!("create dir {}", out_dir.display()))?;
    let source_stem = derive_source_stem(actions);
    actions
        .iter()
        .map(|a| emit_one(a, out_dir, &source_stem))
        .collect()
}

fn derive_source_stem(actions: &[Action]) -> String {
    let path = actions
        .first()
        .map(|a| a.source_path.as_str())
        .unwrap_or("source");
    Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("source")
        .to_string()
}

fn emit_one(action: &Action, out_dir: &Path, source_stem: &str) -> Result<EmitOutput> {
    let slug = make_slug(&action.title);
    let file_name = format!("{}-{}-{}.toml", source_stem, action.id, slug);
    let path = out_dir.join(&file_name);
    let body = build_task_toml(action);
    std::fs::write(&path, &body).with_context(|| format!("write {}", path.display()))?;
    Ok(EmitOutput {
        action_id: action.id.clone(),
        source_format: action.source_format.clone(),
        path,
        bytes: body.len(),
    })
}

fn build_task_toml(action: &Action) -> String {
    let mut s = String::new();
    s.push_str(&format!(
        "# Auto-emitted by kei-decompose from {}\n",
        action.source_path
    ));
    s.push_str(&format!(
        "# source-format: {}  source-line: {}\n",
        action.source_format, action.source_line
    ));
    s.push_str(&format!(
        "# severity: {}  effort-hint: {}\n\n",
        action.severity.as_str(),
        action.effort_hint
    ));
    s.push_str("[task]\n");
    s.push_str(&format!("role = \"{}\"\n", role_for_format(&action.source_format)));
    s.push_str(&format!("description = {}\n", toml_escape(&action.title)));
    s.push_str("\n[scope]\n");
    s.push_str("files-whitelist = [\n");
    for w in whitelist_for(&action.source_format, &action.title) {
        s.push_str(&format!("  {},\n", toml_escape(&w)));
    }
    s.push_str("]\n\n");
    s.push_str("[body]\n");
    s.push_str(&format!("text = {}\n", toml_escape(&action.body)));
    s
}

fn role_for_format(format: &str) -> &'static str {
    match format {
        "research" | "audit" | "architecture" | "new-project" => "code-implementer",
        "sleep" => "code-implementer",
        _ => "code-implementer",
    }
}

fn whitelist_for(format: &str, title: &str) -> Vec<String> {
    match format {
        "research" => guess_research_whitelist(title),
        "audit" => vec!["<TODO-orchestrator-fill: target of audit fix>".into()],
        "architecture" => vec!["<TODO-orchestrator-fill: scope of architecture change>".into()],
        "new-project" => vec!["<TODO-orchestrator-fill: project root>".into()],
        "sleep" => vec!["<TODO-orchestrator-fill: scope of sleep follow-up>".into()],
        _ => vec!["<TODO-orchestrator-fill>".into()],
    }
}

/// Heuristic whitelist for research actions: pick the longest run of letters
/// after `kei-` if the title mentions a primitive, else punt to TODO.
fn guess_research_whitelist(title: &str) -> Vec<String> {
    let lower = title.to_lowercase();
    if let Some(after) = lower.split("kei-").nth(1) {
        let s: String = after
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '-')
            .collect();
        if !s.is_empty() {
            let trimmed = s.trim_end_matches('-').to_string();
            return vec![format!("_primitives/_rust/kei-{}/**", trimmed)];
        }
    }
    vec!["<TODO-orchestrator-fill: scope of research action>".into()]
}

/// Build a 1-8-token kebab slug from the title.
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
    if collapsed.is_empty() {
        "action".to_string()
    } else {
        collapsed
    }
}

/// Use serde_json string escape for safety, then return TOML basic-string.
fn toml_escape(s: &str) -> String {
    serde_json::to_string(s).unwrap_or_else(|_| format!("\"{}\"", s.replace('"', "\\\"")))
}
