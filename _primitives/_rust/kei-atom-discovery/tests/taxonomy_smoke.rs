//! Taxonomy + Lineage facet parsing smoke tests.
//!
//! Covers (a) full 7-facet taxonomy + lineage with multiple parents,
//! (b) partial taxonomy (only kingdom + mechanism) — remaining fields None,
//! (c) backward-compat: atom without any [taxonomy]/[lineage] still parses,
//! (d) lineage.parents array parses correctly (multi-parent diamond lineage).

use kei_atom_discovery::{discover_atoms, Lineage, TaxonomyFacets};
use std::fs;
use std::path::Path;
use tempfile::tempdir;

const ATOM_FULL: &str = r#"---
atom: kei-task::create
kind: command
version: "0.1.0"
input:
  schema: schemas/create-input.json
output:
  schema: schemas/create-output.json
side_effects:
  - { op: write, domain: kei-task-db }
idempotent: false
stability: stable
keywords: [task]
related: []
taxonomy:
  kingdom: atom
  mechanism: transform
  domain: task
  layer: atom-substrate
  stage: runtime
  stability: stable
  language: rust
lineage:
  parents:
    - "[[kei-task::add-dependency]]"
    - "[[rules/RULE 0.12]]"
  creator: ag-orchestrator-human
  created: "2026-04-23"
  fork_from: dna-abc123
---
# body
"#;

const ATOM_PARTIAL: &str = r#"---
atom: kei-task::update
kind: command
version: "0.1.0"
input:
  schema: schemas/create-input.json
output:
  schema: schemas/create-output.json
side_effects: []
taxonomy:
  kingdom: atom
  mechanism: transform
lineage:
  parents: []
---
# body
"#;

const ATOM_NO_FACETS: &str = r#"---
atom: kei-task::delete
kind: command
version: "0.1.0"
input:
  schema: schemas/create-input.json
output:
  schema: schemas/create-output.json
side_effects: []
---
# body
"#;

fn write_atom(root: &Path, crate_name: &str, verb: &str, body: &str) {
    let atoms_dir = root.join(crate_name).join("atoms");
    fs::create_dir_all(atoms_dir.join("schemas")).unwrap();
    fs::write(atoms_dir.join(format!("{verb}.md")), body).unwrap();
    fs::write(atoms_dir.join("schemas").join("create-input.json"), "{}").unwrap();
    fs::write(atoms_dir.join("schemas").join("create-output.json"), "{}").unwrap();
}

fn find<'a>(
    atoms: &'a [kei_atom_discovery::AtomMeta],
    full_id: &str,
) -> &'a kei_atom_discovery::AtomMeta {
    atoms
        .iter()
        .find(|a| a.full_id == full_id)
        .expect("atom present")
}

#[test]
fn full_taxonomy_and_lineage_parse() {
    let tmp = tempdir().unwrap();
    write_atom(tmp.path(), "kei-task", "create", ATOM_FULL);
    let atoms = discover_atoms(tmp.path());
    let a = find(&atoms, "kei-task::create");
    let tax: &TaxonomyFacets = a.taxonomy.as_ref().expect("taxonomy present");
    assert_eq!(tax.kingdom.as_deref(), Some("atom"));
    assert_eq!(tax.mechanism.as_deref(), Some("transform"));
    assert_eq!(tax.domain.as_deref(), Some("task"));
    assert_eq!(tax.layer.as_deref(), Some("atom-substrate"));
    assert_eq!(tax.stage.as_deref(), Some("runtime"));
    assert_eq!(tax.stability.as_deref(), Some("stable"));
    assert_eq!(tax.language.as_deref(), Some("rust"));
    let lin: &Lineage = a.lineage.as_ref().expect("lineage present");
    assert_eq!(lin.creator.as_deref(), Some("ag-orchestrator-human"));
    assert_eq!(lin.created.as_deref(), Some("2026-04-23"));
    assert_eq!(lin.fork_from.as_deref(), Some("dna-abc123"));
}

#[test]
fn partial_taxonomy_leaves_rest_none() {
    let tmp = tempdir().unwrap();
    write_atom(tmp.path(), "kei-task", "update", ATOM_PARTIAL);
    let atoms = discover_atoms(tmp.path());
    let a = find(&atoms, "kei-task::update");
    let tax = a.taxonomy.as_ref().expect("taxonomy present");
    assert_eq!(tax.kingdom.as_deref(), Some("atom"));
    assert_eq!(tax.mechanism.as_deref(), Some("transform"));
    assert!(tax.domain.is_none());
    assert!(tax.layer.is_none());
    assert!(tax.stage.is_none());
    assert!(tax.stability.is_none());
    assert!(tax.language.is_none());
    let lin = a.lineage.as_ref().expect("lineage present");
    assert!(lin.parents.is_empty());
    assert!(lin.creator.is_none());
}

#[test]
fn no_facets_section_still_parses_backward_compat() {
    let tmp = tempdir().unwrap();
    write_atom(tmp.path(), "kei-task", "delete", ATOM_NO_FACETS);
    let atoms = discover_atoms(tmp.path());
    let a = find(&atoms, "kei-task::delete");
    assert!(a.taxonomy.is_none(), "no [taxonomy] → None");
    assert!(a.lineage.is_none(), "no [lineage] → None");
}

#[test]
fn lineage_parents_array_preserved() {
    let tmp = tempdir().unwrap();
    write_atom(tmp.path(), "kei-task", "create", ATOM_FULL);
    let atoms = discover_atoms(tmp.path());
    let a = find(&atoms, "kei-task::create");
    let lin = a.lineage.as_ref().expect("lineage present");
    assert_eq!(lin.parents.len(), 2);
    assert_eq!(lin.parents[0], "[[kei-task::add-dependency]]");
    assert_eq!(lin.parents[1], "[[rules/RULE 0.12]]");
}
