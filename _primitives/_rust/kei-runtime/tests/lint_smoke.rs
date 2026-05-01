//! Integration test — schema_lint over a temp root with 1 valid + 1 broken atom.

use kei_runtime::lint::schema_lint;
use std::fs;
use std::path::Path;

fn write_valid_atom(root: &Path) {
    let crate_dir = root.join("kei-demo");
    let atoms = crate_dir.join("atoms");
    let schemas = atoms.join("schemas");
    fs::create_dir_all(&schemas).unwrap();
    let input_schema = r#"{
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "required": ["title"],
        "properties": { "title": { "type": "string" } },
        "additionalProperties": false
    }"#;
    let output_schema = r#"{
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": { "id": { "type": "integer" } }
    }"#;
    fs::write(schemas.join("create-input.json"), input_schema).unwrap();
    fs::write(schemas.join("create-output.json"), output_schema).unwrap();
    let md = r#"---
atom: kei-demo::create
kind: command
version: "0.1.0"
input:
  schema: schemas/create-input.json
output:
  schema: schemas/create-output.json
side_effects:
  - { op: write, domain: kei-demo-db }
idempotent: false
stability: stable
---

# kei-demo::create
"#;
    fs::write(atoms.join("create.md"), md).unwrap();
}

fn write_broken_atom(root: &Path) {
    let crate_dir = root.join("kei-broken");
    let atoms = crate_dir.join("atoms");
    fs::create_dir_all(&atoms).unwrap();
    // Missing `kind` field.
    let md = r#"---
atom: kei-broken::oops
version: "0.1.0"
input:
  schema: schemas/oops-input.json
output:
  schema: schemas/oops-output.json
side_effects: []
idempotent: true
stability: experimental
---

# kei-broken::oops
"#;
    fs::write(atoms.join("oops.md"), md).unwrap();
}

#[test]
fn lint_separates_valid_and_broken() {
    let tmp = tempfile::tempdir().unwrap();
    write_valid_atom(tmp.path());
    write_broken_atom(tmp.path());
    let report = schema_lint(tmp.path());
    assert_eq!(report.passed.len(), 1, "expected 1 passing atom");
    assert_eq!(report.failed.len(), 1, "expected 1 failing atom");
    let (label, errs) = &report.failed[0];
    assert!(label.contains("oops.md"), "failed label mismatch: {label}");
    let joined = errs.join(" ");
    assert!(joined.contains("missing kind"), "expected 'missing kind' in: {joined}");
}
