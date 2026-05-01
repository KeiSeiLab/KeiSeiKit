//! Compose smoke test — load fake role + 2 capabilities from a tempdir
//! fixture, assert composed prompt contains both text fragments and the
//! task body.

use kei_agent_runtime::capability::TaskSpec;
use kei_agent_runtime::compose::compose_prompt;
use tempfile::TempDir;

#[test]
fn compose_concatenates_fragments_and_body() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();

    std::fs::create_dir_all(root.join("_capabilities/policy/no-git-ops")).unwrap();
    std::fs::write(
        root.join("_capabilities/policy/no-git-ops/text.md"),
        "## No git\n\nYou must not git.\n",
    )
    .unwrap();
    std::fs::create_dir_all(root.join("_capabilities/output/report-format")).unwrap();
    std::fs::write(
        root.join("_capabilities/output/report-format/text.md"),
        "## Report\n\nEmit a report.\n",
    )
    .unwrap();
    std::fs::create_dir_all(root.join("_roles")).unwrap();
    std::fs::write(
        root.join("_roles/fake.toml"),
        r#"
[role]
name = "fake"

[capabilities]
required = ["policy::no-git-ops", "output::report-format"]
"#,
    )
    .unwrap();

    let mut task = TaskSpec::default();
    task.task.role = "fake".into();
    task.task.agent_id = "abc123".into();
    task.body.text = "Do the thing.".into();

    let prompt = compose_prompt(&task, root).expect("compose");
    assert!(prompt.contains("You must not git"));
    assert!(prompt.contains("Emit a report"));
    assert!(prompt.contains("Do the thing."));
    assert!(prompt.contains("---")); // separator
}

#[test]
fn compose_missing_role_errors() {
    let tmp = TempDir::new().unwrap();
    let mut task = TaskSpec::default();
    task.task.role = "nonexistent".into();
    task.task.agent_id = "x".into();
    let err = compose_prompt(&task, tmp.path()).unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("role") || msg.contains("nonexistent"));
}

#[test]
fn compose_empty_role_errors() {
    let tmp = TempDir::new().unwrap();
    let task = TaskSpec::default();
    let err = compose_prompt(&task, tmp.path()).unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("role"));
}
