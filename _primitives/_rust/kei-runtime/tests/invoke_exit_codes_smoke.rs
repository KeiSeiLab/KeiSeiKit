//! Integration test — `kei-runtime invoke` exit codes per §Runtime contract.
//!
//! - Unknown atom id → exit 2 (atom rejected)
//! - Known atom whose crate binary is not on PATH → exit 127 (BinaryNotFound)
//!
//! Real-atom execution (happy path) lives in `invoke_real_atom.rs`, which
//! points `KEI_RUNTIME_BIN_DIR` at the workspace `target/` to pick up
//! `kei-task` without polluting the user's PATH.

use std::fs;
use std::path::Path;
use std::process::Command;

const BIN: &str = env!("CARGO_BIN_EXE_kei-runtime");

fn write_atom(root: &Path, crate_name: &str, verb: &str) {
    let atoms = root.join(crate_name).join("atoms");
    let schemas = atoms.join("schemas");
    fs::create_dir_all(&schemas).unwrap();
    let input = format!("{verb}-input.json");
    let output = format!("{verb}-output.json");
    let input_schema = r#"{
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": { "title": { "type": "string" } }
    }"#;
    let output_schema = r#"{
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object"
    }"#;
    fs::write(schemas.join(&input), input_schema).unwrap();
    fs::write(schemas.join(&output), output_schema).unwrap();
    let md = format!(
        r#"---
atom: {crate_name}::{verb}
kind: command
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
fn invoke_atom_not_found_exits_2() {
    let tmp = tempfile::tempdir().unwrap();
    write_atom(tmp.path(), "kei-demo", "create");
    let out = Command::new(BIN)
        .arg("invoke")
        .arg("kei-demo::ghost")
        .arg("--input")
        .arg("{}")
        .arg("--root")
        .arg(tmp.path())
        .output()
        .expect("spawn kei-runtime");
    assert_eq!(out.status.code(), Some(2),
        "expected exit 2 on unknown atom; stderr: {}",
        String::from_utf8_lossy(&out.stderr));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("no atom matching"),
        "expected 'no atom matching' in stderr: {stderr}");
}

#[test]
fn invoke_binary_not_found_exits_127() {
    let tmp = tempfile::tempdir().unwrap();
    write_atom(tmp.path(), "kei-demo-absent", "create");
    // Use an empty bin dir so the `kei-demo-absent` binary cannot be found.
    let empty_bin = tmp.path().join("empty-bin-dir");
    std::fs::create_dir_all(&empty_bin).unwrap();
    let out = Command::new(BIN)
        .env("KEI_RUNTIME_BIN_DIR", &empty_bin)
        .env("PATH", &empty_bin)
        .arg("invoke")
        .arg("kei-demo-absent::create")
        .arg("--input")
        .arg(r#"{"title":"hello"}"#)
        .arg("--root")
        .arg(tmp.path())
        .output()
        .expect("spawn kei-runtime");
    assert_eq!(out.status.code(), Some(127),
        "expected exit 127 on BinaryNotFound; stderr: {}",
        String::from_utf8_lossy(&out.stderr));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("not found"),
        "expected 'not found' in stderr: {stderr}");
}

/// An atom whose `crate_name` is not in the `kei-*` allowlist should exit 2
/// (InvalidAtom is mapped to the same "atom rejected" exit code).
#[test]
fn invoke_unsafe_crate_name_exits_2() {
    let tmp = tempfile::tempdir().unwrap();
    // Write a well-structured atom dir but with a crate_name that would be
    // dangerous (e.g. "rm") — this must be rejected before any binary lookup.
    let crate_name = "rm";
    let verb = "all";
    let atoms = tmp.path().join(crate_name).join("atoms");
    let schemas = atoms.join("schemas");
    std::fs::create_dir_all(&schemas).unwrap();
    let input_schema = r#"{"$schema":"http://json-schema.org/draft-07/schema#","type":"object"}"#;
    let output_schema = r#"{"$schema":"http://json-schema.org/draft-07/schema#","type":"object"}"#;
    std::fs::write(schemas.join("all-input.json"), input_schema).unwrap();
    std::fs::write(schemas.join("all-output.json"), output_schema).unwrap();
    let md = format!(
        "---\natom: {crate_name}::{verb}\nkind: command\nversion: \"0.1.0\"\n\
         input:\n  schema: schemas/all-input.json\n\
         output:\n  schema: schemas/all-output.json\n\
         side_effects: []\nidempotent: true\nstability: stable\n---\n"
    );
    std::fs::write(atoms.join(format!("{verb}.md")), md).unwrap();
    let out = std::process::Command::new(BIN)
        .arg("invoke")
        .arg(format!("{crate_name}::{verb}"))
        .arg("--input").arg("{}")
        .arg("--root").arg(tmp.path())
        .output()
        .expect("spawn kei-runtime");
    assert_eq!(out.status.code(), Some(2),
        "expected exit 2 for unsafe crate_name; stderr: {}",
        String::from_utf8_lossy(&out.stderr));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("allowlist") || stderr.contains("invalid"),
        "expected allowlist error in stderr: {stderr}");
}
