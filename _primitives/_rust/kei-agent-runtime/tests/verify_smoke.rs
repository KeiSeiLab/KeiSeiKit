//! Verify smoke tests — one happy + one fail per verify capability.
//!
//! Git-dependent verifies use an init-ed tempdir with `main` branch.

use kei_agent_runtime::capability::{RunMode, TaskSpec, VerifyContext, VerifyResult};
use kei_agent_runtime::registry;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;
use tempfile::TempDir;

/// Serialise access to env vars across parallel tests.
static ENV_LOCK: Mutex<()> = Mutex::new(());

fn vctx<'a>(
    task: &'a TaskSpec,
    worktree: &'a Path,
    main: &'a Path,
    agent_id: &'a str,
) -> VerifyContext<'a> {
    VerifyContext {
        agent_id,
        task,
        worktree_path: worktree,
        main_repo: main,
        run_mode: RunMode::Worktree,
        simulated_merge_path: None,
    }
}

fn init_git_repo(dir: &Path) {
    Command::new("git").args(["init", "-q", "-b", "main"]).current_dir(dir).output().unwrap();
    Command::new("git").args(["config", "user.email", "t@t"]).current_dir(dir).output().unwrap();
    Command::new("git").args(["config", "user.name", "t"]).current_dir(dir).output().unwrap();
    std::fs::write(dir.join("README.md"), "seed\n").unwrap();
    Command::new("git").args(["add", "."]).current_dir(dir).output().unwrap();
    Command::new("git").args(["commit", "-q", "-m", "seed"]).current_dir(dir).output().unwrap();
}

fn commit_all(dir: &Path, msg: &str) {
    Command::new("git").args(["add", "."]).current_dir(dir).output().unwrap();
    Command::new("git").args(["commit", "-q", "-m", msg]).current_dir(dir).output().unwrap();
}

#[test]
fn constructor_pattern_pass_on_small_file() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("small.rs"), "fn x() -> i32 { 1 }\n").unwrap();
    let cap = registry::get_verify("quality::constructor-pattern").unwrap();
    let task = TaskSpec::default();
    let ctx = vctx(&task, tmp.path(), tmp.path(), "t");
    assert_eq!(cap.verify(&ctx), VerifyResult::Pass);
}

#[test]
fn constructor_pattern_fails_on_large_file() {
    let tmp = TempDir::new().unwrap();
    let big = (0..250).map(|i| format!("// line {i}")).collect::<Vec<_>>().join("\n");
    std::fs::write(tmp.path().join("big.rs"), big).unwrap();
    let cap = registry::get_verify("quality::constructor-pattern").unwrap();
    let task = TaskSpec::default();
    let ctx = vctx(&task, tmp.path(), tmp.path(), "t");
    matches!(cap.verify(&ctx), VerifyResult::Fail { .. });
}

#[test]
fn constructor_pattern_fails_on_long_fn() {
    let tmp = TempDir::new().unwrap();
    let body = (0..40).map(|_| "    let _ = 0;").collect::<Vec<_>>().join("\n");
    let src = format!("fn long() {{\n{body}\n}}\n");
    std::fs::write(tmp.path().join("longfn.rs"), src).unwrap();
    let cap = registry::get_verify("quality::constructor-pattern").unwrap();
    let task = TaskSpec::default();
    let ctx = vctx(&task, tmp.path(), tmp.path(), "t");
    matches!(cap.verify(&ctx), VerifyResult::Fail { .. });
}

#[test]
fn tests_green_passes_with_no_crates_configured() {
    let tmp = TempDir::new().unwrap();
    let cap = registry::get_verify("quality::tests-green").unwrap();
    let task = TaskSpec::default(); // empty cargo-test-crates
    let ctx = vctx(&task, tmp.path(), tmp.path(), "t");
    assert_eq!(cap.verify(&ctx), VerifyResult::Pass);
}

#[test]
fn scope_whitelist_verify_passes_on_matching_diff() {
    let tmp = TempDir::new().unwrap();
    init_git_repo(tmp.path());
    std::fs::create_dir_all(tmp.path().join("allowed")).unwrap();
    std::fs::write(tmp.path().join("allowed/f.rs"), "fn x() {}\n").unwrap();
    commit_all(tmp.path(), "add");
    let mut task = TaskSpec::default();
    task.scope.files_whitelist = vec!["allowed/**".into()];
    let cap = registry::get_verify("scope::files-whitelist").unwrap();
    let ctx = vctx(&task, tmp.path(), tmp.path(), "t");
    // diff against main: no new changes beyond what's already committed → PASS trivially
    assert_eq!(cap.verify(&ctx), VerifyResult::Pass);
}

