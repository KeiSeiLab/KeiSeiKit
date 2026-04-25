//! Smoke tests for facet-query over capability.toml primitives.

use kei_sage::facet_query::{
    discover_primitives, discover_primitives_with_roles, matches_all, parse_filters,
};
use std::fs;
use tempfile::tempdir;

const CAP_GATE: &str = r#"
[capability]
name = "policy::no-git-ops"

[taxonomy]
kingdom = "capability"
mechanism = "gate"
"#;

const CAP_SCOPE: &str = r#"
[capability]
name = "scope::files-whitelist"

[taxonomy]
kingdom = "capability"
mechanism = "gate"
severity = "warn"
"#;

const CAP_PLAIN: &str = r#"
[capability]
name = "tools::read-only"
"#;

fn write_cap(root: &std::path::Path, sub: &str, name: &str, body: &str) {
    let dir = root.join(sub).join(name);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("capability.toml"), body).unwrap();
}

#[test]
fn facet_and_filter_matches_two_primitives() {
    let cap = tempdir().unwrap();
    let man = tempdir().unwrap();
    write_cap(cap.path(), "policy", "no-git-ops", CAP_GATE);
    write_cap(cap.path(), "scope", "files-whitelist", CAP_SCOPE);
    write_cap(cap.path(), "tools", "read-only", CAP_PLAIN);

    let all = discover_primitives(cap.path(), man.path());
    assert_eq!(all.len(), 3);

    let filters = parse_filters(&["kingdom=capability".into(), "mechanism=gate".into()]);
    let hits: Vec<_> = all.iter().filter(|p| matches_all(p, &filters)).collect();
    assert_eq!(hits.len(), 2);
    let ids: Vec<&str> = hits.iter().map(|p| p.full_id.as_str()).collect();
    assert!(ids.contains(&"policy::no-git-ops"));
    assert!(ids.contains(&"scope::files-whitelist"));
}

#[test]
fn missing_facet_excluded_from_match() {
    let cap = tempdir().unwrap();
    let man = tempdir().unwrap();
    write_cap(cap.path(), "tools", "read-only", CAP_PLAIN);

    let all = discover_primitives(cap.path(), man.path());
    let filters = parse_filters(&["kingdom=capability".into()]);
    let hits: Vec<_> = all.iter().filter(|p| matches_all(p, &filters)).collect();
    assert_eq!(hits.len(), 0, "primitive without [taxonomy] must not match");
}

#[test]
fn single_filter_matches_subset() {
    let cap = tempdir().unwrap();
    let man = tempdir().unwrap();
    write_cap(cap.path(), "policy", "no-git-ops", CAP_GATE);
    write_cap(cap.path(), "scope", "files-whitelist", CAP_SCOPE);

    let all = discover_primitives(cap.path(), man.path());
    let filters = parse_filters(&["severity=warn".into()]);
    let hits: Vec<_> = all.iter().filter(|p| matches_all(p, &filters)).collect();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].full_id, "scope::files-whitelist");
}

const ROLE_READ_ONLY: &str = r#"
[role]
name = "read-only"

[taxonomy]
kingdom = "role"
mechanism = "compose"
domain = "agent"
"#;

const ROLE_GIT_OPS: &str = r#"
[role]
name = "git-ops"

[taxonomy]
kingdom = "role"
mechanism = "compose"
domain = "agent"
"#;

fn write_role(root: &std::path::Path, name: &str, body: &str) {
    fs::create_dir_all(root).unwrap();
    fs::write(root.join(format!("{name}.toml")), body).unwrap();
}

#[test]
fn role_taxonomy_discovered_via_roles_root() {
    let cap = tempdir().unwrap();
    let man = tempdir().unwrap();
    let roles = tempdir().unwrap();
    write_role(roles.path(), "read-only", ROLE_READ_ONLY);
    write_role(roles.path(), "git-ops", ROLE_GIT_OPS);

    let all = discover_primitives_with_roles(cap.path(), man.path(), Some(roles.path()));
    let filters = parse_filters(&["kingdom=role".into()]);
    let hits: Vec<_> = all.iter().filter(|p| matches_all(p, &filters)).collect();
    assert_eq!(hits.len(), 2);
    let ids: Vec<&str> = hits.iter().map(|p| p.full_id.as_str()).collect();
    assert!(ids.contains(&"role::read-only"));
    assert!(ids.contains(&"role::git-ops"));
}

#[test]
fn backward_compat_capability_still_matches_without_roles() {
    let cap = tempdir().unwrap();
    let man = tempdir().unwrap();
    let roles = tempdir().unwrap();
    write_cap(cap.path(), "policy", "no-git-ops", CAP_GATE);
    write_cap(cap.path(), "scope", "files-whitelist", CAP_SCOPE);
    write_role(roles.path(), "read-only", ROLE_READ_ONLY);

    let all = discover_primitives_with_roles(cap.path(), man.path(), Some(roles.path()));
    let filters = parse_filters(&["kingdom=capability".into()]);
    let hits: Vec<_> = all.iter().filter(|p| matches_all(p, &filters)).collect();
    assert_eq!(hits.len(), 2, "role entry must NOT match kingdom=capability");
    let ids: Vec<&str> = hits.iter().map(|p| p.full_id.as_str()).collect();
    assert!(ids.contains(&"policy::no-git-ops"));
    assert!(ids.contains(&"scope::files-whitelist"));
}
