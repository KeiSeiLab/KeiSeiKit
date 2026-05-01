//! Facet-query over capability.toml + manifest .toml primitives.
//!
//! TX1 adds `[taxonomy]` + `[lineage]` sections to primitive TOMLs.
//! This module walks a capabilities root (`<root>/*/*/capability.toml`)
//! and a manifests root (`<root>/*.toml`), parses the taxonomy section,
//! and filters by `key=value` AND predicates.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// A primitive's identity + its taxonomy facets.
#[derive(Debug, Clone)]
pub struct PrimitiveFacets {
    pub full_id: String,
    pub source: PathBuf,
    pub facets: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct CapabilityDoc {
    capability: Option<CapabilityHead>,
    #[serde(default)]
    taxonomy: Option<BTreeMap<String, toml::Value>>,
}

#[derive(Debug, Deserialize)]
struct CapabilityHead {
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ManifestDoc {
    name: Option<String>,
    #[serde(default)]
    taxonomy: Option<BTreeMap<String, toml::Value>>,
}

#[derive(Debug, Deserialize)]
struct RoleDoc {
    role: Option<RoleHead>,
    #[serde(default)]
    taxonomy: Option<BTreeMap<String, toml::Value>>,
}

#[derive(Debug, Deserialize)]
struct RoleHead {
    name: Option<String>,
}

/// Parse a single TOML file into a `PrimitiveFacets`, or `None` if it's
/// unparseable or has no discoverable id. Tries capability, then role,
/// then flat manifest form.
pub fn parse_primitive(path: &Path) -> Result<Option<PrimitiveFacets>> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("read {}", path.display()))?;
    if let Some(p) = parse_capability(&text, path) {
        return Ok(Some(p));
    }
    if let Some(p) = parse_role(&text, path) {
        return Ok(Some(p));
    }
    Ok(parse_manifest(&text, path))
}

fn parse_capability(text: &str, path: &Path) -> Option<PrimitiveFacets> {
    let doc: CapabilityDoc = toml::from_str(text).ok()?;
    let id = doc.capability.as_ref().and_then(|c| c.name.clone())?;
    let facets = flatten_facets(doc.taxonomy.as_ref());
    Some(PrimitiveFacets { full_id: id, source: path.to_path_buf(), facets })
}

fn parse_manifest(text: &str, path: &Path) -> Option<PrimitiveFacets> {
    let doc: ManifestDoc = toml::from_str(text).ok()?;
    let id = doc.name?;
    let facets = flatten_facets(doc.taxonomy.as_ref());
    Some(PrimitiveFacets { full_id: id, source: path.to_path_buf(), facets })
}

fn parse_role(text: &str, path: &Path) -> Option<PrimitiveFacets> {
    let doc: RoleDoc = toml::from_str(text).ok()?;
    let name = doc.role.as_ref().and_then(|r| r.name.clone())?;
    let facets = flatten_facets(doc.taxonomy.as_ref());
    Some(PrimitiveFacets {
        full_id: format!("role::{name}"),
        source: path.to_path_buf(),
        facets,
    })
}

fn flatten_facets(tax: Option<&BTreeMap<String, toml::Value>>) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    let Some(map) = tax else { return out };
    for (k, v) in map {
        if let Some(s) = value_to_string(v) {
            out.insert(k.clone(), s);
        }
    }
    out
}

fn value_to_string(v: &toml::Value) -> Option<String> {
    match v {
        toml::Value::String(s) => Some(s.clone()),
        toml::Value::Integer(i) => Some(i.to_string()),
        toml::Value::Boolean(b) => Some(b.to_string()),
        _ => None,
    }
}

/// Walk capabilities + manifests roots and return all parseable primitives.
/// Silently skips files that fail to parse (lint is a separate concern).
pub fn discover_primitives(cap_root: &Path, man_root: &Path) -> Vec<PrimitiveFacets> {
    discover_primitives_with_roles(cap_root, man_root, None)
}

/// Same as `discover_primitives`, but also walks an optional roles root
/// (`_roles/*.toml`). Role entries emit id `role::<name>`.
pub fn discover_primitives_with_roles(
    cap_root: &Path,
    man_root: &Path,
    roles_root: Option<&Path>,
) -> Vec<PrimitiveFacets> {
    let mut out = Vec::new();
    walk_capabilities(cap_root, &mut out);
    walk_manifests(man_root, &mut out);
    if let Some(r) = roles_root {
        walk_roles(r, &mut out);
    }
    out
}

fn walk_capabilities(root: &Path, out: &mut Vec<PrimitiveFacets>) {
    if !root.is_dir() {
        return;
    }
    for entry in WalkDir::new(root).max_depth(4).follow_links(false).into_iter().flatten() {
        if entry.file_name() == "capability.toml" && entry.path().is_file() {
            if let Ok(Some(p)) = parse_primitive(entry.path()) {
                out.push(p);
            }
        }
    }
}

fn walk_manifests(root: &Path, out: &mut Vec<PrimitiveFacets>) {
    if !root.is_dir() {
        return;
    }
    for entry in WalkDir::new(root).max_depth(2).follow_links(false).into_iter().flatten() {
        let p = entry.path();
        if p.is_file() && p.extension().and_then(|s| s.to_str()) == Some("toml") {
            if let Ok(Some(pf)) = parse_primitive(p) {
                out.push(pf);
            }
        }
    }
}

fn walk_roles(root: &Path, out: &mut Vec<PrimitiveFacets>) {
    if !root.is_dir() {
        return;
    }
    for entry in WalkDir::new(root).max_depth(2).follow_links(false).into_iter().flatten() {
        let p = entry.path();
        if p.is_file() && p.extension().and_then(|s| s.to_str()) == Some("toml") {
            if let Ok(Some(pf)) = parse_primitive(p) {
                out.push(pf);
            }
        }
    }
}

/// Parse `k=v` filter strings into pairs. Bad entries (no `=`) are dropped.
pub fn parse_filters(raw: &[String]) -> Vec<(String, String)> {
    raw.iter()
        .filter_map(|s| s.split_once('=').map(|(k, v)| (k.to_string(), v.to_string())))
        .collect()
}

/// AND-filter: a primitive matches iff ALL `(k, v)` pairs are present and equal.
/// Missing facet key → not a match (None != specific value).
pub fn matches_all(p: &PrimitiveFacets, filters: &[(String, String)]) -> bool {
    filters.iter().all(|(k, v)| p.facets.get(k).map(|s| s == v).unwrap_or(false))
}

