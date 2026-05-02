use crate::types::{dna_prefix, sanitize_id, truncate_chars, Node};
use anyhow::Result;
use rusqlite::Connection;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct ManifestInfo {
    pub node_id: String,
    pub name: String,
    pub blocks: Vec<String>,
    pub path_refs: Vec<String>,
    pub rule_blocks: Vec<String>,
}

pub struct AgentInfo {
    pub node_id: String,
    pub branch: String,
    pub fork_parent_id: Option<String>,
}

pub fn collect_blocks(reg: &Connection) -> Result<(Vec<Node>, HashMap<String, String>)> {
    // Pull `path` too so we can derive the file-stem slug (e.g.
    // `_blocks/baseline.md` → `baseline`). Manifests reference blocks by
    // this short slug, while the `name` column stores display names like
    // "BASELINE — inherit from Main Claude". The lookup must accept both.
    let mut stmt = reg.prepare(
        "SELECT dna, block_type, name, path FROM blocks WHERE superseded_by IS NULL",
    )?;
    let mut nodes = Vec::new();
    let mut lookup: HashMap<String, String> = HashMap::new();
    let rows = stmt.query_map([], |r| {
        Ok((
            r.get::<_, String>(0)?,
            r.get::<_, String>(1)?,
            r.get::<_, String>(2)?,
            r.get::<_, String>(3)?,
        ))
    })?;
    for row in rows {
        let (dna, btype, name, path) = row?;
        let id = dna_prefix(&dna);
        // Primary lookup: full display name (e.g. used by some manifests).
        lookup.insert(name.clone(), id.clone());
        // Secondary lookup: lowercased name (case-insensitive resolve).
        lookup.insert(name.to_lowercase(), id.clone());
        // Tertiary lookup: file-stem slug — the canonical reference form
        // in `_manifests/*.toml` blocks=[] / rule_blocks=[].
        if let Some(stem) = std::path::Path::new(&path)
            .file_stem()
            .and_then(|s| s.to_str())
        {
            lookup.insert(stem.to_string(), id.clone());
            lookup.insert(stem.to_lowercase(), id.clone());
        }
        nodes.push(Node {
            id,
            title: format!("{}: {}", btype, truncate_chars(&name, 60)),
            kind: btype.clone(),
            category: btype,
            tags: vec![],
            connections: 0,
            extra: HashMap::new(),
        });
    }
    Ok((nodes, lookup))
}

type AgentRow = (String, Option<String>, Option<String>, Option<String>,
                 Option<String>, Option<String>, Option<i64>, Option<i64>,
                 Option<i64>, Option<String>);

pub fn collect_agents(led: &Connection, nodes: &mut Vec<Node>) -> Result<Vec<AgentInfo>> {
    let mut stmt = led.prepare(
        "SELECT id, branch, parent_branch, model, outcome, status, \
         cost_micro_cents, tokens_in, tokens_out, fork_parent_id FROM agents",
    )?;
    let mut agents = Vec::new();
    let rows = stmt.query_map([], |r| Ok((
        r.get::<_, String>(0)?, r.get::<_, Option<String>>(1)?,
        r.get::<_, Option<String>>(2)?, r.get::<_, Option<String>>(3)?,
        r.get::<_, Option<String>>(4)?, r.get::<_, Option<String>>(5)?,
        r.get::<_, Option<i64>>(6)?, r.get::<_, Option<i64>>(7)?,
        r.get::<_, Option<i64>>(8)?, r.get::<_, Option<String>>(9)?,
    )))?;
    for row in rows {
        let info = push_agent_node(row?, nodes);
        agents.push(info);
    }
    Ok(agents)
}

fn push_agent_node(row: AgentRow, nodes: &mut Vec<Node>) -> AgentInfo {
    let (id, branch, _par, model, outcome, status, cost, tin, tout, fork_pid) = row;
    let branch = branch.unwrap_or_default();
    let model_s = model.unwrap_or_default();
    let outcome_s = outcome.unwrap_or_default();
    let status_s = status.unwrap_or_default();
    let cost_usd = cost.unwrap_or(0) as f64 / 1_000_000.0;
    let node_id = sanitize_id(&id);
    let mut extra = HashMap::new();
    extra.insert("model".to_string(), model_s.clone());
    extra.insert("cost_usd".to_string(), format!("{:.4}", cost_usd));
    extra.insert("outcome".to_string(), outcome_s.clone());
    extra.insert("status".to_string(), status_s.clone());
    extra.insert("tokens_in".to_string(), tin.unwrap_or(0).to_string());
    extra.insert("tokens_out".to_string(), tout.unwrap_or(0).to_string());
    nodes.push(Node {
        id: node_id.clone(),
        title: format!("agent: {} ({}, {})", truncate_chars(&branch, 40), model_s, outcome_s),
        kind: "agent".to_string(),
        category: model_s,
        tags: vec![outcome_s, status_s],
        connections: 0,
        extra,
    });
    AgentInfo { node_id, branch, fork_parent_id: fork_pid }
}

