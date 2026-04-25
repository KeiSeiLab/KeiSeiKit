//! Claude Code `SKILL.md` parser (KeiSeiKit native).
//!
//! Format reference (verified against `skills/pet-init/SKILL.md` and
//! `skills/onboard/SKILL.md` in this repo):
//!
//! - File: `skills/<name>/SKILL.md` (the index) plus optional
//!   `phase-<n>-*.md` siblings.
//! - Frontmatter: YAML with `name`, `description`, optional
//!   `argument-hint`, `category`.
//! - Body: H2 sections; the canonical wizard pattern uses an explicit
//!   "Pipeline overview" table that lists `phase-*.md` references —
//!   we DO NOT recurse into siblings here (parser is single-file);
//!   instead we map H2 sections to phases, same as OpenClaw.
//! - Tools detection: `## References` and `## Rules` sections often
//!   list primitives by name; the classifier picks them up.

use crate::canonical::{ImportedSkill, Phase, SourceFormat};
use crate::parsers::{detect_language, split_frontmatter};
use anyhow::{Context, Result};
use serde::Deserialize;
use serde_yaml_ng::Value as YamlValue;
use std::path::Path;

#[derive(Debug, Default, Deserialize)]
struct Front {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    category: Option<String>,
    #[serde(default, rename = "argument-hint")]
    argument_hint: Option<String>,
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
        .unwrap_or_else(|| "unnamed-claude-skill".into());
    let description = front
        .description
        .clone()
        .or_else(|| extract_first_paragraph(body))
        .unwrap_or_else(|| "(no description)".into());

    let mut tags = Vec::new();
    if let Some(cat) = &front.category {
        tags.push(format!("category:{cat}"));
    }
    if let Some(hint) = &front.argument_hint {
        tags.push(format!("arg:{hint}"));
    }

    let phases = split_h2_sections(body, &name);

    Ok(ImportedSkill {
        name,
        description,
        source_format: SourceFormat::ClaudeCode,
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
        serde_yaml_ng::from_str(fm_text).context("claude frontmatter yaml")?;
    let front: Front = serde_yaml_ng::from_value(raw.clone())
        .context("claude frontmatter shape")?;
    Ok((front, Some(raw)))
}

fn derive_name_from_path(path: &Path) -> Option<String> {
    let parent = path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|s| s.to_str())
        .map(|s| s.to_string());
    parent.filter(|n| !n.is_empty() && n != "skills")
}

fn extract_first_paragraph(body: &str) -> Option<String> {
    body.split("\n\n")
        .map(str::trim)
        .find(|p| !p.is_empty() && !p.starts_with('#'))
        .map(|p| p.lines().next().unwrap_or(p).to_string())
}

fn split_h2_sections(body: &str, fallback_name: &str) -> Vec<Phase> {
    let mut out: Vec<Phase> = Vec::new();
    let mut current: Option<String> = None;
    let mut buf = String::new();
    for line in body.lines() {
        if let Some(rest) = line.strip_prefix("## ") {
            flush(&mut out, &mut current, &mut buf);
            current = Some(rest.trim().to_string());
            continue;
        }
        buf.push_str(line);
        buf.push('\n');
    }
    flush(&mut out, &mut current, &mut buf);
    if out.is_empty() {
        out.push(Phase {
            name: fallback_name.to_string(),
            body: body.to_string(),
            atom_calls: Vec::new(),
        });
    }
    out
}

fn flush(out: &mut Vec<Phase>, name: &mut Option<String>, buf: &mut String) {
    let body = std::mem::take(buf);
    if let Some(n) = name.take() {
        if !body.trim().is_empty() {
            out.push(Phase { name: n, body, atom_calls: Vec::new() });
        }
    }
}
