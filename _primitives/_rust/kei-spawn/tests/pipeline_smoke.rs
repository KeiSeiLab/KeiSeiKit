//! pipeline_smoke — integration tests for `spawn --pipeline` end-to-end.
//!
//! Same pattern as spawn_smoke.rs: minimal tempdir kit, role + capability
//! fixtures, then call the library surface and assert on-disk artefacts.
//! `KEI_SPAWN_LEDGER_NOOP=1` keeps the ledger subprocess a no-op so tests
//! do not depend on a real kei-ledger binary.

use kei_spawn::{
    derive_chain_from_role, pipeline_from_role, spawn_with_pipeline, PipelineChain,
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

fn write_task(root: &Path, toml: &str) -> std::path::PathBuf {
    let path = root.join("task.toml");
    std::fs::write(&path, toml).unwrap();
    path
}

fn minimal_kit_with_handoff(root: &Path) {
    write_capability(root, "policy", "no-git-ops", "## Never git.\n");
    write_capability(root, "output", "report-format", "## Report fields.\n");
    write_capability(root, "scope", "read-only", "## Read-only.\n");
    write_capability(root, "output", "verdict", "## Verdict.\n");
    write_capability(root, "verify", "fork-audit", "## Fork audit.\n");
    write_capability(root, "policy", "git-ops-scope", "## Git ops scope.\n");
    write_capability(root, "output", "merge-result", "## Merge result.\n");

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

[pipeline]
handoff = ["auditor"]
"#,
    );
    write_role(
        root,
        "auditor",
        r#"
[role]
name = "auditor"
spawnable = true
claude-subagent-type = "critic"

[capabilities]
required = ["policy::no-git-ops", "scope::read-only", "verify::fork-audit", "output::verdict"]

[tools]
allowed = ["Read", "Glob", "Grep", "Bash"]
bash-patterns-allowed = ['^cargo( |$)','^git diff','^git log','^git show']

[pipeline]
handoff = ["merger"]
"#,
    );
    write_role(
        root,
        "merger",
        r#"
[role]
name = "merger"
spawnable = true
claude-subagent-type = "infra-implementer"

[capabilities]
required = ["policy::git-ops-scope", "output::merge-result"]

[tools]
allowed = ["Read", "Bash"]
bash-patterns-allowed = ['^git( |$)','^kei-fork( |$)','^kei-ledger( |$)']

[pipeline]
handoff = []
"#,
    );
}

fn set_noop() {
    std::env::set_var("KEI_SPAWN_LEDGER_NOOP", "1");
}

#[test]
fn auditor_role_resolves() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    minimal_kit_with_handoff(root);
    let handoff = pipeline_from_role(root, "auditor").expect("auditor handoff");
    assert_eq!(handoff, vec!["merger".to_string()]);
}

#[test]
fn merger_role_resolves() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    minimal_kit_with_handoff(root);
    let handoff = pipeline_from_role(root, "merger").expect("merger handoff");
    assert!(handoff.is_empty(), "merger is the terminal step: {handoff:?}");
}

#[test]
fn pipeline_handoff_produces_chain() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    minimal_kit_with_handoff(root);
    let chain: PipelineChain =
        derive_chain_from_role(root, "edit-local", "ag-edit-local-test-001").unwrap();
    assert_eq!(chain.steps.len(), 1);
    assert_eq!(chain.steps[0].role, "auditor");
    assert_eq!(chain.steps[0].agent_id, "ag-edit-local-test-001-auditor");
}

#[test]
fn pipeline_missing_role_returns_empty() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    minimal_kit_with_handoff(root);
    // merger exists but has empty handoff — empty Vec, not error.
    let chain = derive_chain_from_role(root, "merger", "ag-merger-test-001").unwrap();
    assert!(chain.steps.is_empty());

    // unknown role — must error cleanly.
    let err = derive_chain_from_role(root, "does-not-exist", "ag-x-001")
        .expect_err("unknown role must error");
    let msg = format!("{err:#}");
    assert!(msg.contains("does-not-exist"), "msg: {msg}");
}

#[test]
fn spawn_with_pipeline_scaffolds_downstream_stubs() {
    set_noop();
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    minimal_kit_with_handoff(root);

    let task_path = write_task(
        root,
        r#"
[task]
role = "edit-local"

[body]
text = "Writer body that hands off to auditor."
"#,
    );

    let (out, chain) = spawn_with_pipeline(&task_path, root).expect("spawn_with_pipeline");
    assert_eq!(chain.steps.len(), 1);
    assert_eq!(chain.steps[0].role, "auditor");
    assert_eq!(chain.steps[0].agent_id, format!("{}-auditor", out.agent_id));

    let writer_dir = root.join("tasks").join(&out.agent_id);
    assert!(writer_dir.join("pipeline.json").is_file());

    let auditor_dir = root.join("tasks").join(format!("{}-auditor", out.agent_id));
    assert!(auditor_dir.join("task.stub.toml").is_file());
    let stub_text = std::fs::read_to_string(auditor_dir.join("task.stub.toml")).unwrap();
    assert!(stub_text.contains(r#"role = "auditor""#));
    assert!(stub_text.contains(&format!(r#"parent-agent = "{}""#, out.agent_id)));
}
