//! Integration tests for kei-fork — hermetic, ledger skipped.
//!
//! Each test spins up a fresh `TempDir`, runs `git init` + initial
//! commit, then drives the public API. `KEI_FORK_SKIP_LEDGER=1` keeps
//! the test tree free of SQLite side-effects.
//!
//! NOTE: `KEI_FORK_SKIP_LEDGER` is process-wide. Tests set it once in
//! `setup_kit()` — do not unset mid-test.
//!
//! Tests that mutate *other* env vars (`KEI_FORK_FORCE_LEDGER_FAIL`,
//! `KEI_FORK_GIT_BIN`) serialize against all other tests via the
//! `ENV_LOCK` mutex below — cargo test runs in parallel by default and
//! leaking a binary override into a peer test would be catastrophic.

use kei_fork::{collect, create, gc, list, rescue, ForkStatus};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, MutexGuard};
use tempfile::TempDir;

/// Serializes every test in this binary. Cargo runs tests in parallel
/// by default; two parallel tests with different `KEI_FORK_GIT_BIN` or
/// `KEI_FORK_FORCE_LEDGER_FAIL` settings would corrupt each other.
/// `setup_kit()` returns the guard — its lifetime is the test body.
static ENV_LOCK: Mutex<()> = Mutex::new(());

type KitGuard = MutexGuard<'static, ()>;

fn setup_kit() -> (TempDir, PathBuf, KitGuard) {
    // `lock()` returns a poisoned guard if a previous test panicked —
    // we still want to run, so recover the guard unconditionally.
    let guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    // Defensive: clear any env the previous test may have left behind.
    std::env::remove_var("KEI_FORK_FORCE_LEDGER_FAIL");
    std::env::remove_var("KEI_FORK_GIT_BIN");
    std::env::set_var("KEI_FORK_SKIP_LEDGER", "1");
    let td = TempDir::new().expect("tempdir");
    let root = td.path().to_path_buf();
    run_git(&root, &["init", "-q", "-b", "main"]);
    run_git(&root, &["config", "user.email", "t@example.com"]);
    run_git(&root, &["config", "user.name", "Test"]);
    run_git(&root, &["config", "commit.gpgsign", "false"]);
    fs::write(root.join("README.md"), "hi").unwrap();
    run_git(&root, &["add", "README.md"]);
    run_git(&root, &["commit", "-q", "-m", "init"]);
    (td, root, guard)
}

fn run_git(cwd: &Path, args: &[&str]) {
    let out = Command::new("git")
        .current_dir(cwd)
        .args(args)
        .output()
        .expect("git runnable");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

fn mark_done(worktree: &Path) {
    fs::write(worktree.join(".DONE"), "").unwrap();
    // Add one real artefact so collect has something to commit.
    fs::write(worktree.join("hello.txt"), "world").unwrap();
}

#[test]
fn create_produces_worktree_and_branch() {
    let (_td, root, _g) = setup_kit();
    let h = create("ag-one", "main", &root).expect("create ok");
    assert_eq!(h.agent_id, "ag-one");
    assert_eq!(h.branch, "fork/ag-one");
    assert!(h.worktree.exists());
    assert!(h.worktree.join(".KEI_FORK_META.toml").exists());
    let br = Command::new("git")
        .current_dir(&root)
        .args(["branch", "--list", "fork/ag-one"])
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&br.stdout).contains("fork/ag-one"));
}

#[test]
fn create_rejects_invalid_agent_id() {
    let (_td, root, _g) = setup_kit();
    let err = create("../evil", "main", &root).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("invalid agent-id"), "got: {msg}");
}

#[test]
fn create_rejects_duplicate_agent_id() {
    let (_td, root, _g) = setup_kit();
    create("ag-dup", "main", &root).expect("first create");
    let err = create("ag-dup", "main", &root).unwrap_err();
    assert!(err.to_string().contains("already exists"));
}

