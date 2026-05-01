//! Emit `ImportedSkill` as a KeiSeiKit atom markdown.
//!
//! Produces a YAML-frontmatter `.md` file matching the shape of
//! `_primitives/_rust/kei-task/atoms/search.md`.
//!
//! Provenance: an HTML-comment line is injected immediately after the
//! frontmatter delimiter recording the source path and import time.

use crate::canonical::ImportedSkill;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Render `skill` as atom markdown text. Side-effect-free.
pub fn render(skill: &ImportedSkill) -> Result<String> {
    let atom_id = derive_atom_id(skill);
    let frontmatter = render_frontmatter(skill, &atom_id);
    let body = render_body(skill, &atom_id);
    let provenance = render_provenance(skill);
    Ok(format!("{frontmatter}\n{provenance}\n{body}"))
}

/// Render + write to `<output_dir>/atoms/<verb>.md`. Returns the
/// absolute path of the emitted file.
pub fn write(skill: &ImportedSkill, output_dir: &Path) -> Result<PathBuf> {
    let text = render(skill)?;
    let dir = output_dir.join("atoms");
    std::fs::create_dir_all(&dir)
        .with_context(|| format!("create_dir_all {}", dir.display()))?;
    let verb = derive_verb(skill);
    let file = dir.join(format!("{verb}.md"));
    std::fs::write(&file, text)
        .with_context(|| format!("write {}", file.display()))?;
    Ok(file)
}

fn derive_atom_id(skill: &ImportedSkill) -> String {
    format!("kei-imported::{}", derive_verb(skill))
}

fn derive_verb(skill: &ImportedSkill) -> String {
    let raw = skill.name.trim().to_ascii_lowercase();
    let mut verb = String::with_capacity(raw.len());
    let mut prev_dash = false;
    for c in raw.chars() {
        if c.is_ascii_alphanumeric() {
            verb.push(c);
            prev_dash = false;
        } else if (c == '-' || c == '_' || c == ' ') && !prev_dash {
            verb.push('-');
            prev_dash = true;
        }
    }
    let trimmed = verb.trim_matches('-').to_string();
    if trimmed.is_empty() {
        "imported-skill".into()
    } else {
        trimmed
    }
}

fn render_frontmatter(skill: &ImportedSkill, atom_id: &str) -> String {
    let desc_escaped = escape_yaml_double_quoted(&skill.description);
    let lang = skill.language.as_deref().unwrap_or("en");
    let mut keywords = vec!["imported".to_string(), skill.source_format.as_str().into()];
    keywords.extend(skill.tags.iter().map(|s| yaml_safe_keyword(s)));
    let keywords_inline = inline_yaml_string_array(&keywords);

    format!(
        "---\n\
         atom: {atom_id}\n\
         kind: transform\n\
         version: \"0.1.0\"\n\
         \n\
         side_effects: []\n\
         idempotent: true\n\
         stability: experimental\n\
         \n\
         language: {lang}\n\
         keywords: {keywords_inline}\n\
         related: []\n\
         \n\
         description: \"{desc_escaped}\"\n\
         ---"
    )
}

fn render_provenance(skill: &ImportedSkill) -> String {
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ");
    let src = skill.source_path.display();
    let fmt = skill.source_format.as_str();
    format!("<!-- imported from {src} (format={fmt}) at {now} -->")
}

fn render_body(skill: &ImportedSkill, atom_id: &str) -> String {
    let mut s = String::new();
    s.push_str(&format!("# {atom_id}\n\n"));
    s.push_str(&format!("{}\n\n", skill.description));
    if !skill.tools_required.is_empty() {
        s.push_str("## Tools required\n\n");
        for t in &skill.tools_required {
            s.push_str(&format!("- {t}\n"));
        }
        s.push('\n');
    }
    s.push_str("## Body (imported)\n\n");
    s.push_str(skill.body.trim());
    s.push('\n');
    s
}

fn escape_yaml_double_quoted(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', " ")
}

fn yaml_safe_keyword(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == ':' { c } else { '-' })
        .collect()
}

fn inline_yaml_string_array(items: &[String]) -> String {
    let inner = items
        .iter()
        .map(|s| format!("\"{}\"", escape_yaml_double_quoted(s)))
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{inner}]")
}
