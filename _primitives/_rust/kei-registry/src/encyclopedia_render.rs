//! Markdown section builders for the encyclopedia renderer.
//!
//! Constructor Pattern: one cube for string-building, no I/O, no SQL.
//! Called exclusively by `encyclopedia::render_markdown`.

use std::collections::BTreeMap;

use crate::block::{Block, BlockType};
use crate::encyclopedia::EncyclopediaEntry;
use crate::encyclopedia_time::utc_now;

pub fn push_header(out: &mut String, total: u64, counts: &BTreeMap<String, u64>) {
    out.push_str("# KeiSeiKit DNA Encyclopedia\n\n");
    out.push_str(&format!(
        "> Auto-generated from kei-registry. Last regenerated: {}.\n",
        utc_now()
    ));
    out.push_str(&format!("> Total blocks: {total}. Per-type breakdown:\n\n"));
    out.push_str("| Type | Count |\n|---|---:|\n");
    for (t, c) in counts {
        out.push_str(&format!("| {t} | {c} |\n"));
    }
    out.push_str("\n---\n\n");
}

pub fn push_section(out: &mut String, bt: &BlockType, entries: &[&EncyclopediaEntry]) {
    let label = capitalise(bt.as_str());
    out.push_str(&format!("## {label} ({})\n\n", entries.len()));
    out.push_str("Sorted alphabetically by name.\n\n");
    match bt {
        BlockType::Hook => push_hook_table(out, entries),
        BlockType::Skill => push_skill_table(out, entries),
        BlockType::Rule => push_rule_section(out, entries),
        _ => push_default_table(out, entries),
    }
    out.push('\n');
}

pub fn push_supersede_chains(out: &mut String, all_blocks: &[Block]) {
    let mut by_name: BTreeMap<String, Vec<&Block>> = BTreeMap::new();
    for b in all_blocks {
        by_name.entry(b.name.clone()).or_default().push(b);
    }
    let chains: Vec<_> = by_name.iter().filter(|(_, v)| v.len() >= 2).collect();
    if chains.is_empty() {
        return;
    }
    out.push_str("---\n\n## Supersede chains\n\n");
    for (name, versions) in chains {
        let shas: Vec<&str> = versions.iter().map(|b| b.body_sha.as_str()).collect();
        out.push_str(&format!(
            "- `{name}` — {} versions: {}\n",
            versions.len(),
            shas.join(" → ")
        ));
    }
    out.push('\n');
}

pub fn push_schema_notes(out: &mut String) {
    out.push_str("---\n\n## Schema notes\n\n");
    out.push_str(
        "- `dna` wire format: \
         `<block_type>::<caps>::<scope_sha8>::<body_sha8>-<nonce8>` (80 chars).\n",
    );
    out.push_str("- Active vs superseded: rows where `superseded_by IS NULL` are active.\n");
    out.push_str(
        "- See `_primitives/_rust/kei-shared/src/dna.rs` for canonical DNA spec.\n",
    );
}

// ── private table builders ─────────────────────────────────────────────────

fn push_default_table(out: &mut String, entries: &[&EncyclopediaEntry]) {
    out.push_str("| Name | DNA prefix | Path | Body sha8 |\n|---|---|---|---|\n");
    for e in entries {
        out.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            e.name,
            dna_prefix(&e.dna),
            short_path(&e.path),
            e.body_sha,
        ));
    }
}

fn push_skill_table(out: &mut String, entries: &[&EncyclopediaEntry]) {
    out.push_str("| Name | Caps | DNA prefix | Path |\n|---|---|---|---|\n");
    for e in entries {
        out.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            e.name,
            e.caps,
            dna_prefix(&e.dna),
            short_path(&e.path),
        ));
    }
}

fn push_hook_table(out: &mut String, entries: &[&EncyclopediaEntry]) {
    out.push_str("| Name | Event | DNA prefix | Path |\n|---|---|---|---|\n");
    for e in entries {
        let event = if e.caps.is_empty() { "—" } else { &e.caps };
        out.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            e.name,
            event,
            dna_prefix(&e.dna),
            short_path(&e.path),
        ));
    }
}