#[test]
fn create_writes_meta_toml() {
    let (_td, root, _g) = setup_kit();
    let h = create("ag-meta", "main", &root).expect("create ok");
    let raw = fs::read_to_string(h.worktree.join(".KEI_FORK_META.toml")).unwrap();
    let parsed: toml::Value = toml::from_str(&raw).unwrap();
    assert_eq!(parsed["agent_id"].as_str(), Some("ag-meta"));
    assert_eq!(parsed["base_branch"].as_str(), Some("main"));
    assert!(parsed["started_ts"].as_integer().unwrap() > 0);
    assert_eq!(parsed["ledger_id"].as_str(), Some("ag-meta"));
}

#[test]
fn collect_without_done_fails() {
    let (_td, root, _g) = setup_kit();
    create("ag-nodone", "main", &root).unwrap();
    let err = collect("ag-nodone", "msg", &root).unwrap_err();
    assert!(err.to_string().contains(".DONE"));
}

#[test]
fn collect_with_done_produces_merge_commit() {
    let (_td, root, _g) = setup_kit();
    let h = create("ag-merge", "main", &root).unwrap();
    mark_done(&h.worktree);
    let report = collect("ag-merge", "feat: agent work", &root).expect("collect ok");
    assert_eq!(report.commit_sha.len(), 40);
    // HEAD of kit_root must be a merge commit with 2 parents.
    let out = Command::new("git")
        .current_dir(&root)
        .args(["log", "-1", "--pretty=%P"])
        .output()
        .unwrap();
    let parents: Vec<&str> = std::str::from_utf8(&out.stdout)
        .unwrap()
        .trim()
        .split_whitespace()
        .collect();
    assert_eq!(parents.len(), 2, "expected merge commit");
}

#[test]
fn collect_archives_worktree() {
    let (_td, root, _g) = setup_kit();
    let h = create("ag-arch", "main", &root).unwrap();
    mark_done(&h.worktree);
    let report = collect("ag-arch", "msg", &root).expect("collect ok");
    assert!(report.archive_path.exists());
    assert!(report.archive_path.starts_with(root.join("_archive/forks")));
    assert!(report.archive_path.ends_with("ag-arch"));
}

#[test]
fn collect_removes_live_worktree() {
    let (_td, root, _g) = setup_kit();
    let h = create("ag-gone", "main", &root).unwrap();
    mark_done(&h.worktree);
    collect("ag-gone", "msg", &root).expect("collect ok");
    assert!(!h.worktree.exists(), "live worktree should be gone");
}

#[test]
fn list_filters_by_status() {
    let (_td, root, _g) = setup_kit();
    // Active
    create("ag-active", "main", &root).unwrap();
    // Done (mark .DONE but do not collect)
    let h_done = create("ag-done", "main", &root).unwrap();
    fs::write(h_done.worktree.join(".DONE"), "").unwrap();
    // Merged (collect one)
    let h_merged = create("ag-merged", "main", &root).unwrap();
    mark_done(&h_merged.worktree);
    collect("ag-merged", "msg", &root).unwrap();

    let all = list(&root, None).unwrap();
    assert_eq!(all.len(), 3);

    let active = list(&root, Some(ForkStatus::Active)).unwrap();
    assert_eq!(active.len(), 1);
    assert_eq!(active[0].agent_id, "ag-active");

    let done = list(&root, Some(ForkStatus::Done)).unwrap();
    assert_eq!(done.len(), 1);
    assert_eq!(done[0].agent_id, "ag-done");

    let merged = list(&root, Some(ForkStatus::Merged)).unwrap();
    assert_eq!(merged.len(), 1);
    assert_eq!(merged[0].agent_id, "ag-merged");
}

#[test]
fn gc_prunes_stale() {
    let (_td, root, _g) = setup_kit();
    let h = create("ag-stale", "main", &root).unwrap();
    // Backdate meta.started_ts by 48h — no .DONE → STALE under 24h threshold.
    let raw = fs::read_to_string(h.worktree.join(".KEI_FORK_META.toml")).unwrap();
    let mut parsed: toml::Value = toml::from_str(&raw).unwrap();
    let old_ts = parsed["started_ts"].as_integer().unwrap() - 48 * 3600;
    parsed.as_table_mut().unwrap().insert(
        "started_ts".to_string(),
        toml::Value::Integer(old_ts),
    );
    fs::write(
        h.worktree.join(".KEI_FORK_META.toml"),
        toml::to_string(&parsed).unwrap(),
    )
    .unwrap();

    let report = gc(&root, 24).unwrap();
    assert_eq!(report.pruned, vec!["ag-stale".to_string()]);
    assert!(!h.worktree.exists());
}