#[test]
fn scope_whitelist_verify_fails_on_outside_diff() {
    let tmp = TempDir::new().unwrap();
    init_git_repo(tmp.path());
    // Create branch off main with an outside edit
    Command::new("git")
        .args(["checkout", "-q", "-b", "feature"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    std::fs::write(tmp.path().join("outside.rs"), "fn x() {}\n").unwrap();
    commit_all(tmp.path(), "outside");
    let mut task = TaskSpec::default();
    task.scope.files_whitelist = vec!["allowed/**".into()];
    let cap = registry::get_verify("scope::files-whitelist").unwrap();
    let ctx = vctx(&task, tmp.path(), tmp.path(), "t");
    matches!(cap.verify(&ctx), VerifyResult::Fail { .. });
}

#[test]
fn scope_denylist_verify_fails_on_denied_path() {
    let tmp = TempDir::new().unwrap();
    init_git_repo(tmp.path());
    Command::new("git")
        .args(["checkout", "-q", "-b", "feature"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    std::fs::write(tmp.path().join("Cargo.toml"), "[package]\n").unwrap();
    commit_all(tmp.path(), "bad");
    let mut task = TaskSpec::default();
    task.scope.files_denylist = vec!["Cargo.toml".into()];
    let cap = registry::get_verify("scope::files-denylist").unwrap();
    let ctx = vctx(&task, tmp.path(), tmp.path(), "t");
    matches!(cap.verify(&ctx), VerifyResult::Fail { .. });
}

#[test]
fn safety_no_dep_bump_verify_passes_with_no_version_diff() {
    let tmp = TempDir::new().unwrap();
    init_git_repo(tmp.path());
    let task = TaskSpec::default();
    let cap = registry::get_verify("safety::no-dep-bump").unwrap();
    let ctx = vctx(&task, tmp.path(), tmp.path(), "t");
    assert_eq!(cap.verify(&ctx), VerifyResult::Pass);
}

#[test]
fn safety_no_dep_bump_verify_fails_on_version_diff() {
    let tmp = TempDir::new().unwrap();
    init_git_repo(tmp.path());
    std::fs::write(tmp.path().join("Cargo.toml"), "version = \"0.1.0\"\n").unwrap();
    commit_all(tmp.path(), "seed version");
    Command::new("git")
        .args(["checkout", "-q", "-b", "feature"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    std::fs::write(tmp.path().join("Cargo.toml"), "version = \"0.2.0\"\n").unwrap();
    commit_all(tmp.path(), "bump");
    let task = TaskSpec::default();
    let cap = registry::get_verify("safety::no-dep-bump").unwrap();
    let ctx = vctx(&task, tmp.path(), tmp.path(), "t");
    matches!(cap.verify(&ctx), VerifyResult::Fail { .. });
}

#[test]
fn output_report_format_passes_when_fields_present() {
    let _guard = ENV_LOCK.lock().unwrap();
    let tmp = TempDir::new().unwrap();
    let report_path: PathBuf = tmp.path().join("report.md");
    std::fs::write(
        &report_path,
        "## Summary\n\nfiles-touched: 3\ncargo-check: PASS\n",
    )
    .unwrap();
    std::env::set_var("AGENT_REPORT_PATH", &report_path);
    let mut task = TaskSpec::default();
    task.output.report_fields_required =
        vec!["files-touched".into(), "cargo-check".into()];
    let cap = registry::get_verify("output::report-format").unwrap();
    let ctx = vctx(&task, tmp.path(), tmp.path(), "t");
    let r = cap.verify(&ctx);
    std::env::remove_var("AGENT_REPORT_PATH");
    assert_eq!(r, VerifyResult::Pass);
}

#[test]
fn output_report_format_fails_when_missing() {
    let _guard = ENV_LOCK.lock().unwrap();
    let tmp = TempDir::new().unwrap();
    let report_path: PathBuf = tmp.path().join("report.md");
    std::fs::write(&report_path, "only summary").unwrap();
    std::env::set_var("AGENT_REPORT_PATH", &report_path);
    let mut task = TaskSpec::default();
    task.output.report_fields_required = vec!["files-touched".into()];
    let cap = registry::get_verify("output::report-format").unwrap();
    let ctx = vctx(&task, tmp.path(), tmp.path(), "t");
    let r = cap.verify(&ctx);
    std::env::remove_var("AGENT_REPORT_PATH");
    match r {
        VerifyResult::Fail { .. } => {}
        other => panic!("expected Fail, got {other:?}"),
    }
}

#[test]
fn output_severity_grade_accepts_high() {
    let _guard = ENV_LOCK.lock().unwrap();
    let tmp = TempDir::new().unwrap();
    let report_path: PathBuf = tmp.path().join("r.md");
    std::fs::write(&report_path, "**HIGH**: foo\n").unwrap();
    std::env::set_var("AGENT_REPORT_PATH", &report_path);
    let task = TaskSpec::default();
    let cap = registry::get_verify("output::severity-grade").unwrap();
    let ctx = vctx(&task, tmp.path(), tmp.path(), "t");
    let r = cap.verify(&ctx);
    std::env::remove_var("AGENT_REPORT_PATH");
    assert_eq!(r, VerifyResult::Pass);
}
