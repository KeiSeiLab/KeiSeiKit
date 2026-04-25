//! Cline rules parser.
//!
//! Format research (verified 2026-04-25 via Cline docs page metadata):
//!
//! - Files live under `.clinerules/*.md` in the project root.
//! - Frontmatter: optional YAML — common keys are `description`,
//!   `paths` (glob array — file scope filter; NOT a tools-required list),
//!   `tags`. The `paths:` key is a SCOPE FILTER (which files this rule
//!   applies to), not an invocation list.
//! - Body: free-form markdown. No standard phase convention; many rules
//!   are flat single-message instructions. We map the entire body to
//!   one synthetic phase.
//! - Bash code-fences are uncommon but valid; classifier picks them up.

use crate::canonical::{ImportedSkill, Phase, SourceFormat};
use crate::parsers::{detect_language, split_frontmatter};
use anyhow::{Context, Result};
use serde::Deserialize;
use serde_yaml_ng::Value as YamlValue;
use std::path::Path;

#[derive(Debug, Default, Deserialize)]
struct Front {
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    paths: Vec<String>,
    #[serde(default)]
    tags: Vec<String>,
}

pub fn parse(path: &Path) -> Result<ImportedSkill> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("read {}", path.display()))?;
    let (fm_text, body) = split_frontmatter(&text);
    let (front, raw_yaml) = parse_front(fm_text)?;

    let name = front
        .name
        .clone()
        .or_else(|| derive_name_from_path(path))
        .unwrap_or_else(|| "unnamed-cline-rule".into());
    let description = front
        .description
        .clone()
        .or_else(|| extract_first_paragraph(body))
        .unwrap_or_else(|| "(no description)".into());

    let mut tags = front.tags.clone();
    // `paths` is scope filter, but we surface it as a tag for visibility.
    for p in &front.paths {
        tags.push(format!("paths:{p}"));
    }

    let phases = vec![Phase {
        name: name.clone(),
        body: body.to_string(),
        atom_calls: Vec::new(),
    }];

    Ok(ImportedSkill {
        name,
        description,
        source_format: SourceFormat::Cline,
        source_path: path.to_path_buf(),
        language: detect_language(body),
        tags,
        phases,
        tools_required: Vec::new(), // Cline rules have no tools-required field
        yaml_frontmatter: raw_yaml,
        body: body.to_string(),
    })
}

fn parse_front(fm_text: &str) -> Result<(Front, Option<YamlValue>)> {
    if fm_text.trim().is_empty() {
        return Ok((Front::default(), None));
    }
    let raw: YamlValue =
        serde_yaml_ng::from_str(fm_text).context("cline frontmatter yaml")?;
    let front: Front = serde_yaml_ng::from_value(raw.clone())
        .context("cline frontmatter shape")?;
    Ok((front, Some(raw)))
}

fn derive_name_from_path(path: &Path) -> Option<String> {
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())?;
    Some(stem.trim_start_matches("cline-").to_string())
}

fn extract_first_paragraph(body: &str) -> Option<String> {
    body.split("\n\n")
        .map(str::trim)
        .find(|p| !p.is_empty() && !p.starts_with('#'))
        .map(|p| p.lines().next().unwrap_or(p).to_string())
}