#[test]
fn rescue_copies_live_files() {
    let (_td, root, _g) = setup_kit();
    let h = create("ag-rescue-live", "main", &root).unwrap();
    fs::write(h.worktree.join("note.txt"), "payload").unwrap();
    fs::create_dir_all(h.worktree.join("sub")).unwrap();
    fs::write(h.worktree.join("sub/nested.txt"), "deep").unwrap();

    let out_dir = root.join("rescue-out");
    let n = rescue("ag-rescue-live", &root, &out_dir).unwrap();
    assert!(n >= 3, "expected ≥3 files, got {n}");
    assert_eq!(
        fs::read_to_string(out_dir.join("note.txt")).unwrap(),
        "payload"
    );
    assert_eq!(
        fs::read_to_string(out_dir.join("sub/nested.txt")).unwrap(),
        "deep"
    );
}

#[test]
fn rescue_extracts_archived() {
    let (_td, root, _g) = setup_kit();
    let h = create("ag-rescue-arch", "main", &root).unwrap();
    mark_done(&h.worktree);
    fs::write(h.worktree.join("artefact.md"), "# hi").unwrap();
    collect("ag-rescue-arch", "msg", &root).unwrap();

    let out_dir = root.join("rescue-out-arch");
    let n = rescue("ag-rescue-arch", &root, &out_dir).unwrap();
    assert!(n >= 1);
    assert!(out_dir.join("artefact.md").exists());
}

#[test]
fn rescue_missing_agent_errors() {
    let (_td, root, _g) = setup_kit();
    let err = rescue("ag-nope", &root, &root.join("x")).unwrap_err();
    assert!(err.to_string().contains("no live or archived"));
}

// ---------------------------------------------------------------------
// Regression tests for HIGH findings (Critic F1 / F7a, Security #3 / #4).
// One test per finding + helpers shared via `setup_kit()` above.
// ---------------------------------------------------------------------

/// HIGH #1 (Critic F1): `git add -A` used to bleed `.DONE` and
/// `.KEI_FORK_META.toml` into every fork commit. The explicit-path
/// staging strategy must exclude them.
#[test]
fn collect_does_not_stage_done_or_meta() {
    let (_td, root, _g) = setup_kit();
    let h = create("ag-no-bleed", "main", &root).unwrap();
    mark_done(&h.worktree);
    let report = collect("ag-no-bleed", "feat: agent work", &root).expect("collect ok");

    // Snapshot files introduced by the merge commit itself (second parent).
    let out = Command::new("git")
        .current_dir(&root)
        .args(["show", "--name-only", "--pretty=", &report.commit_sha])
        .output()
        .unwrap();
    let listing = String::from_utf8_lossy(&out.stdout).into_owned();
    assert!(
        listing.contains("hello.txt"),
        "expected agent artefact in commit, got: {listing}"
    );
    assert!(
        !listing.contains(".DONE"),
        ".DONE must NOT be staged, got: {listing}"
    );
    assert!(
        !listing.contains(".KEI_FORK_META.toml"),
        "meta must NOT be staged, got: {listing}"
    );
}

/// HIGH #1 secondary: kit-internal prefixes (`_forks/`, `_archive/`)
/// must also be filtered out even if the agent writes into them.
#[test]
fn collect_does_not_stage_kit_internal_prefixes() {
    let (_td, root, _g) = setup_kit();
    let h = create("ag-no-kit", "main", &root).unwrap();
    mark_done(&h.worktree);
    // Agent accidentally writes into a kit-internal path.
    fs::create_dir_all(h.worktree.join("_archive/forks/oops")).unwrap();
    fs::write(h.worktree.join("_archive/forks/oops/evil.txt"), "nope").unwrap();

    let report = collect("ag-no-kit", "msg", &root).expect("collect ok");
    let out = Command::new("git")
        .current_dir(&root)
        .args(["show", "--name-only", "--pretty=", &report.commit_sha])
        .output()
        .unwrap();
    let listing = String::from_utf8_lossy(&out.stdout).into_owned();
    assert!(
        !listing.contains("_archive/"),
        "_archive/** must be excluded, got: {listing}"
    );
}

