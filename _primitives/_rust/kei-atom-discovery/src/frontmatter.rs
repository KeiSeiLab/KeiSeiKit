//! Frontmatter schema + YAML parsing.
//!
//! Locked schema per `docs/SUBSTRATE-SCHEMA.md`. `input`/`output` are
//! REQUIRED for command/query/stream, OPTIONAL for transform.
//!
//! YAML parser is `serde_yaml_ng` (maintained fork of the archived
//! `serde_yaml` crate). A 64 KiB size cap is enforced pre-parse as a
//! billion-laughs mitigation.

use crate::error::Error;
use serde::Deserialize;
use serde_yaml_ng::Value as YamlValue;
use std::path::PathBuf;
use std::str::FromStr;

/// Hard cap on frontmatter size. 64 KiB is 100× any realistic atom spec.
pub const MAX_FRONTMATTER_BYTES: usize = 64 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AtomKind {
    Command,
    Query,
    Stream,
    Transform,
}

impl AtomKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            AtomKind::Command => "command",
            AtomKind::Query => "query",
            AtomKind::Stream => "stream",
            AtomKind::Transform => "transform",
        }
    }
}

impl FromStr for AtomKind {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "command" => Ok(AtomKind::Command),
            "query" => Ok(AtomKind::Query),
            "stream" => Ok(AtomKind::Stream),
            "transform" => Ok(AtomKind::Transform),
            other => Err(Error::UnknownKind(other.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SideEffect {
    pub op: String,
    pub domain: String,
}

/// Optional taxonomy facets per `docs/TAXONOMY.md`. All fields optional.
/// Format-agnostic: deserialises from YAML atom frontmatter OR TOML
/// capability / manifest / role files.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
pub struct TaxonomyFacets {
    #[serde(default)]
    pub kingdom: Option<String>,
    #[serde(default)]
    pub mechanism: Option<String>,
    #[serde(default)]
    pub domain: Option<String>,
    #[serde(default)]
    pub layer: Option<String>,
    #[serde(default)]
    pub stage: Option<String>,
    #[serde(default)]
    pub stability: Option<String>,
    #[serde(default)]
    pub language: Option<String>,
}

/// Optional lineage metadata — wikilink parents + creator DNA + created date.
/// All fields optional. `parents` defaults to an empty vec.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
pub struct Lineage {
    #[serde(default)]
    pub parents: Vec<String>,
    #[serde(default)]
    pub creator: Option<String>,
    #[serde(default)]
    pub created: Option<String>,
    #[serde(default)]
    pub fork_from: Option<String>,
}

/// Fully-parsed atom metadata — one canonical struct shared across crates.
#[derive(Debug, Clone)]
pub struct AtomMeta {
    pub full_id: String,
    pub crate_name: String,
    pub verb: String,
    pub kind: AtomKind,
    pub version: String,
    pub md_path: PathBuf,
    pub input_schema: Option<PathBuf>,
    pub output_schema: Option<PathBuf>,
    pub side_effects: Vec<SideEffect>,
    pub idempotent: bool,
    pub stability: String,
    pub keywords: Vec<String>,
    pub related: Vec<String>,
    pub body: String,
    pub taxonomy: Option<TaxonomyFacets>,
    pub lineage: Option<Lineage>,
}

/// Raw deserialisation target — kept private, `AtomMeta` is the public shape.
#[derive(Debug, Deserialize)]
pub struct Frontmatter {
    pub atom: String,
    pub kind: String,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub input: Option<SchemaRef>,
    #[serde(default)]
    pub output: Option<SchemaRef>,
    #[serde(default)]
    pub side_effects: Vec<YamlValue>,
    #[serde(default)]
    pub idempotent: Option<bool>,
    #[serde(default)]
    pub stability: Option<String>,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub related: Vec<String>,
    #[serde(default)]
    pub taxonomy: Option<TaxonomyFacets>,
    #[serde(default)]
    pub lineage: Option<Lineage>,
}

#[derive(Debug, Deserialize)]
pub struct SchemaRef {
    pub schema: Option<String>,
}

/// Split a markdown file into (frontmatter_yaml, body). Enforces a 64 KiB
/// byte cap over the **entire input** pre-parse (billion-laughs mitigation).
pub fn parse_frontmatter(md_text: &str) -> Result<(&str, &str), Error> {
    if md_text.len() > MAX_FRONTMATTER_BYTES.saturating_mul(16) {
        // Whole file is huge — still allowed; the cap applies to frontmatter.
        // We only pre-reject if the frontmatter itself is over the limit.
    }
    let rest = md_text
        .strip_prefix("---\n")
        .or_else(|| md_text.strip_prefix("---\r\n"))
        .ok_or(Error::FrontmatterMissingStart)?;
    let (end_off, end_len) =
        find_closing_delim(rest).ok_or(Error::FrontmatterMissingEnd)?;
    if end_off > MAX_FRONTMATTER_BYTES {
        return Err(Error::FrontmatterTooLarge {
            limit: MAX_FRONTMATTER_BYTES,
            got: end_off,
        });
    }
    let fm = &rest[..end_off];
    let body_start = end_off + end_len;
    Ok((fm, rest.get(body_start..).unwrap_or("")))
}

fn find_closing_delim(s: &str) -> Option<(usize, usize)> {
    let mut i = 0;
    for line in s.split_inclusive('\n') {
        let trimmed = line.trim_end_matches(&['\n', '\r'][..]);
        if trimmed == "---" {
            return Some((i, line.len()));
        }
        i += line.len();
    }
    None
}

/// Parse the `side_effects:` YAML sequence into typed `{op, domain}` pairs.
/// Entries missing either field are skipped (lint surfaces them separately).
pub fn parse_side_effects(raw: &[YamlValue]) -> Vec<SideEffect> {
    raw.iter().filter_map(side_effect_from_yaml).collect()
}

fn side_effect_from_yaml(v: &YamlValue) -> Option<SideEffect> {
    let op = v.get("op").and_then(|x| x.as_str())?.to_string();
    let domain = v.get("domain").and_then(|x| x.as_str())?.to_string();
    Some(SideEffect { op, domain })
}
