//! Integration smoke test for atom discovery + wikilink resolution.
//!
//! Creates a temp root with 2 fake crates, each with `atoms/<verb>.md`,
//! asserts `discover_atoms` returns 2 records and frontmatter is parsed.

use kei_sage::atom_index::index_atoms;
use kei_sage::atoms::{discover_atoms, resolve_wikilinks, AtomKind};
use kei_sage::Store;
use std::fs;
use tempfile::tempdir;

const ATOM_A: &str = r#"---
atom: kei-task::create
kind: command
version: "0.1.0"
input:
  schema: schemas/create-input.json
output:
  schema: schemas/create-output.json
stability: stable
keywords: [task, todo]
related:
  - "[[kei-task::add-dependency]]"
  - "[[rules/RULE 0.12]]"
---
# kei-task::create

Creates a task.
"#;

const ATOM_B: &str = r#"---
atom: kei-task::add-dependency
kind: command
version: "0.1.0"
stability: beta
keywords: [task, dag]
related: []
---
# kei-task::add-dependency

Links two tasks.
"#;

const ATOM_BAD: &str = r#"not-yaml-frontmatter

just a plain markdown file
"#;

fn write_atom(root: &std::path::Path, crate_name: &str, verb: &str, body: &str) {
    let atoms_dir = root.join(crate_name).join("atoms");
    fs::create_dir_all(&atoms_dir).unwrap();
    fs::write(atoms_dir.join(format!("{verb}.md")), body).unwrap();
}

#[test]
fn discover_returns_both_records() {
    let tmp = tempdir().unwrap();
    write_atom(tmp.path(), "kei-task", "create", ATOM_A);
    write_atom(tmp.path(), "kei-task", "add-dependency", ATOM_B);

    let recs = discover_atoms(tmp.path()).unwrap();
    assert_eq!(recs.len(), 2, "expected 2 records, got {}", recs.len());

    let ids: Vec<&str> = recs.iter().map(|r| r.full_id.as_str()).collect();
    assert!(ids.contains(&"kei-task::create"));
    assert!(ids.contains(&"kei-task::add-dependency"));
}

#[test]
fn frontmatter_fields_parsed() {
    let tmp = tempdir().unwrap();
    write_atom(tmp.path(), "kei-task", "create", ATOM_A);

    let recs = discover_atoms(tmp.path()).unwrap();
    let rec = recs.iter().find(|r| r.full_id == "kei-task::create").unwrap();

    assert_eq!(rec.kind, AtomKind::Command);
    assert_eq!(rec.crate_name, "kei-task");
    assert_eq!(rec.verb, "create");
    assert_eq!(rec.version, "0.1.0");
    assert_eq!(rec.stability, "stable");
    assert!(rec.keywords.contains(&"task".to_string()));
    assert!(rec.input_schema.is_some());
    assert!(rec.output_schema.is_some());
    assert!(rec.body.contains("Creates a task"));
}

#[test]
fn invalid_frontmatter_is_skipped_not_fatal() {
    let tmp = tempdir().unwrap();
    write_atom(tmp.path(), "kei-task", "create", ATOM_A);
    write_atom(tmp.path(), "kei-task", "broken", ATOM_BAD);

    let recs = discover_atoms(tmp.path()).unwrap();
    assert_eq!(recs.len(), 1);
    assert_eq!(recs[0].full_id, "kei-task::create");
}

#[test]
fn wikilinks_filter_rule_targets() {
    let tmp = tempdir().unwrap();
    write_atom(tmp.path(), "kei-task", "create", ATOM_A);
    write_atom(tmp.path(), "kei-task", "add-dependency", ATOM_B);

    let recs = discover_atoms(tmp.path()).unwrap();
    let edges = resolve_wikilinks(&recs);

    // Only atom-to-atom edges remain; `[[rules/RULE 0.12]]` filtered.
    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].0, "kei-task::create");
    assert_eq!(edges[0].1, "kei-task::add-dependency");
}

#[test]
fn empty_root_returns_empty() {
    let tmp = tempdir().unwrap();
    let recs = discover_atoms(tmp.path()).unwrap();
    assert!(recs.is_empty());
}

#[test]
fn index_atoms_persists_units_and_edges() {
    let tmp = tempdir().unwrap();
    write_atom(tmp.path(), "kei-task", "create", ATOM_A);
    write_atom(tmp.path(), "kei-task", "add-dependency", ATOM_B);

    let recs = discover_atoms(tmp.path()).unwrap();
    let store = Store::open_memory().unwrap();
    let stats = index_atoms(&store, &recs).unwrap();

    assert_eq!(stats.units_indexed, 2);
    assert_eq!(stats.edges_indexed, 1);
    assert_eq!(store.count_units().unwrap(), 2);
}
