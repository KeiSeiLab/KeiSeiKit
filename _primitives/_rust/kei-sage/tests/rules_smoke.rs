//! Integration smoke test for rule discovery + atom→rule edge persistence.
//!
//! Creates a temp rules tree with 2 rule files (flat dir), asserts
//! `discover_rules` extracts slugs + heading names correctly. Then stages
//! an atom whose `related:` lists one of those rules and asserts
//! `index_rule_edges` persists a `rule_ref` edge into the store.

use kei_sage::atoms::discover_atoms;
use kei_sage::edges::list_outgoing;
use kei_sage::rule_index::{discover_rules, index_rule_edges, index_rules};
use kei_sage::Store;
use std::fs;
use tempfile::tempdir;

const RULE_012: &str = r#"# RULE 0.12 — AGENT GIT MODEL

Body of the rule.
"#;

const RULE_MEMORY: &str = r#"# Memory Protocol

3-layer architecture.
"#;

const ATOM_A: &str = r#"---
atom: kei-task::create
kind: command
version: "0.1.0"
input:
  schema: schemas/create-input.json
output:
  schema: schemas/create-output.json
stability: stable
keywords: [task]
related:
  - "[[rules/RULE 0.12]]"
  - "[[rules/memory-protocol]]"
---
# kei-task::create

Body.
"#;

fn write_rule(root: &std::path::Path, slug: &str, body: &str) {
    fs::create_dir_all(root).unwrap();
    fs::write(root.join(format!("{slug}.md")), body).unwrap();
}

fn write_atom(root: &std::path::Path, crate_name: &str, verb: &str, body: &str) {
    let atoms_dir = root.join(crate_name).join("atoms");
    fs::create_dir_all(&atoms_dir).unwrap();
    fs::write(atoms_dir.join(format!("{verb}.md")), body).unwrap();
}

#[test]
fn discover_rules_returns_two_records_with_correct_slugs_and_names() {
    let tmp = tempdir().unwrap();
    write_rule(tmp.path(), "agent-git-model", RULE_012);
    write_rule(tmp.path(), "memory-protocol", RULE_MEMORY);

    let recs = discover_rules(tmp.path()).unwrap();
    assert_eq!(recs.len(), 2, "expected 2 rules, got {}", recs.len());

    let by_slug: std::collections::HashMap<_, _> =
        recs.iter().map(|r| (r.slug.as_str(), r.name.as_str())).collect();
    assert_eq!(
        by_slug.get("agent-git-model"),
        Some(&"RULE 0.12 — AGENT GIT MODEL")
    );
    assert_eq!(by_slug.get("memory-protocol"), Some(&"Memory Protocol"));
}

#[test]
fn index_rules_persists_rule_units() {
    let tmp = tempdir().unwrap();
    write_rule(tmp.path(), "memory-protocol", RULE_MEMORY);

    let recs = discover_rules(tmp.path()).unwrap();
    let store = Store::open_memory().unwrap();
    let n = index_rules(&store, &recs).unwrap();
    assert_eq!(n, 1);
    assert_eq!(store.count_units().unwrap(), 1);
}

#[test]
fn index_rule_edges_persists_atom_to_rule() {
    let tmp_rules = tempdir().unwrap();
    let tmp_atoms = tempdir().unwrap();

    // 1 rule file; the atom references `rules/RULE 0.12` → slug "0.12".
    write_rule(tmp_rules.path(), "agent-git-model", RULE_012);
    write_atom(tmp_atoms.path(), "kei-task", "create", ATOM_A);

    let rule_recs = discover_rules(tmp_rules.path()).unwrap();
    let atom_recs = discover_atoms(tmp_atoms.path()).unwrap();

    let store = Store::open_memory().unwrap();
    index_rules(&store, &rule_recs).unwrap();
    let edges_written = index_rule_edges(&store, &atom_recs).unwrap();

    // 2 rule wikilinks in ATOM_A — "rules/RULE 0.12" → "0.12" and
    // "rules/memory-protocol" → "memory-protocol". Both edges persisted
    // regardless of whether the rule unit exists (edges are path-keyed).
    assert_eq!(edges_written, 2);

    let outgoing = list_outgoing(&store, "kei-task::create").unwrap();
    let rule_edges: Vec<&str> = outgoing
        .iter()
        .filter(|e| e.edge_type == "rule_ref")
        .map(|e| e.dst_path.as_str())
        .collect();
    assert!(rule_edges.contains(&"rule:0.12"));
    assert!(rule_edges.contains(&"rule:memory-protocol"));
}

#[test]
fn discover_rules_empty_dir_returns_empty() {
    let tmp = tempdir().unwrap();
    let recs = discover_rules(tmp.path()).unwrap();
    assert!(recs.is_empty());
}

#[test]
fn discover_rules_without_heading_falls_back_to_slug() {
    let tmp = tempdir().unwrap();
    fs::write(tmp.path().join("plain.md"), "no heading in this file\n").unwrap();

    let recs = discover_rules(tmp.path()).unwrap();
    assert_eq!(recs.len(), 1);
    assert_eq!(recs[0].slug, "plain");
    assert_eq!(recs[0].name, "plain");
}
