//! Integration test — walk_atoms returns 2 well-formed records from temp root.

use kei_runtime::discover::{walk_atoms, AtomKind};
use std::fs;
use std::path::Path;

fn write_atom(root: &Path, crate_name: &str, verb: &str) {
    let atoms = root.join(crate_name).join("atoms");
    let schemas = atoms.join("schemas");
    fs::create_dir_all(&schemas).unwrap();
    let input = format!("{verb}-input.json");
    let output = format!("{verb}-output.json");
    fs::write(schemas.join(&input), "{}").unwrap();
    fs::write(schemas.join(&output), "{}").unwrap();
    let md = format!(
        r#"---
atom: {crate_name}::{verb}
kind: query
version: "0.1.0"
input:
  schema: schemas/{input}
output:
  schema: schemas/{output}
side_effects: []
idempotent: true
stability: stable
---

# {crate_name}::{verb}
"#,
    );
    fs::write(atoms.join(format!("{verb}.md")), md).unwrap();
}

#[test]
fn walk_atoms_finds_two_records() {
    let tmp = tempfile::tempdir().unwrap();
    write_atom(tmp.path(), "kei-alpha", "search");
    write_atom(tmp.path(), "kei-beta", "fetch");
    let mut atoms = walk_atoms(tmp.path());
    atoms.sort_by(|a, b| a.full_id.cmp(&b.full_id));
    assert_eq!(atoms.len(), 2);
    assert_eq!(atoms[0].full_id, "kei-alpha::search");
    assert_eq!(atoms[0].crate_name, "kei-alpha");
    assert_eq!(atoms[0].verb, "search");
    assert_eq!(atoms[0].kind, AtomKind::Query);
    assert_eq!(atoms[1].full_id, "kei-beta::fetch");
    assert!(atoms[1]
        .input_schema
        .as_ref()
        .unwrap()
        .ends_with("schemas/fetch-input.json"));
    assert!(atoms[1]
        .output_schema
        .as_ref()
        .unwrap()
        .ends_with("schemas/fetch-output.json"));
}
