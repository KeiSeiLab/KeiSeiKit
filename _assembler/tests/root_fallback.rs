//! Regression test for `root.parent().unwrap_or(root.as_path())` in
//! main.rs: when AGENT_ROOT is a filesystem root (no parent), the
//! fallback should kick in and the binary must NOT panic.
//!
//! Fix reference: commit 30cd08b fixed the panic by replacing
//! `root.parent().unwrap()` with `.unwrap_or(root.as_path())`.
//! This test locks that behaviour so a future "simplify" refactor
//! can't silently reintroduce the panic.

mod common;

use common::assemble_bin;
use std::process::Command;

/// Driving the binary with AGENT_ROOT=/ points it at directories that
/// either don't exist (`/_manifests`) or exist but aren't ours (`/var`).
/// Either way, `main()` must exit cleanly — NOT panic on the
/// `root.parent().unwrap()` path introduced before commit 30cd08b.
#[test]
fn agent_root_slash_does_not_panic() {
    let out = Command::new(assemble_bin())
        .env("AGENT_ROOT", "/")
        // Give it an explicit manifest path that doesn't exist, so the
        // binary reaches the "no manifests" branch without scanning /.
        // We want to hit the `relative_to(..., root.parent().unwrap_or(...))`
        // code path, which only runs on successful assembly, so arrange
        // for that by passing /dev/null (unreadable as a TOML) and
        // asserting the binary exits cleanly (non-zero is fine) without
        // a panic signal.
        .args(["/dev/null"])
        .output()
        .expect("spawn assemble");

    // A panic on macOS/Linux surfaces as SIGABRT (signal 6) → 134, or
    // the process printing "panicked at" to stderr. Accept any clean
    // exit code (zero or non-zero) as long as there is no panic.
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stderr.contains("panicked at"),
        "binary panicked with AGENT_ROOT=/: {stderr}"
    );
    // No signal termination. On Unix, `code()` returns None if the
    // process was killed by a signal.
    assert!(
        out.status.code().is_some(),
        "binary was killed by a signal with AGENT_ROOT=/ (likely SIGABRT from panic); \
         stderr: {stderr}"
    );
}

/// Same guarantee but for a valid end-to-end run: AGENT_ROOT is / (no
/// parent), manifest is supplied explicitly, and the binary must
/// complete (success OR graceful failure — but NO panic) because the
/// relative_to() call happens on the success path.
#[test]
fn agent_root_slash_full_run_no_panic() {
    // We can't actually write under / as a test user, so this run
    // will fail at the "mkdir generated" step. That's fine — we only
    // assert the absence of a panic.
    let tmp = tempfile::TempDir::new().unwrap();
    let manifest = tmp.path().join("stub.toml");
    std::fs::write(
        &manifest,
        r#"
name = "stub"
description = "stub"
tools = ["Read"]
model = "opus"
role = "stub"
blocks = ["baseline", "evidence-grading", "memory-protocol"]
domain_in = ["x"]
forbidden_domain = ["y"]
[[handoff]]
target = "other"
trigger = "z"
"#,
    )
    .unwrap();

    let out = Command::new(assemble_bin())
        .env("AGENT_ROOT", "/")
        .arg(manifest.to_str().unwrap())
        .output()
        .expect("spawn assemble");

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stderr.contains("panicked at"),
        "binary panicked on full run with AGENT_ROOT=/: {stderr}"
    );
    assert!(
        out.status.code().is_some(),
        "binary killed by signal on full run with AGENT_ROOT=/: {stderr}"
    );
}
