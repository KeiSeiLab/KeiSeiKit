//! Lineage traversal for primitive TOMLs.
//!
//! Parses `[lineage]` section of capability.toml + manifest TOMLs,
//! extracting `parents` wikilinks, `created-by`, `fork-from`. Builds
//! an in-memory directed graph and walks ancestors + descendants.

use anyhow::{Context, Result};
use kei_atom_discovery::parse_wikilink;
use serde::Deserialize;
use std::collections::{BTreeMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Lineage metadata for a single primitive.
#[derive(Debug, Clone)]
pub struct LineageNode {
    pub id: String,
    pub source: PathBuf,
    pub parents: Vec<String>,
    pub created_by: Option<String>,
    pub fork_from: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CapDoc {
    capability: Option<IdHead>,
    #[serde(default)]
    lineage: Option<LineageSection>,
}

#[derive(Debug, Deserialize)]
struct ManDoc {
    name: Option<String>,
    #[serde(default)]
    lineage: Option<LineageSection>,
}

#[derive(Debug, Deserialize)]
struct IdHead {
    name: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
struct LineageSection {
    #[serde(default)]
    parents: Vec<String>,
    #[serde(rename = "created-by", default)]
    created_by: Option<String>,
    #[serde(rename = "fork-from", default)]
    fork_from: Option<String>,
    #[serde(rename = "created-at", default)]
    created_at: Option<String>,
}

/// Parse a single TOML into a `LineageNode`, or `None` if unidentifiable.
pub fn parse_lineage(path: &Path) -> Result<Option<LineageNode>> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("read {}", path.display()))?;
    if let Some(n) = parse_cap_lineage(&text, path) {
        return Ok(Some(n));
    }
    Ok(parse_man_lineage(&text, path))
}

fn parse_cap_lineage(text: &str, path: &Path) -> Option<LineageNode> {
    let doc: CapDoc = toml::from_str(text).ok()?;
    let id = doc.capability.as_ref().and_then(|c| c.name.clone())?;
    Some(build_node(id, path, doc.lineage))
}

fn parse_man_lineage(text: &str, path: &Path) -> Option<LineageNode> {
    let doc: ManDoc = toml::from_str(text).ok()?;
    let id = doc.name?;
    Some(build_node(id, path, doc.lineage))
}

fn build_node(id: String, path: &Path, lin: Option<LineageSection>) -> LineageNode {
    let lin = lin.unwrap_or_default();
    let parents = lin.parents.iter().filter_map(|w| parse_wikilink(w)).collect();
    LineageNode {
        id,
        source: path.to_path_buf(),
        parents,
        created_by: lin.created_by,
        fork_from: lin.fork_from,
        created_at: lin.created_at,
    }
}

/// Walk capabilities + manifests roots and parse every lineage node.
pub fn discover_lineage(cap_root: &Path, man_root: &Path) -> Vec<LineageNode> {
    let mut out = Vec::new();
    walk_root(cap_root, "capability.toml", 4, &mut out);
    walk_manifest_root(man_root, &mut out);
    out
}

fn walk_root(root: &Path, fname: &str, depth: usize, out: &mut Vec<LineageNode>) {
    if !root.is_dir() {
        return;
    }
    for e in WalkDir::new(root).max_depth(depth).follow_links(false).into_iter().flatten() {
        if e.file_name() == fname && e.path().is_file() {
            if let Ok(Some(n)) = parse_lineage(e.path()) {
                out.push(n);
            }
        }
    }
}

fn walk_manifest_root(root: &Path, out: &mut Vec<LineageNode>) {
    if !root.is_dir() {
        return;
    }
    for e in WalkDir::new(root).max_depth(2).follow_links(false).into_iter().flatten() {
        let p = e.path();
        if p.is_file() && p.extension().and_then(|s| s.to_str()) == Some("toml") {
            if let Ok(Some(n)) = parse_lineage(p) {
                out.push(n);
            }
        }
    }
}

/// Traversal result: ancestors (via parents + fork-from) and descendants.
#[derive(Debug, Clone, Default)]
pub struct LineageTrace {
    pub focus: Option<LineageNode>,
    pub ancestors: Vec<String>,
    pub descendants: Vec<String>,
}

/// BFS ancestors (follow parents + fork_from) + descendants (inverse edges).
pub fn trace_lineage(nodes: &[LineageNode], id: &str, depth: usize) -> LineageTrace {
    let by_id: BTreeMap<&str, &LineageNode> = nodes.iter().map(|n| (n.id.as_str(), n)).collect();
    LineageTrace {
        focus: by_id.get(id).map(|n| (*n).clone()),
        ancestors: bfs_up(&by_id, id, depth),
        descendants: bfs_down(nodes, id, depth),
    }
}

fn bfs_up(by_id: &BTreeMap<&str, &LineageNode>, start: &str, depth: usize) -> Vec<String> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<(String, usize)> = VecDeque::new();
    queue.push_back((start.to_string(), 0));
    let mut out = Vec::new();
    while let Some((cur, d)) = queue.pop_front() {
        if d >= depth { continue; }
        let Some(n) = by_id.get(cur.as_str()) else { continue };
        let mut parents = n.parents.clone();
        if let Some(f) = &n.fork_from { parents.push(f.clone()); }
        for p in parents {
            if seen.insert(p.clone()) {
                out.push(p.clone());
                queue.push_back((p, d + 1));
            }
        }
    }
    out
}

fn bfs_down(nodes: &[LineageNode], start: &str, depth: usize) -> Vec<String> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut frontier: Vec<String> = vec![start.to_string()];
    let mut out = Vec::new();
    for _ in 0..depth {
        let mut next: Vec<String> = Vec::new();
        for n in nodes {
            let is_child = n.parents.iter().any(|p| frontier.contains(p))
                || n.fork_from.as_ref().is_some_and(|f| frontier.contains(f));
            if is_child && seen.insert(n.id.clone()) {
                out.push(n.id.clone());
                next.push(n.id.clone());
            }
        }
        if next.is_empty() { break; }
        frontier = next;
    }
    out
}

/// Filter + sort nodes by a creator id, return most-recent first (by created_at).
pub fn nodes_by_author(nodes: &[LineageNode], creator: &str, limit: usize) -> Vec<LineageNode> {
    let mut matched: Vec<LineageNode> = nodes
        .iter()
        .filter(|n| n.created_by.as_deref() == Some(creator))
        .cloned()
        .collect();
    matched.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    matched.truncate(limit);
    matched
}

