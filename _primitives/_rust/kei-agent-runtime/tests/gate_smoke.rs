//! Gate smoke tests — one happy + one deny + one bypass/boundary per gate.

use kei_agent_runtime::capability::{GateContext, GateDecision, TaskSpec};
use kei_agent_runtime::registry;
use serde_json::json;
use std::collections::HashMap;

fn ctx<'a>(
    tool: &'a str,
    input: &'a serde_json::Value,
    task: &'a TaskSpec,
    env: &'a HashMap<String, String>,
) -> GateContext<'a> {
    GateContext { tool_name: tool, tool_input: input, task, env }
}

fn env_empty() -> HashMap<String, String> {
    HashMap::new()
}

fn env_with(key: &str, val: &str) -> HashMap<String, String> {
    let mut m = HashMap::new();
    m.insert(key.into(), val.into());
    m
}

#[test]
fn no_git_ops_denies_git_command() {
    let g = registry::get_gate("policy::no-git-ops").unwrap();
    let task = TaskSpec::default();
    let env = env_empty();
    let input = json!({"command": "git commit -m foo"});
    match g.check(&ctx("Bash", &input, &task, &env)) {
        GateDecision::Deny { .. } => {}
        other => panic!("expected Deny, got {other:?}"),
    }
}

#[test]
fn no_git_ops_allows_non_git_bash() {
    let g = registry::get_gate("policy::no-git-ops").unwrap();
    let task = TaskSpec::default();
    let env = env_empty();
    let input = json!({"command": "cargo build"});
    assert_eq!(g.check(&ctx("Bash", &input, &task, &env)), GateDecision::Allow);
}

#[test]
fn no_git_ops_bypass_orchestrator_meta() {
    let g = registry::get_gate("policy::no-git-ops").unwrap();
    let task = TaskSpec::default();
    let env = env_with("ORCHESTRATOR_META", "1");
    let input = json!({"command": "git commit -m bypass"});
    assert_eq!(g.check(&ctx("Bash", &input, &task, &env)), GateDecision::Allow);
}

#[test]
fn deny_tools_denies_write() {
    let g = registry::get_gate("tools::deny-tools").unwrap();
    let task = TaskSpec::default();
    let env = env_empty();
    let input = json!({"file_path": "/tmp/foo.rs"});
    matches!(g.check(&ctx("Write", &input, &task, &env)), GateDecision::Deny { .. });
    matches!(g.check(&ctx("Edit", &input, &task, &env)), GateDecision::Deny { .. });
}

#[test]
fn deny_tools_allows_read() {
    let g = registry::get_gate("tools::deny-tools").unwrap();
    let task = TaskSpec::default();
    let env = env_empty();
    let input = json!({});
    assert_eq!(
        g.check(&ctx("Read", &input, &task, &env)),
        GateDecision::NotApplicable
    );
}

#[test]
fn bash_allowlist_allows_cargo() {
    let g = registry::get_gate("tools::bash-allowlist").unwrap();
    let task = TaskSpec::default();
    let env = env_empty();
    let input = json!({"command": "cargo test --workspace"});
    assert_eq!(g.check(&ctx("Bash", &input, &task, &env)), GateDecision::Allow);
}

#[test]
fn bash_allowlist_denies_curl() {
    let g = registry::get_gate("tools::bash-allowlist").unwrap();
    let task = TaskSpec::default();
    let env = env_empty();
    let input = json!({"command": "curl example.com"});
    matches!(
        g.check(&ctx("Bash", &input, &task, &env)),
        GateDecision::Deny { .. }
    );
}

#[test]
fn scope_whitelist_allows_matching_path() {
    let g = registry::get_gate("scope::files-whitelist").unwrap();
    let mut task = TaskSpec::default();
    task.scope.files_whitelist = vec!["_primitives/_rust/kei-forge/**".into()];
    let env = env_empty();
    let input = json!({"file_path": "_primitives/_rust/kei-forge/src/lib.rs"});
    assert_eq!(g.check(&ctx("Edit", &input, &task, &env)), GateDecision::Allow);
}

#[test]
fn scope_whitelist_denies_outside() {
    let g = registry::get_gate("scope::files-whitelist").unwrap();
    let mut task = TaskSpec::default();
    task.scope.files_whitelist = vec!["_primitives/_rust/kei-forge/**".into()];
    let env = env_empty();
    let input = json!({"file_path": "hooks/foo.sh"});
    matches!(
        g.check(&ctx("Edit", &input, &task, &env)),
        GateDecision::Deny { .. }
    );
}

#[test]
fn scope_denylist_denies_match() {
    let g = registry::get_gate("scope::files-denylist").unwrap();
    let mut task = TaskSpec::default();
    task.scope.files_denylist = vec!["_primitives/_rust/Cargo.toml".into()];
    let env = env_empty();
    let input = json!({"file_path": "_primitives/_rust/Cargo.toml"});
    matches!(
        g.check(&ctx("Edit", &input, &task, &env)),
        GateDecision::Deny { .. }
    );
}

#[test]
fn no_dep_bump_blocks_cargo_toml() {
    let g = registry::get_gate("safety::no-dep-bump").unwrap();
    let task = TaskSpec::default();
    let env = env_empty();
    let input = json!({"file_path": "foo/Cargo.toml"});
    matches!(
        g.check(&ctx("Edit", &input, &task, &env)),
        GateDecision::Deny { .. }
    );
}

#[test]
fn no_dep_bump_allow_bypass() {
    let g = registry::get_gate("safety::no-dep-bump").unwrap();
    let task = TaskSpec::default();
    let env = env_with("ALLOW_DEP_BUMP", "1");
    let input = json!({"file_path": "foo/Cargo.toml"});
    assert_eq!(g.check(&ctx("Edit", &input, &task, &env)), GateDecision::Allow);
}
