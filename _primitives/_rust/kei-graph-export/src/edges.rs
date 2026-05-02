use crate::nodes::{AgentInfo, ManifestInfo};
use crate::types::{sanitize_id, Edge};
use anyhow::Result;
use rusqlite::Connection;
use std::collections::HashMap;

// Edge rule 1: manifest → block
pub fn edges_manifest_block(
    manifests: &[ManifestInfo],
    lookup: &HashMap<String, String>,
    links: &mut Vec<Edge>,
) {
    for m in manifests {
        for name in &m.blocks {
            if let Some(id) = lookup.get(name.as_str()) {
                links.push(edge(&m.node_id, id, "block_dep", 1.0));
            }
        }
    }
}

// Edge rule 2: manifest → path-atom
pub fn edges_manifest_path_ref(
    manifests: &[ManifestInfo],
    lookup: &HashMap<String, String>,
    links: &mut Vec<Edge>,
) {
    for m in manifests {
        for name in &m.path_refs {
            if let Some(id) = lookup.get(name.as_str()) {
                links.push(edge(&m.node_id, id, "path_ref", 0.5));
            }
        }
    }
}

// Edge rule 3: manifest → rule block
pub fn edges_manifest_rule(
    manifests: &[ManifestInfo],
    lookup: &HashMap<String, String>,
    links: &mut Vec<Edge>,
) {
    for m in manifests {
        for name in &m.rule_blocks {
            if let Some(id) = lookup.get(name.as_str()) {
                links.push(edge(&m.node_id, id, "rule_dep", 0.7));
            }
        }
    }
}

// Edge rule 4: branch lineage
pub fn edges_branch_lineage(led: &Connection, links: &mut Vec<Edge>) -> Result<()> {
    let mut stmt = led.prepare(
        "SELECT branch, parent_branch FROM agents \
         WHERE branch IS NOT NULL AND parent_branch IS NOT NULL AND parent_branch != ''",
    )?;
    let rows = stmt.query_map([], |r| {
        Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))
    })?;
    for row in rows {
        let (branch, parent) = row?;
        links.push(edge(
            &format!("branch::{}", sanitize_id(&parent)),
            &format!("branch::{}", sanitize_id(&branch)),
            "branch_lineage", 1.0,
        ));
    }
    Ok(())
}

// Edge rule 5: agent fork
pub fn edges_agent_fork(agents: &[AgentInfo], links: &mut Vec<Edge>) {
    for a in agents {
        if let Some(pid) = &a.fork_parent_id {
            links.push(edge(&sanitize_id(pid), &a.node_id, "agent_fork", 1.0));
        }
    }
}

// Edge rule 6: skill invocation
pub fn edges_skill_invocation(led: &Connection, links: &mut Vec<Edge>) -> Result<()> {
    let mut stmt = led.prepare(
        "SELECT agent_id, skill_name FROM skill_invocations \
         WHERE agent_id IS NOT NULL AND skill_name IS NOT NULL",
    )?;
    let rows = stmt.query_map([], |r| {
        Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))
    })?;
    for row in rows {
        let (agent_id, skill_name) = row?;
        links.push(edge(
            &sanitize_id(&agent_id),
            &format!("skill::{}", sanitize_id(&skill_name)),
            "skill_run", 0.5,
        ));
    }
    Ok(())
}

// Edge rule 7: agent → manifest (heuristic)
pub fn edges_agent_manifest(
    agents: &[AgentInfo],
    manifests: &[ManifestInfo],
    links: &mut Vec<Edge>,
) {
    for a in agents {
        let slug = subagent_slug(&a.branch);
        if let Some(m) = manifests.iter().find(|m| m.name == slug) {
            links.push(edge(&a.node_id, &m.node_id, "agent_uses_manifest", 1.0));
        } else if !slug.is_empty() {
            eprintln!("warn: no manifest for slug '{}' (branch: {})", slug, a.branch);
        }
    }
}

fn subagent_slug(branch: &str) -> String {
    let part = branch.split('/').last().unwrap_or(branch);
    let stripped = strip_trailing_digits_and_dashes(part);
    let stripped = stripped.strip_prefix("inline-").unwrap_or(stripped);
    stripped.to_string()
}

fn strip_trailing_digits_and_dashes(s: &str) -> &str {
    let mut end = s.len();
    let bytes = s.as_bytes();
    while end > 0 && (bytes[end - 1].is_ascii_digit() || bytes[end - 1] == b'-') {
        end -= 1;
    }
    s[..end].trim_end_matches('-')
}

fn edge(src: &str, tgt: &str, kind: &str, weight: f32) -> Edge {
    Edge {
        source: src.to_string(),
        target: tgt.to_string(),
        kind: kind.to_string(),
        weight,
    }
}
