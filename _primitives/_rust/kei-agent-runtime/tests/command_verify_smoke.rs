//! Smoke tests for the generic `CommandVerify` (Layer D convergence).
//!
//! Covers default exit-code mapper (pass + fail), custom runner path,
//! and WorkDir resolution.

use kei_agent_runtime::capability::{Capability, RunMode, TaskSpec, VerifyContext, VerifyResult};
use kei_agent_runtime::verifies::command_verify::{CommandVerify, WorkDir};
use std::path::Path;
use tempfile::TempDir;

fn vctx<'a>(task: &'a TaskSpec, worktree: &'a Path, agent_id: &'a str) -> VerifyContext<'a> {
    VerifyContext {
        agent_id,
        task,
        worktree_path: worktree,
        main_repo: worktree,
        run_mode: RunMode::Worktree,
        simulated_merge_path: None,
    }
}

const TRUE_VERIFY: CommandVerify = CommandVerify {
    name: "test::true",
    program: "true",
    base_args: &[],
    work_dir: WorkDir::RunDir,
    expected_exit: 0,
    fail_reason: "true somehow failed",
    custom_runner: None,
    arg_builder: None,
    result_mapper: None,
};

const FALSE_VERIFY: CommandVerify = CommandVerify {
    name: "test::false",
    program: "false",
    base_args: &[],
    work_dir: WorkDir::RunDir,
    expected_exit: 0,
    fail_reason: "false returned non-zero",
    custom_runner: None,
    arg_builder: None,
    result_mapper: None,
};

#[test]
fn default_runner_passes_on_zero_exit() {
    let tmp = TempDir::new().unwrap();
    let task = TaskSpec::default();
    let ctx = vctx(&task, tmp.path(), "t");
    assert_eq!(TRUE_VERIFY.verify(&ctx), VerifyResult::Pass);
}

#[test]
fn default_runner_fails_on_nonzero_exit() {
    let tmp = TempDir::new().unwrap();
    let task = TaskSpec::default();
    let ctx = vctx(&task, tmp.path(), "t");
    match FALSE_VERIFY.verify(&ctx) {
        VerifyResult::Fail { .. } => {}
        other => panic!("expected Fail, got {other:?}"),
    }
}

#[test]
fn custom_runner_is_invoked() {
    const MARKER_VERIFY: CommandVerify = CommandVerify {
        name: "test::custom",
        program: "/does/not/matter",
        base_args: &[],
        work_dir: WorkDir::RunDir,
        expected_exit: 0,
        fail_reason: "",
        custom_runner: Some(|_cv, _ctx| VerifyResult::Pass),
        arg_builder: None,
        result_mapper: None,
    };
    let tmp = TempDir::new().unwrap();
    let task = TaskSpec::default();
    let ctx = vctx(&task, tmp.path(), "t");
    assert_eq!(MARKER_VERIFY.verify(&ctx), VerifyResult::Pass);
}

#[test]
fn workspace_root_resolves_to_primitives_rust_when_present() {
    let tmp = TempDir::new().unwrap();
    let prims = tmp.path().join("_primitives/_rust");
    std::fs::create_dir_all(&prims).unwrap();
    let cv = CommandVerify {
        name: "test::dir",
        program: "true",
        base_args: &[],
        work_dir: WorkDir::WorkspaceRoot,
        expected_exit: 0,
        fail_reason: "",
        custom_runner: None,
        arg_builder: None,
        result_mapper: None,
    };
    let task = TaskSpec::default();
    let ctx = vctx(&task, tmp.path(), "t");
    let resolved = cv.resolve_dir(&ctx);
    assert_eq!(resolved, prims);
}

#[test]
fn workspace_root_falls_back_to_run_dir() {
    let tmp = TempDir::new().unwrap();
    let cv = CommandVerify {
        name: "test::dir-fallback",
        program: "true",
        base_args: &[],
        work_dir: WorkDir::WorkspaceRoot,
        expected_exit: 0,
        fail_reason: "",
        custom_runner: None,
        arg_builder: None,
        result_mapper: None,
    };
    let task = TaskSpec::default();
    let ctx = vctx(&task, tmp.path(), "t");
    let resolved = cv.resolve_dir(&ctx);
    assert_eq!(resolved, tmp.path().to_path_buf());
}

#[test]
fn cargo_check_green_const_is_registered() {
    use kei_agent_runtime::registry;
    let c = registry::get_verify("quality::cargo-check-green").unwrap();
    assert_eq!(c.name(), "quality::cargo-check-green");
}