pub fn collect_manifests(dir: &PathBuf, nodes: &mut Vec<Node>) -> Result<Vec<ManifestInfo>> {
    let mut manifests = Vec::new();
    let Ok(rd) = std::fs::read_dir(dir) else { return Ok(manifests) };
    for entry in rd.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("toml") { continue; }
        let stem = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
        let content = std::fs::read_to_string(&path).unwrap_or_default();
        let val: toml::Value = toml::from_str(&content)
            .unwrap_or(toml::Value::Table(Default::default()));
        let blocks = extract_str_array(&val, "blocks");
        let rule_blocks = extract_str_array(&val, "rule_blocks");
        let path_refs = extract_path_refs(&val);
        let substrate = val.get("substrate_role")
            .and_then(|v| v.as_str()).unwrap_or("agent").to_string();
        let node_id = format!("manifest::{}", sanitize_id(&stem));
        nodes.push(Node {
            id: node_id.clone(),
            title: format!("manifest: {}", stem),
            kind: "manifest".to_string(),
            category: substrate,
            tags: vec![],
            connections: 0,
            extra: HashMap::new(),
        });
        manifests.push(ManifestInfo { node_id, name: stem, blocks, path_refs, rule_blocks });
    }
    Ok(manifests)
}

pub fn collect_branches(led: &Connection, nodes: &mut Vec<Node>) -> Result<()> {
    let mut stmt = led.prepare(
        "SELECT branch, parent_branch FROM agents WHERE branch IS NOT NULL",
    )?;
    let mut seen = std::collections::HashSet::new();
    let rows = stmt.query_map([], |r| {
        Ok((r.get::<_, Option<String>>(0)?, r.get::<_, Option<String>>(1)?))
    })?;
    for row in rows {
        let (branch, parent) = row?;
        let mut candidates: Vec<String> = Vec::new();
        if let Some(b) = branch { candidates.push(b); }
        if let Some(p) = parent { candidates.push(p); }
        for b in candidates {
            if seen.insert(b.clone()) {
                nodes.push(Node {
                    id: format!("branch::{}", sanitize_id(&b)),
                    title: format!("branch: {}", truncate_chars(&b, 50)),
                    kind: "branch".to_string(),
                    category: "branch".to_string(),
                    tags: vec![],
                    connections: 0,
                    extra: HashMap::new(),
                });
            }
        }
    }
    Ok(())
}

pub fn collect_skills(led: &Connection, nodes: &mut Vec<Node>) -> Result<()> {
    let mut stmt = led.prepare(
        "SELECT DISTINCT skill_name FROM skill_invocations WHERE skill_name IS NOT NULL",
    )?;
    let rows = stmt.query_map([], |r| r.get::<_, String>(0))?;
    for row in rows {
        let name = row?;
        nodes.push(Node {
            id: format!("skill::{}", sanitize_id(&name)),
            title: format!("skill: {}", name),
            kind: "skill".to_string(),
            category: "skill".to_string(),
            tags: vec![],
            connections: 0,
            extra: HashMap::new(),
        });
    }
    Ok(())
}

fn extract_str_array(val: &toml::Value, key: &str) -> Vec<String> {
    val.get(key).and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default()
}

fn extract_path_refs(val: &toml::Value) -> Vec<String> {
    let extras = val.get("references")
        .and_then(|r| r.get("extra"))
        .and_then(|e| e.as_array());
    let Some(arr) = extras else { return vec![] };
    arr.iter().filter_map(|v| v.as_str())
        .filter(|s| s.starts_with("path:"))
        .map(|s| {
            let after = &s["path:".len()..];
            after.split('/').next().unwrap_or(after).to_string()
        })
        .collect()
}

