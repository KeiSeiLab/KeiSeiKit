//! pipeline_unit — fine-grained coverage for pipeline helpers + the
//! spawn-rollback-on-ledger-failure contract.
//!
//! Complements pipeline_smoke.rs (end-to-end) with unit-level assertions
//! that don't need the full spawn pipeline.

use kei_spawn::{
    derive_steps, emit_pipeline_json, pipeline_json_path, spawn_from_task, PipelineChain,
    PipelineStep,
};
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

#[test]
fn derive_steps_child_ids_distinct() {
    let roles = vec!["auditor".to_string(), "merger".to_string()];
    let steps = derive_steps("ag-edit-local-zzz", &roles);
    assert_eq!(steps.len(), 2);
    assert_eq!(steps[0].agent_id, "ag-edit-local-zzz-auditor");
    assert_eq!(steps[1].agent_id, "ag-edit-local-zzz-merger");
    assert_ne!(steps[0].agent_id, steps[1].agent_id);
}

#[test]
fn derive_steps_skips_empty_role_names() {
    let roles = vec![
        "auditor".to_string(),
        "   ".to_string(),
        "".to_string(),
        "merger".to_string(),
    ];
    let steps = derive_steps("ag-writer-001", &roles);
    assert_eq!(steps.len(), 2);
    assert_eq!(steps[0].role, "auditor");
    assert_eq!(steps[1].role, "merger");
}

#[test]
fn emit_pipeline_json_creates_parent_dir() {
    let tmp = TempDir::new().unwrap();
    let nested = tmp.path().join("a").join("b").join("pipeline.json");
    let chain = PipelineChain {
        steps: vec![PipelineStep {
            role: "auditor".into(),
            agent_id: "ag-x-auditor".into(),
        }],
    };
    emit_pipeline_json(&nested, &chain).expect("emit");
    assert!(nested.is_file(), "{} should exist", nested.display());
    let body = std::fs::read_to_string(&nested).unwrap();
    assert!(body.contains("\"auditor\""), "json: {body}");
    assert!(body.contains("\"ag-x-auditor\""), "json: {body}");
}

#[test]
fn pipeline_json_path_uses_convention() {
    let root = Path::new("/tmp/kit");
    let path = pipeline_json_path(root, "ag-writer-42");
    assert_eq!(
        path,
        Path::new("/tmp/kit/tasks/ag-writer-42/pipeline.json")
    );
}

#[test]
fn precedent_check_env_gated_off_silent() {
    // Ensure env flag absent → run_advisory returns Ok(0) without shelling out.
    std::env::remove_var("KEI_SPAWN_PRECEDENT_CHECK");
    let n = kei_spawn::precedent::run_advisory("00".repeat(32).as_str()).unwrap();
    assert_eq!(n, 0);
}

#[test]
fn spawn_rolls_back_task_dir_on_ledger_fail() {
    // Force ledger failure by pointing at a bogus binary AND clearing the
    // noop escape hatch so the subprocess actually runs (and fails).
    std::env::remove_var("KEI_SPAWN_LEDGER_NOOP");
    std::env::set_var("KEI_LEDGER_BIN", "/nonexistent/kei-ledger-rollback-test");

    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    minimal_kit(root);
    let task_path = root.join("task.toml");
    std::fs::write(
        &task_path,
        r#"
[task]
role = "edit-local"
agent-id = "ag-edit-local-rollback-001"

[body]
text = "Rollback test."
"#,
    )
    .unwrap();

    let result = spawn_from_task(&task_path, root);
    assert!(result.is_err(), "ledger fail must propagate");

    let agent_dir = root.join("tasks").join("ag-edit-local-rollback-001");
    assert!(
        !agent_dir.exists(),
        "task dir must be cleaned up after ledger failure; still exists at {}",
        agent_dir.display()
    );

    // Restore ledger env so other tests in the crate don't see the bogus path.
    std::env::remove_var("KEI_LEDGER_BIN");
    std::env::set_var("KEI_SPAWN_LEDGER_NOOP", "1");
}
