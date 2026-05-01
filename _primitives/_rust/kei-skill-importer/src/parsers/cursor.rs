//! Cursor `.mdc` parser.
//!
//! Format research (verified 2026-04-25 via Cursor docs page extraction):
//!
//! - Files: `<repo>/.cursor/rules/<name>.mdc` (modern) OR
//!   `<repo>/.cursorrules` (legacy, plain text). We support `.mdc` here.
//! - Frontmatter: REQUIRED YAML. Common keys:
//!     description: "Short description shown in rule selector"
//!     globs: ["**/*.tsx", "src/**/*.ts"]   # file glob scope
//!     alwaysApply: false                    # auto-include in every prompt
//! - Body: free-form markdown rule content.
//! - No phases convention; rules are flat. We synthesize one phase.

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
    globs: Vec<String>,
    #[serde(default, rename = "alwaysApply")]
    always_apply: Option<bool>,
}

pub fn parse(path: &Path) -> Result<ImportedSkill> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("read {}", path.display()))?;
    let (fm_text, body) = split_frontmatter(&text);
    let (front, raw_yaml) = parse_front(fm_text)?;

    let name = derive_name_from_path(path).unwrap_or_else(|| "unnamed-cursor-rule".into());
    let description = front
        .description
        .clone()
        .unwrap_or_else(|| "(no description)".into());

    let mut tags = Vec::new();
    for g in &front.globs {
        tags.push(format!("glob:{g}"));
    }
    if front.always_apply.unwrap_or(false) {
        tags.push("alwaysApply".into());
    }

    let phases = vec![Phase {
        name: name.clone(),
        body: body.to_string(),
        atom_calls: Vec::new(),
    }];

    Ok(ImportedSkill {
        name,
        description,
        source_format: SourceFormat::Cursor,
        source_path: path.to_path_buf(),
        language: detect_language(body),
        tags,
        phases,
        tools_required: Vec::new(),
        yaml_frontmatter: raw_yaml,
        body: body.to_string(),
    })
}

fn parse_front(fm_text: &str) -> Result<(Front, Option<YamlValue>)> {
    if fm_text.trim().is_empty() {
        return Ok((Front::default(), None));
    }
    let raw: YamlValue =
        serde_yaml_ng::from_str(fm_text).context("cursor frontmatter yaml")?;
    let front: Front = serde_yaml_ng::from_value(raw.clone())
        .context("cursor frontmatter shape")?;
    Ok((front, Some(raw)))
}

fn derive_name_from_path(path: &Path) -> Option<String> {
    path.file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.trim_start_matches("cursor-").to_string())
}
