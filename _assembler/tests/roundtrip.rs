//! Roundtrip / data-preservation tests.
//!
//! The assembler projects the Manifest struct into a Markdown file.
//! We cannot re-parse a Markdown file back into a Manifest (the
//! projection is lossy: comments / blank lines / heading formatting),
//! but we CAN assert that every user-visible string from the manifest
//! appears verbatim in the generated output — i.e. no field is
//! silently dropped by a refactor.

mod common;

use common::{assemble_one, seed_tempdir};
use std::fs;

/// Every `domain_in` bullet, every `forbidden_domain` bullet, every
/// handoff target + trigger, and the agent name must appear in the
/// generated output. Covers the kei-code-implementer manifest which has
/// the richest field population.
#[test]
fn every_manifest_string_appears_in_output() {
    let (_tmp, root) = seed_tempdir();
    let out = assemble_one(&root, "code-implementer");

    // Parse the same manifest independently with toml crate so we
    // can iterate its fields without reaching into the private
    // Manifest struct from main.rs.
    let toml_text =
        fs::read_to_string(root.join("_manifests/code-implementer.toml")).unwrap();
    let parsed: toml::Value = toml::from_str(&toml_text).unwrap();

    let name = parsed["name"].as_str().unwrap();
    assert!(
        out.contains(&format!("name: {name}")),
        "frontmatter missing name"
    );

    let model = parsed["model"].as_str().unwrap();
    assert!(
        out.contains(&format!("model: {model}")),
        "frontmatter missing model"
    );

    // Tools are joined with ", ".
    let tools: Vec<&str> = parsed["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    let tools_line = format!("tools: {}", tools.join(", "));
    assert!(
        out.contains(&tools_line),
        "frontmatter tools line missing or wrong order"
    );

    // domain_in bullets.
    for item in parsed["domain_in"].as_array().unwrap() {
        let s = item.as_str().unwrap();
        assert!(out.contains(s), "domain_in entry missing: {s}");
    }

    // forbidden_domain bullets.
    for item in parsed["forbidden_domain"].as_array().unwrap() {
        let s = item.as_str().unwrap();
        assert!(out.contains(s), "forbidden_domain entry missing: {s}");
    }

    // Handoffs: each target AND each trigger appears.
    for h in parsed["handoff"].as_array().unwrap() {
        let target = h["target"].as_str().unwrap();
        let trigger = h["trigger"].as_str().unwrap();
        assert!(out.contains(target), "handoff target missing: {target}");
        assert!(out.contains(trigger), "handoff trigger missing: {trigger}");
    }
}

/// Double-assembly determinism at the text level: parse + assemble
/// twice from the very same tempdir (not two separate tempdirs) —
/// catches any caching or mutable-global drift inside the binary.
#[test]
fn double_assembly_same_tempdir_identical() {
    let (_tmp, root) = seed_tempdir();
    let first = assemble_one(&root, "cost-guardian");
    let second = assemble_one(&root, "cost-guardian");
    assert_eq!(
        first.as_bytes(),
        second.as_bytes(),
        "consecutive runs in same tempdir diverged"
    );
}
