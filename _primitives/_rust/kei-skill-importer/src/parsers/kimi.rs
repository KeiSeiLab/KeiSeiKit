//! Kimi CLI agent-spec parser.
//!
//! Format research (verified 2026-04-25 via WebFetch
//! `https://raw.githubusercontent.com/MoonshotAI/kimi-cli/main/AGENTS.md`):
//!
//! - Two file types coexist:
//!     1. `AGENTS.md` — root-level architecture map (markdown body).
//!     2. `src/kimi_cli/agents/<name>.yaml` — agent spec (pure YAML)
//!        with keys: `name`, `description`, `extend`, `tools`,
//!        `subagents`, `system_prompt` (often inline multiline string).
//! - For YAML specs, the entire file is the frontmatter; body = empty
//!   OR pulled from `system_prompt` (multiline literal scalar).
//! - For markdown AGENTS.md, behaviour matches OpenClaw (H2 sections).
//!
//! This parser handles BOTH shapes: detect by extension. `.yaml`/`.yml`
//! → spec mode; otherwise → markdown mode (delegates to internal H2 split).

use crate::canonical::{ImportedSkill, Phase, SourceFormat};
use crate::parsers::{detect_language, split_frontmatter};
use anyhow::{Context, Result};
use serde::Deserialize;
use serde_yaml_ng::Value as YamlValue;
use std::path::Path;

#[derive(Debug, Default, Deserialize)]
struct Spec {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    extend: Option<String>,
    #[serde(default)]
    tools: Vec<String>,
    #[serde(default)]
    subagents: Vec<String>,
    #[serde(default)]
    system_prompt: Option<String>,
}

pub fn parse(path: &Path) -> Result<ImportedSkill> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("read {}", path.display()))?;
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if ext == "yaml" || ext == "yml" {
        parse_yaml_spec(&text, path)
    } else {
        parse_markdown(&text, path)
    }
}

fn parse_yaml_spec(text: &str, path: &Path) -> Result<ImportedSkill> {
    let raw: YamlValue =
        serde_yaml_ng::from_str(text).context("kimi yaml spec")?;
    let spec: Spec = serde_yaml_ng::from_value(raw.clone())
        .context("kimi spec shape")?;
    let name = spec
        .name
        .clone()
        .or_else(|| derive_name_from_path(path))
        .unwrap_or_else(|| "unnamed-kimi-agent".into());
    let description = spec
        .description
        .clone()
        .unwrap_or_else(|| "(no description)".into());

    let body = spec.system_prompt.clone().unwrap_or_default();
    let phases = vec![Phase {
        name: name.clone(),
        body: body.clone(),
        atom_calls: Vec::new(),
    }];
    let mut tags = Vec::new();
    if let Some(parent) = &spec.extend {
        tags.push(format!("extend:{parent}"));
    }
    for sa in &spec.subagents {
        tags.push(format!("subagent:{sa}"));
    }

    Ok(ImportedSkill {
        name,
        description,
        source_format: SourceFormat::Kimi,
        source_path: path.to_path_buf(),
        language: detect_language(&body),
        tags,
        phases,
        tools_required: spec.tools,
        yaml_frontmatter: Some(raw),
        body,
    })
}

fn parse_markdown(text: &str, path: &Path) -> Result<ImportedSkill> {
    let (fm_text, body) = split_frontmatter(text);
    let raw_yaml = if fm_text.trim().is_empty() {
        None
    } else {
        Some(serde_yaml_ng::from_str::<YamlValue>(fm_text).context("kimi md frontmatter")?)
    };
    let spec: Spec = match &raw_yaml {
        Some(v) => serde_yaml_ng::from_value(v.clone()).unwrap_or_default(),
        None => Spec::default(),
    };

    let name = spec
        .name
        .clone()
        .or_else(|| derive_name_from_path(path))
        .unwrap_or_else(|| "unnamed-kimi-md".into());
    let description = spec
        .description
        .clone()
        .or_else(|| extract_first_paragraph(body))
        .unwrap_or_else(|| "(no description)".into());

    let phases = split_h2_sections(body, &name);

    Ok(ImportedSkill {
        name,
        description,
        source_format: SourceFormat::Kimi,
        source_path: path.to_path_buf(),
        language: detect_language(body),
        tags: Vec::new(),
        phases,
        tools_required: spec.tools,
        yaml_frontmatter: raw_yaml,
        body: body.to_string(),
    })
}

fn derive_name_from_path(path: &Path) -> Option<String> {
    path.file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.trim_start_matches("kimi-").to_string())
}

fn extract_first_paragraph(body: &str) -> Option<String> {
    body.split("\n\n")
        .map(str::trim)
        .find(|p| !p.is_empty() && !p.starts_with('#'))
        .map(|p| p.lines().next().unwrap_or(p).to_string())
}

fn split_h2_sections(body: &str, fallback: &str) -> Vec<Phase> {
    let mut out: Vec<Phase> = Vec::new();
    let mut name: Option<String> = None;
    let mut buf = String::new();
    for line in body.lines() {
        if let Some(rest) = line.strip_prefix("## ") {
            flush(&mut out, &mut name, &mut buf);
            name = Some(rest.trim().to_string());
            continue;
        }
        buf.push_str(line);
        buf.push('\n');
    }
    flush(&mut out, &mut name, &mut buf);
    if out.is_empty() {
        out.push(Phase {
            name: fallback.to_string(),
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
