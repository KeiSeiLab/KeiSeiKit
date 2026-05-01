//! OpenClaw format parser.
//!
//! Format research (verified 2026-04-25 via WebFetch
//! `https://raw.githubusercontent.com/openclaw/openclaw/main/AGENTS.md`):
//!
//! - File: `AGENTS.md` (root) OR `~/.openclaw/workspace/skills/<name>/SKILL.md`.
//! - Frontmatter: optional YAML (often absent for AGENTS.md). When
//!   present, common keys: `name`, `description`, `tags`, `tools`.
//! - Body: H2-rooted sections (`## Start`, `## Map`, `## Architecture`,
//!   `## Commands`, `## Gates`, `## Code`, …) — each H2 is a "phase"
//!   in our canonical model. "Telegraph style" — terse bullet lists.
//! - Tools detection: `## Commands` section frequently lists `pnpm <verb>`
//!   bash invocations as bullets; we treat those as candidate atom calls.

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
    tags: Vec<String>,
    #[serde(default)]
    tools: Vec<String>,
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
        .unwrap_or_else(|| "unnamed-openclaw-skill".into());
    let description = front
        .description
        .clone()
        .or_else(|| extract_first_paragraph(body))
        .unwrap_or_else(|| "(no description)".into());

    let phases = split_h2_sections(body, &name);

    Ok(ImportedSkill {
        name,
        description,
        source_format: SourceFormat::OpenClaw,
        source_path: path.to_path_buf(),
        language: detect_language(body),
        tags: front.tags,
        phases,
        tools_required: front.tools,
        yaml_frontmatter: raw_yaml,
        body: body.to_string(),
    })
}

fn parse_front(fm_text: &str) -> Result<(Front, Option<YamlValue>)> {
    if fm_text.trim().is_empty() {
        return Ok((Front::default(), None));
    }
    let raw: YamlValue =
        serde_yaml_ng::from_str(fm_text).context("openclaw frontmatter yaml")?;
    let front: Front =
        serde_yaml_ng::from_value(raw.clone()).context("openclaw frontmatter shape")?;
    Ok((front, Some(raw)))
}

fn derive_name_from_path(path: &Path) -> Option<String> {
    let parent_dir = path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|s| s.to_str())
        .map(|s| s.to_string());
    parent_dir.filter(|n| !n.is_empty() && n != "skills" && n != "workspace")
}

fn extract_first_paragraph(body: &str) -> Option<String> {
    body.split("\n\n")
        .map(str::trim)
        .find(|p| !p.is_empty() && !p.starts_with('#'))
        .map(|p| p.lines().next().unwrap_or(p).to_string())
}

/// Split body into phases by H2 headings (`## Foo`). When body has zero
/// H2 sections, emit one synthetic phase named after the skill.
fn split_h2_sections(body: &str, fallback_name: &str) -> Vec<Phase> {
    let mut phases: Vec<Phase> = Vec::new();
    let mut current_name: Option<String> = None;
    let mut buf = String::new();
    for line in body.lines() {
        if let Some(rest) = line.strip_prefix("## ") {
            flush_phase(&mut phases, &mut current_name, &mut buf);
            current_name = Some(rest.trim().to_string());
            continue;
        }
        buf.push_str(line);
        buf.push('\n');
    }
    flush_phase(&mut phases, &mut current_name, &mut buf);

    if phases.is_empty() {
        phases.push(Phase {
            name: fallback_name.to_string(),
            body: body.to_string(),
            atom_calls: Vec::new(),
        });
    }
    phases
}

fn flush_phase(out: &mut Vec<Phase>, name: &mut Option<String>, buf: &mut String) {
    let body = std::mem::take(buf);
    if let Some(n) = name.take() {
        if !body.trim().is_empty() {
            out.push(Phase { name: n, body, atom_calls: Vec::new() });
        }
    }
}
