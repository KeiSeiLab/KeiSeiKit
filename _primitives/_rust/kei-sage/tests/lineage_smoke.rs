//! Smoke tests for lineage traversal over capability.toml primitives.

use kei_sage::lineage::{discover_lineage, trace_lineage};
use std::fs;
use tempfile::tempdir;

const CAP_ROOT: &str = r#"
[capability]
name = "policy::no-git-ops"

[lineage]
parents = []
created-by = "ag-human"
created-at = "2026-04-23T10:00:00Z"
"#;

const CAP_CHILD: &str = r#"
[capability]
name = "policy::no-git-ops-lax"

[lineage]
parents = ["[[policy::no-git-ops]]"]
fork-from = "policy::no-git-ops"
created-by = "ag-user-xyz"
created-at = "2026-04-23T12:00:00Z"
"#;

fn write_cap(root: &std::path::Path, sub: &str, name: &str, body: &str) {
    let dir = root.join(sub).join(name);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("capability.toml"), body).unwrap();
}

#[test]
fn lineage_no_parents_returns_only_self() {
    let cap = tempdir().unwrap();
    let man = tempdir().unwrap();
    write_cap(cap.path(), "policy", "no-git-ops", CAP_ROOT);

    let nodes = discover_lineage(cap.path(), man.path());
    let trace = trace_lineage(&nodes, "policy::no-git-ops", 3);
    assert!(trace.focus.is_some());
    assert!(trace.ancestors.is_empty(), "no parents expected");
    assert!(trace.descendants.is_empty(), "no descendants expected");
}

#[test]
fn lineage_parent_wikilink_is_traversed_upward() {
    let cap = tempdir().unwrap();
    let man = tempdir().unwrap();
    write_cap(cap.path(), "policy", "no-git-ops", CAP_ROOT);
    write_cap(cap.path(), "policy", "no-git-ops-lax", CAP_CHILD);

    let nodes = discover_lineage(cap.path(), man.path());
    let trace = trace_lineage(&nodes, "policy::no-git-ops-lax", 3);
    assert!(trace.ancestors.contains(&"policy::no-git-ops".to_string()));
}

#[test]
fn lineage_fork_from_yields_descendant() {
    let cap = tempdir().unwrap();
    let man = tempdir().unwrap();
    write_cap(cap.path(), "policy", "no-git-ops", CAP_ROOT);
    write_cap(cap.path(), "policy", "no-git-ops-lax", CAP_CHILD);

    let nodes = discover_lineage(cap.path(), man.path());
    let trace = trace_lineage(&nodes, "policy::no-git-ops", 3);
    assert!(trace.descendants.contains(&"policy::no-git-ops-lax".to_string()));
}
