//! spawn_smoke — integration tests for kei-spawn library API.
//!
//! These tests set `KEI_SPAWN_LEDGER_NOOP=1` so the ledger subprocess is a
//! no-op — we exercise the compose + prepare_agent + output shape path
//! without depending on a real `kei-ledger` binary being on PATH.
//!
//! Fixtures follow the same pattern as kei-agent-runtime's tests: write a
//! minimal `_roles/` + `_capabilities/` tree into a tempdir, a task.toml
//! referencing the role, then call `spawn_from_task` and assert the JSON
//! shape + on-disk artefacts.

use kei_spawn::{spawn_from_task, verify_agent};
use std::path::Path;
use tempfile::TempDir;

fn write_capability(root: &Path, cat: &str, slug: &str, body: &str) {
    let dir = root.join("_capabilities").join(cat).join(slug);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("text.md"), body).unwrap();
}

fn write_role(root: &Path, name: &str, toml: &str) {
    std::fs::create_dir_all(root.join("_roles")).unwrap();
    std::fs::write(root.join("_roles").join(format!("{name}.toml")), toml).unwrap();
}

fn write_task(root: &Path, toml: &str) -> std::path::PathBuf {
    let path = root.join("task.toml");
    std::fs::write(&path, toml).unwrap();
    path
}

fn minimal_kit(root: &Path) {
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
}

fn set_noop() {
    std::env::set_var("KEI_SPAWN_LEDGER_NOOP", "1");
}

#[test]
fn spawn_happy_path_emits_full_output() {
    set_noop();
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    minimal_kit(root);

    let task_path = write_task(
        root,
        r#"
[task]
role = "edit-local"

[body]
text = "Port kei-forge templating to pure Rust."
"#,
    );

    let out = spawn_from_task(&task_path, root).expect("spawn should succeed");
    assert!(out.agent_id.starts_with("ag-edit-local-"), "id: {}", out.agent_id);
    assert_eq!(out.role, "edit-local");
    assert_eq!(out.subagent_type, "code-implementer");
    assert_eq!(out.isolation.as_deref(), Some("worktree"));
    assert!(out.prompt.contains("Port kei-forge"));
    assert!(out.prompt.contains("Never git"));
    assert_eq!(out.spec_sha.len(), 64, "sha256 hex = 64 chars: {}", out.spec_sha);
    assert!(out.branch.starts_with("agent/"));
    assert!(out.branch.contains(&out.agent_id));
    assert!(out.next_step.contains("code-implementer"));
    assert!(out.prompt_path.is_file());
    assert!(out.task_path.is_file());
    assert!(!out.dna.is_empty());
}

#[test]
fn spawn_preserves_explicit_agent_id() {
    set_noop();
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    minimal_kit(root);

    let task_path = write_task(
        root,
        r#"
[task]
role = "edit-local"
agent-id = "ag-edit-local-explicit-12345"

[body]
text = "Explicit id test."
"#,
    );

    let out = spawn_from_task(&task_path, root).expect("spawn should succeed");
    assert_eq!(out.agent_id, "ag-edit-local-explicit-12345");
    assert_eq!(out.branch, "agent/ag-edit-local-explicit-12345");
}

#[test]
fn spawn_rejects_unknown_role() {
    set_noop();
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();

    let task_path = write_task(
        root,
        r#"
[task]
role = "does-not-exist"

[body]
text = "x"
"#,
    );

    let err = spawn_from_task(&task_path, root).expect_err("unknown role must fail");
    let msg = format!("{err:#}");
    assert!(
        msg.contains("role") || msg.contains("does-not-exist"),
        "error should reference the role: {msg}"
    );
}

#[test]
fn spawn_refuses_non_spawnable_role() {
    set_noop();
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();

    write_role(
        root,
        "git-ops",
        r#"
[role]
name = "git-ops"
spawnable = false

[capabilities]
required = []
"#,
    );

    let task_path = write_task(
        root,
        r#"
[task]
role = "git-ops"

[body]
text = "should refuse"
"#,
    );

    let err = spawn_from_task(&task_path, root).expect_err("git-ops must be refused");
    let msg = format!("{err:#}");
    assert!(msg.contains("RULE 0.13"), "refusal must cite RULE 0.13: {msg}");
}

#[test]
fn verify_fails_when_task_missing() {
    set_noop();
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    minimal_kit(root);

    let worktree = tmp.path().join("wt");
    std::fs::create_dir_all(&worktree).unwrap();

    let err = verify_agent("ag-does-not-exist", &worktree, root)
        .expect_err("missing task.toml must fail");
    let msg = format!("{err:#}");
    assert!(msg.contains("task.toml not found"), "msg: {msg}");
}

#[test]
fn spawn_then_verify_end_to_end() {
    set_noop();
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    minimal_kit(root);

    let task_path = write_task(
        root,
        r#"
[task]
role = "edit-local"

[body]
text = "Round-trip test."
"#,
    );

    let spawned = spawn_from_task(&task_path, root).expect("spawn");
    // worktree doesn't need real content — the two capabilities in
    // minimal_kit have no verify implementations, so the report is clean.
    let worktree = tmp.path().join("wt");
    std::fs::create_dir_all(&worktree).unwrap();

    let verified = verify_agent(&spawned.agent_id, &worktree, root).expect("verify");
    assert_eq!(verified.agent_id, spawned.agent_id);
    assert!(verified.is_clean, "failed: {:?}", verified.failed);
}
