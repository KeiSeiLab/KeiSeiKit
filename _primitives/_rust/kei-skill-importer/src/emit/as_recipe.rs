//! Emit `ImportedSkill` as a KeiSeiKit recipe TOML DAG.
//!
//! Recipe schema (defined in this wave — see `recipes/<name>.toml`):
//!
//! ```toml
//! [recipe]
//! name = "..."
//! description = "..."
//! imported_from = "openclaw://create-npm-package"
//! imported_at = "2026-04-25T..."
//!
//! [[steps]]
//! id = "step-1"
//! atom = "kei-cortex::chat"
//! input = { ... }
//! depends_on = []
//! ```
//!
//! Steps are derived by walking each phase in order and emitting one
//! `[[steps]]` block per detected `atom_call` whose `atom_id` resolves.

use crate::canonical::{AtomCall, ImportedSkill};
use anyhow::{Context, Result};
use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Serialize)]
struct RecipeFile {
    recipe: RecipeMeta,
    steps: Vec<Step>,
}

#[derive(Serialize)]
struct RecipeMeta {
    name: String,
    description: String,
    imported_from: String,
    imported_at: String,
    source_format: String,
}

#[derive(Serialize)]
struct Step {
    id: String,
    atom: String,
    input: toml::Value,
    depends_on: Vec<String>,
    raw_command: String,
}

/// Render `skill` as recipe TOML text. Side-effect-free.
pub fn render(skill: &ImportedSkill) -> Result<String> {
    let recipe = build_recipe(skill);
    let body =
        toml::to_string_pretty(&recipe).context("serialize recipe to TOML")?;
    Ok(body)
}

/// Render + write to `<output_dir>/recipes/<name>.toml`.
pub fn write(skill: &ImportedSkill, output_dir: &Path) -> Result<PathBuf> {
    let text = render(skill)?;
    let dir = output_dir.join("recipes");
    std::fs::create_dir_all(&dir)
        .with_context(|| format!("create_dir_all {}", dir.display()))?;
    let file = dir.join(format!("{}.toml", recipe_filename(skill)));
    std::fs::write(&file, text)
        .with_context(|| format!("write {}", file.display()))?;
    Ok(file)
}

fn build_recipe(skill: &ImportedSkill) -> RecipeFile {
    let imported_from = format!(
        "{}://{}",
        skill.source_format.as_str(),
        recipe_filename(skill)
    );
    let imported_at = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

    let mut steps: Vec<Step> = Vec::new();
    let mut step_idx: usize = 1;
    let mut last_id: Option<String> = None;
    for phase in &skill.phases {
        for call in &phase.atom_calls {
            let Some(atom_id) = call.atom_id.clone() else {
                continue;
            };
            let id = format!("step-{step_idx}");
            let depends_on = last_id.iter().cloned().collect::<Vec<_>>();
            steps.push(Step {
                id: id.clone(),
                atom: atom_id,
                input: derive_input(call),
                depends_on,
                raw_command: call.raw_command.clone(),
            });
            last_id = Some(id);
            step_idx += 1;
        }
    }

    RecipeFile {
        recipe: RecipeMeta {
            name: skill.name.clone(),
            description: skill.description.clone(),
            imported_from,
            imported_at,
            source_format: skill.source_format.as_str().to_string(),
        },
        steps,
    }
}

fn derive_input(call: &AtomCall) -> toml::Value {
    let mut table = toml::map::Map::new();
    let mut parts = call.raw_command.split_whitespace();
    let _ = parts.next(); // primitive name
    let _ = parts.next(); // verb
    let args: Vec<String> = parts.map(|s| s.to_string()).collect();
    table.insert(
        "args".to_string(),
        toml::Value::Array(args.into_iter().map(toml::Value::String).collect()),
    );
    toml::Value::Table(table)
}

fn recipe_filename(skill: &ImportedSkill) -> String {
    let raw = skill.name.trim().to_ascii_lowercase();
    let mut out = String::with_capacity(raw.len());
    let mut prev_dash = false;
    for c in raw.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c);
            prev_dash = false;
        } else if (c == '-' || c == '_' || c == ' ') && !prev_dash {
            out.push('-');
            prev_dash = true;
        }
    }
    let t = out.trim_matches('-').to_string();
    if t.is_empty() {
        "imported-recipe".into()
    } else {
        t
    }
}