/// HIGH #2 (Critic F7a): `create()` must roll back the worktree and
/// branch if a post-worktree-add step fails. We force a ledger error
/// via `KEI_FORK_FORCE_LEDGER_FAIL=1` and assert there is no residue.
#[test]
fn create_rolls_back_on_ledger_failure() {
    let (_td, root, _g) = setup_kit();
    std::env::set_var("KEI_FORK_FORCE_LEDGER_FAIL", "1");
    let err = create("ag-rollback", "main", &root).unwrap_err();
    std::env::remove_var("KEI_FORK_FORCE_LEDGER_FAIL");

    assert!(
        err.to_string().contains("ledger"),
        "expected ledger error, got: {err}"
    );
    // Worktree dir is gone.
    assert!(
        !root.join("_forks/ag-rollback").exists(),
        "worktree should be rolled back"
    );
    // Branch is gone.
    let br = Command::new("git")
        .current_dir(&root)
        .args(["branch", "--list", "fork/ag-rollback"])
        .output()
        .unwrap();
    assert!(
        String::from_utf8_lossy(&br.stdout).trim().is_empty(),
        "branch should be deleted, got: {}",
        String::from_utf8_lossy(&br.stdout)
    );
    // And a retry MUST now succeed (no Duplicate error).
    create("ag-rollback", "main", &root).expect("retry after rollback");
}

/// HIGH #3 (Security #3): refname validator must reject a
/// base-branch that starts with `-` (would be parsed as an option).
#[test]
fn create_rejects_base_branch_starting_with_dash() {
    let (_td, root, _g) = setup_kit();
    let err = create("ag-dash", "--evil-flag", &root).unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("invalid ref name"),
        "expected InvalidRef error, got: {msg}"
    );
}

/// HIGH #3: NUL byte inside the refname must be rejected before git
/// sees it.
#[test]
fn create_rejects_base_branch_with_nul() {
    let (_td, root, _g) = setup_kit();
    let err = create("ag-nul", "main\0evil", &root).unwrap_err();
    assert!(err.to_string().contains("invalid ref name"));
}

/// HIGH #3: dotty traversal also rejected.
#[test]
fn create_rejects_base_branch_with_dot_dot() {
    let (_td, root, _g) = setup_kit();
    let err = create("ag-dots", "foo..bar", &root).unwrap_err();
    assert!(err.to_string().contains("invalid ref name"));
}

/// HIGH #4 (Security #4): the `git` binary MUST be resolvable via the
/// `KEI_FORK_GIT_BIN` env var. Point it at `false(1)` and confirm that
/// `create()` fails at the first git call (proving the env was
/// honoured). Any residue is cleaned up by `setup_kit()` of the next
/// test via the `ENV_LOCK` + defensive `remove_var`.
#[test]
fn custom_git_bin_env_respected() {
    let (_td, root, _g) = setup_kit();
    // Portable: macOS has /usr/bin/false, Linux usually has /bin/false.
    let false_bin = ["/usr/bin/false", "/bin/false"]
        .iter()
        .find(|p| Path::new(p).exists())
        .copied()
        .expect("false(1) not found at either /bin/false or /usr/bin/false");
    std::env::set_var("KEI_FORK_GIT_BIN", false_bin);
    let err = create("ag-custombin", "main", &root).unwrap_err();
    std::env::remove_var("KEI_FORK_GIT_BIN");
    // false(1) exits 1 with no stdout — our wrapper surfaces it as
    // Error::Git. That proves the override was invoked (otherwise the
    // real `git` would have succeeded, and `create` would have
    // returned Ok).
    let msg = err.to_string();
    assert!(
        msg.contains("git command failed") || msg.contains("git"),
        "expected git error, got: {msg}"
    );
}
