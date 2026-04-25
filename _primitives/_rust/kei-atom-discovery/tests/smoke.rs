//! Smoke tests covering the 4 critical fixes consolidated in this crate.

use kei_atom_discovery::{
    classify_wikilink, discover_atoms, parse_frontmatter, parse_wikilink, safe_join, AtomKind,
    Error, WikilinkTarget, MAX_FRONTMATTER_BYTES,
};
use std::fs;
use std::path::Path;
use tempfile::tempdir;

const ATOM_OK: &str = r#"---
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
keywords: [task, todo]
related:
  - "[[kei-task::add-dependency]]"
  - "[[rules/RULE 0.12]]"
---
# kei-task::create
Body text.
"#;

fn write_atom(root: &Path, crate_name: &str, verb: &str, body: &str) {
    let atoms_dir = root.join(crate_name).join("atoms");
    fs::create_dir_all(atoms_dir.join("schemas")).unwrap();
    fs::write(atoms_dir.join(format!("{verb}.md")), body).unwrap();
    fs::write(atoms_dir.join("schemas").join("create-input.json"), "{}").unwrap();
    fs::write(atoms_dir.join("schemas").join("create-output.json"), "{}").unwrap();
}

// FIX 2 happy path — shared Frontmatter correctly parses and exposes typed kind
#[test]
fn discovery_returns_well_formed_atom_meta() {
    let tmp = tempdir().unwrap();
    write_atom(tmp.path(), "kei-task", "create", ATOM_OK);
    let atoms = discover_atoms(tmp.path());
    assert_eq!(atoms.len(), 1);
    let a = &atoms[0];
    assert_eq!(a.full_id, "kei-task::create");
    assert_eq!(a.kind, AtomKind::Command);
    assert_eq!(a.crate_name, "kei-task");
    assert_eq!(a.verb, "create");
    assert!(a.input_schema.is_some());
    assert!(a.output_schema.is_some());
    assert_eq!(a.side_effects.len(), 1);
    assert_eq!(a.side_effects[0].op, "write");
    assert_eq!(a.side_effects[0].domain, "kei-task-db");
    assert!(a.body.contains("Body text"));
}

// FIX 1 — path traversal rejection via safe_join
#[test]
fn safe_join_rejects_parent_component() {
    let tmp = tempdir().unwrap();
    let err = safe_join(tmp.path(), "../etc/shadow").unwrap_err();
    assert!(matches!(err, Error::PathParent(_)));
}

#[test]
fn safe_join_rejects_absolute_path() {
    let tmp = tempdir().unwrap();
    let err = safe_join(tmp.path(), "/etc/shadow").unwrap_err();
    assert!(matches!(err, Error::PathAbsolute(_)));
}

#[test]
fn safe_join_accepts_plain_relative() {
    let tmp = tempdir().unwrap();
    let target = tmp.path().join("schemas");
    fs::create_dir_all(&target).unwrap();
    let joined = safe_join(tmp.path(), "schemas").unwrap();
    assert!(joined.ends_with("schemas"));
}

// FIX 3 — YAML size cap enforced pre-parse
#[test]
fn frontmatter_size_cap_enforced() {
    let huge = "x".repeat(MAX_FRONTMATTER_BYTES + 100);
    let md = format!("---\n{huge}\n---\nbody\n");
    let err = parse_frontmatter(&md).unwrap_err();
    assert!(matches!(err, Error::FrontmatterTooLarge { .. }));
}

#[test]
fn frontmatter_missing_start_rejected() {
    let err = parse_frontmatter("no fence\nbody\n").unwrap_err();
    assert!(matches!(err, Error::FrontmatterMissingStart));
}

#[test]
fn frontmatter_missing_end_rejected() {
    let err = parse_frontmatter("---\nkey: val\nno-end\n").unwrap_err();
    assert!(matches!(err, Error::FrontmatterMissingEnd));
}

// FIX — symlink not followed (walkdir follow_links=false)
#[test]
fn discover_does_not_follow_symlinks() {
    let tmp = tempdir().unwrap();
    write_atom(tmp.path(), "kei-real", "create", ATOM_OK);
    // Create a symlink named `kei-link` pointing at `kei-real`.
    #[cfg(unix)]
    {
        let target = tmp.path().join("kei-real");
        let link = tmp.path().join("kei-link");
        std::os::unix::fs::symlink(&target, &link).unwrap();
    }
    let atoms = discover_atoms(tmp.path());
    // Only 1 atom — symlinked tree is NOT walked.
    assert_eq!(atoms.len(), 1, "symlink was traversed — follow_links must be false");
}

// Wikilink strictness
#[test]
fn wikilink_malformed_returns_none() {
    assert_eq!(parse_wikilink("[[[foo]]"), None); // triple-bracket open
    assert_eq!(parse_wikilink("foo"), None);
    assert_eq!(parse_wikilink("[[ ]]"), None);
    assert_eq!(
        parse_wikilink("[[kei-task::create]]"),
        Some("kei-task::create".to_string())
    );
}

// classify_wikilink — 3 variants (Atom / Rule / Other)
#[test]
fn classify_atom_target() {
    assert_eq!(
        classify_wikilink("kei-task::create"),
        WikilinkTarget::Atom("kei-task::create".into())
    );
}

#[test]
fn classify_rule_targets() {
    assert_eq!(
        classify_wikilink("rules/RULE 0.12"),
        WikilinkTarget::Rule("0.12".into())
    );
    assert_eq!(
        classify_wikilink("rules/memory-protocol"),
        WikilinkTarget::Rule("memory-protocol".into())
    );
    assert_eq!(
        classify_wikilink("rule 0.12"),
        WikilinkTarget::Rule("0.12".into())
    );
}

#[test]
fn classify_other_target() {
    assert_eq!(
        classify_wikilink("random-note"),
        WikilinkTarget::Other("random-note".into())
    );
}
