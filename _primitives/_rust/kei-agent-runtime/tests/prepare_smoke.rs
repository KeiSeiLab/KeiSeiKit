//! Prepare smoke — validates orchestrator-facing wrapper.
//!
//! Three fixtures per task spec:
//!   1. Happy path — valid task.toml → AgentInvocation fully populated
//!   2. Unknown role → clear error (role lookup fails)
//!   3. Non-spawnable role (git-ops) → explicit refusal + RULE 0.13 pointer

use kei_agent_runtime::capability::TaskSpec;
use kei_agent_runtime::prepare::{prepare, render_human};
use tempfile::TempDir;

fn write_capability(root: &std::path::Path, cat: &str, slug: &str, body: &str) {
    let dir = root.join("_capabilities").join(cat).join(slug);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("text.md"), body).unwrap();
}

fn write_role(root: &std::path::Path, name: &str, toml: &str) {
    std::fs::create_dir_all(root.join("_roles")).unwrap();
    std::fs::write(root.join("_roles").join(format!("{name}.toml")), toml).unwrap();
}

#[test]
fn happy_path_yields_full_invocation() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();

    write_capability(root, "policy", "no-git-ops", "## Never git.\n");
    write_capability(root, "output", "report-format", "## Report fields.\n");
    write_role(
        root,
        "edit-local",
        r#"
[role]
name = "edit-local"
spawnable = true
claude-subagent-type = "code-implementer"

[capabilities]
required = ["policy::no-git-ops", "output::report-format"]
"#,
    );

    let mut task = TaskSpec::default();
    task.task.role = "edit-local".into();
    task.task.agent_id = "edit-local-forge-abc123".into();
    task.body.text = "Port kei-forge templating to pure-Rust.".into();

    let inv = prepare(&task, root).expect("prepare should succeed");
    assert_eq!(inv.agent_id, "edit-local-forge-abc123");
    assert_eq!(inv.role, "edit-local");
    assert_eq!(inv.subagent_type, "code-implementer");
    assert_eq!(inv.isolation.as_deref(), Some("worktree"));
    assert!(inv.prompt.contains("Never git"));
    assert!(inv.prompt.contains("Report fields"));
    assert!(inv.prompt.contains("Port kei-forge templating"));
    assert!(inv.verify_command.contains("kei-agent-runtime verify"));
    assert!(inv.verify_command.contains("edit-local-forge-abc123"));
    assert!(inv.ledger_row.contains("running"));
    assert!(inv.ledger_row.contains("edit-local"));
    assert!(inv.ledger_row.contains("parent=none"));

    let human = render_human(&inv);
    assert!(human.contains("=== AGENT SUBSTRATE v1"));
    assert!(human.contains("--- PROMPT"));
    assert!(human.contains("--- END PROMPT"));
    assert!(human.contains("subagent_type: code-implementer"));
}

#[test]
fn unknown_role_errors_clearly() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();

    let mut task = TaskSpec::default();
    task.task.role = "does-not-exist".into();
    task.task.agent_id = "x-1".into();

    let err = prepare(&task, root).expect_err("unknown role must fail");
    let msg = format!("{err:#}");
    assert!(
        msg.contains("does-not-exist") || msg.contains("role"),
        "error should mention the role or the word 'role': got {msg}"
    );
}

#[test]
fn non_spawnable_role_refused_with_rule_013_pointer() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();

    write_role(
        root,
        "git-ops",
        r#"
[role]
name = "git-ops"
spawnable = false
claude-subagent-type = "NOT-SPAWNABLE"

[capabilities]
required = []
"#,
    );

    let mut task = TaskSpec::default();
    task.task.role = "git-ops".into();
    task.task.agent_id = "orchestrator-only-1".into();

    let err = prepare(&task, root).expect_err("git-ops must be refused");
    let msg = format!("{err:#}");
    assert!(
        msg.contains("RULE 0.13"),
        "refusal must cite RULE 0.13: got {msg}"
    );
    assert!(
        msg.contains("spawnable") || msg.contains("orchestrator"),
        "refusal message should mention spawnable/orchestrator: got {msg}"
    );
}