fn push_rule_section(out: &mut String, entries: &[&EncyclopediaEntry]) {
    let mut grouped: BTreeMap<String, Vec<&EncyclopediaEntry>> = BTreeMap::new();
    for e in entries {
        grouped.entry(rule_slug(&e.name)).or_default().push(e);
    }
    for (slug, rows) in &grouped {
        out.push_str(&format!("### {slug}\n\n"));
        out.push_str("| Section | DNA prefix | Body sha8 |\n|---|---|---|\n");
        for e in rows {
            out.push_str(&format!(
                "| {} | {} | {} |\n",
                e.name,
                dna_prefix(&e.dna),
                e.body_sha,
            ));
        }
        out.push('\n');
    }
}

// ── utilities ──────────────────────────────────────────────────────────────

pub fn dna_prefix(dna: &str) -> String {
    let n = dna.len().min(20);
    format!("{}…", &dna[..n])
}

pub fn short_path(path: &str) -> &str {
    // Strip absolute-path prefix so the rendered table is workspace-relative.
    // `_blocks/`, `_manifests/`, `_generated/`, `_atoms/`, `agents/`,
    // `_assembler/`, `docs/` were missing — the column then leaked the
    // maintainer's `/Users/<user>/Projects/KeiSeiKit-public/` prefix into
    // the public encyclopedia (107 atom rows in v0.17 DNA-INDEX).
    for prefix in &[
        "_primitives",
        "_blocks/",
        "_manifests/",
        "_generated/",
        "_atoms/",
        "_assembler/",
        "_roles/",
        "_capabilities/",
        "skills/",
        "hooks/",
        "rules/",
        "agents/",
        "docs/",
    ] {
        if let Some(pos) = path.find(prefix) {
            return &path[pos..];
        }
    }
    path
}

pub fn rule_slug(name: &str) -> String {
    let parts: Vec<&str> = name.splitn(3, '-').collect();
    if parts.len() >= 2 {
        format!("{}-{}", parts[0], parts[1])
    } else {
        name.to_string()
    }
}

pub fn capitalise(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::short_path;

    #[test]
    fn short_path_strips_blocks_prefix() {
        let abs = "/srv/ci/build/_blocks/api-anthropic.md";
        assert_eq!(short_path(abs), "_blocks/api-anthropic.md");
    }

    #[test]
    fn short_path_strips_primitives_prefix() {
        let abs = "/srv/ci/_primitives/_rust/kei-registry/Cargo.toml";
        assert_eq!(short_path(abs), "_primitives/_rust/kei-registry/Cargo.toml");
    }

    #[test]
    fn short_path_strips_manifests_prefix() {
        let abs = "/srv/ci/build/_manifests/ml-implementer.toml";
        assert_eq!(short_path(abs), "_manifests/ml-implementer.toml");
    }

    #[test]
    fn short_path_strips_agents_prefix() {
        let abs = "/srv/ci/build/agents/researcher.md";
        assert_eq!(short_path(abs), "agents/researcher.md");
    }

    #[test]
    fn short_path_passthrough_unknown() {
        let p = "some/relative/random.md";
        assert_eq!(short_path(p), p);
    }

    #[test]
    fn short_path_no_absolute_leak_for_blocks() {
        // Fixture uses a CI-style absolute path (no username component) so
        // the source file itself does not contain a maintainer-shaped path
        // that would trip the local pre-commit hook + the leak-check CI.
        let abs = "/srv/ci/build/_blocks/api-fal-ai.md";
        let out = short_path(abs);
        assert!(!out.starts_with('/'), "still absolute: {out}");
        assert!(!out.contains("/srv/"), "not stripped: {out}");
        assert_eq!(out, "_blocks/api-fal-ai.md");
    }

    #[test]
    fn short_path_strips_roles_prefix() {
        let abs = "/x/Projects/KeiSeiKit-public/_roles/auditor.toml";
        assert_eq!(short_path(abs), "_roles/auditor.toml");
    }

    #[test]
    fn short_path_strips_capabilities_prefix() {
        let abs = "/x/Projects/KeiSeiKit-public/_capabilities/output/verdict/capability.toml";
        assert_eq!(
            short_path(abs),
            "_capabilities/output/verdict/capability.toml"
        );
    }
}
