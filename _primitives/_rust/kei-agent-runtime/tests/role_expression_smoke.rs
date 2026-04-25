//! Layer E — role expression resolver smoke tests.
//!
//! Fixtures built in tempdir; each test writes the role files it needs,
//! runs `resolve_role`, asserts the flattened required list.

use kei_agent_runtime::role::{resolve_role, RoleError, MAX_DEPTH};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn write_role(root: &Path, name: &str, body: &str) {
    let dir = root.join("_roles");
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join(format!("{name}.toml")), body).unwrap();
}

#[test]
fn extends_chain_merges_parent_plus_local() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    write_role(
        root,
        "base",
        r#"
[role]
name = "base"

[capabilities]
required = ["tools::deny-tools", "output::report-format"]
"#,
    );
    write_role(
        root,
        "child",
        r#"
[role]
name = "child"

[capabilities]
extends = "base"
required = ["tools::bash-allowlist"]
"#,
    );

    let r = resolve_role(root, "child").unwrap();
    assert_eq!(
        r.required,
        vec![
            "tools::deny-tools".to_string(),
            "output::report-format".to_string(),
            "tools::bash-allowlist".to_string(),
        ],
        "child should inherit parent ordering then append local"
    );
}

#[test]
fn cycle_detection_errors_with_path() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    write_role(
        root,
        "a",
        r#"
[role]
name = "a"

[capabilities]
extends = "b"
"#,
    );
    write_role(
        root,
        "b",
        r#"
[role]
name = "b"

[capabilities]
extends = "a"
"#,
    );

    let err = resolve_role(root, "a").unwrap_err();
    let msg = format!("{err:#}");
    assert!(
        msg.contains("cycle"),
        "error should mention cycle: got {msg}"
    );
}

#[test]
fn relaxes_drops_inherited_capability() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    write_role(
        root,
        "parent",
        r#"
[role]
name = "parent"

[capabilities]
required = ["scope::files-whitelist", "quality::cargo-check-green", "output::report-format"]
"#,
    );
    write_role(
        root,
        "relaxed",
        r#"
[role]
name = "relaxed"

[capabilities]
extends = "parent"
relaxes = ["scope::files-whitelist"]
"#,
    );

    let r = resolve_role(root, "relaxed").unwrap();
    assert!(
        !r.required.iter().any(|c| c == "scope::files-whitelist"),
        "relaxed cap must be removed from the inherited list"
    );
    assert!(r.required.iter().any(|c| c == "quality::cargo-check-green"));
    assert!(r.required.iter().any(|c| c == "output::report-format"));
}

#[test]
fn flat_role_without_extends_still_works() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    write_role(
        root,
        "flat",
        r#"
[role]
name = "flat"

[capabilities]
required = ["policy::no-git-ops", "output::report-format"]
"#,
    );

    let r = resolve_role(root, "flat").unwrap();
    assert_eq!(r.required.len(), 2);
    assert_eq!(r.required[0], "policy::no-git-ops");
    assert_eq!(r.required[1], "output::report-format");
}

#[test]
fn extends_chain_deeper_than_max_depth_errors() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    // Build a linear chain r0 -> r1 -> ... -> r20, which exceeds
    // MAX_DEPTH = 16 and must refuse rather than recurse.
    let total = MAX_DEPTH + 5;
    for i in 0..total {
        let body = if i + 1 < total {
            format!(
                "[role]\nname = \"r{i}\"\n\n[capabilities]\nextends = \"r{next}\"\n",
                i = i,
                next = i + 1
            )
        } else {
            format!("[role]\nname = \"r{i}\"\n\n[capabilities]\nrequired = []\n", i = i)
        };
        write_role(root, &format!("r{i}"), &body);
    }
    let err = resolve_role(root, "r0").unwrap_err();
    let role_err = err
        .downcast_ref::<RoleError>()
        .expect("expected typed RoleError");
    match role_err {
        RoleError::MaxDepthExceeded { depth, .. } => {
            assert_eq!(*depth, MAX_DEPTH, "error must report configured cap");
        }
        other => panic!("expected MaxDepthExceeded, got {other:?}"),
    }
}

#[test]
fn role_name_with_path_traversal_is_refused_before_fs_join() {
    // No file created — attacker-controlled name must fail at validation,
    // not at read-time with an ambiguous NotFound.
    let tmp = TempDir::new().unwrap();
    let err = resolve_role(tmp.path(), "../../etc/passwd").unwrap_err();
    let role_err = err
        .downcast_ref::<RoleError>()
        .expect("expected typed RoleError");
    match role_err {
        RoleError::InvalidName { kind, value } => {
            assert_eq!(*kind, "role");
            assert_eq!(value, "../../etc/passwd");
        }
        other => panic!("expected InvalidName, got {other:?}"),
    }
}

#[test]
fn capability_name_with_path_traversal_is_refused_in_compose() {
    use kei_agent_runtime::capability::TaskSpec;
    use kei_agent_runtime::compose::compose_prompt;

    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    write_role(
        root,
        "traversal",
        r#"
[role]
name = "traversal"

[capabilities]
required = ["../../etc::passwd"]
"#,
    );
    let mut task = TaskSpec::default();
    task.task.role = "traversal".into();
    task.task.agent_id = "traversal-attempt".into();
    task.body.text = "whatever".into();
    let err = compose_prompt(&task, root).unwrap_err();
    let role_err = err
        .chain()
        .find_map(|e| e.downcast_ref::<RoleError>())
        .expect("expected typed RoleError in error chain");
    match role_err {
        RoleError::InvalidName { kind, .. } => {
            assert!(
                kind.starts_with("capability-"),
                "kind should be capability-category or capability-slug, got {kind}"
            );
        }
        other => panic!("expected InvalidName, got {other:?}"),
    }
}

#[test]
fn relaxes_missing_cap_collects_warning_without_failing() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    write_role(
        root,
        "base-warn",
        r#"
[role]
name = "base-warn"

[capabilities]
required = ["policy::no-git-ops"]
"#,
    );
    write_role(
        root,
        "child-warn",
        r#"
[role]
name = "child-warn"

[capabilities]
extends = "base-warn"
relaxes = ["scope::files-whitelist"]
"#,
    );

    let r = resolve_role(root, "child-warn").expect("must not fail");
    assert_eq!(r.required, vec!["policy::no-git-ops".to_string()]);
    assert_eq!(r.warnings.len(), 1, "expected exactly one warning");
    let w = &r.warnings[0];
    assert!(
        w.contains("scope::files-whitelist") && w.contains("no-op"),
        "warning should name dropped cap + no-op, got: {w}"
    );
}
