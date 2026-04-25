//! drive_smoke — integration tests for `kei-spawn drive` subcommand.
//!
//! The drive subcommand shells the full pipeline:
//!   1. spawn_from_task (prepare + ledger fork)
//!   2. SpawnOutput JSON → stdout
//!   3. ManualDriver::invoke → NotImplemented → stderr + exit 64
//!
//! Because exit-code assertions require invoking the real binary, these
//! tests use `CARGO_BIN_EXE_kei-spawn` (populated by cargo for integration
//! tests) with `KEI_SPAWN_LEDGER_NOOP=1` set so the ledger subprocess is
//! a no-op.

use std::path::Path;
use std::process::Command;
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

fn bin() -> Command {
    let mut c = Command::new(env!("CARGO_BIN_EXE_kei-spawn"));
    c.env("KEI_SPAWN_LEDGER_NOOP", "1");
    c
}

#[test]
fn drive_on_valid_task_emits_spawn_json_and_exits_64() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    minimal_kit(root);
    let task_path = write_task(
        root,
        r#"
[task]
role = "edit-local"

[body]
text = "Drive smoke test."
"#,
    );

    let output = bin()
        .arg("drive")
        .arg(&task_path)
        .arg("--kit-root")
        .arg(root)
        .output()
        .expect("run kei-spawn drive");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    // Stdout must be SpawnOutput JSON (contain agent_id + prompt fields).
    assert!(stdout.contains("\"agent_id\""), "stdout missing agent_id: {stdout}");
    assert!(stdout.contains("\"subagent_type\""), "stdout missing subagent_type: {stdout}");
    assert!(stdout.contains("\"prompt\""), "stdout missing prompt: {stdout}");
    // Stderr must explain the v0.1 NotImplemented state.
    assert!(
        stderr.contains("HTTP Anthropic-API integration not yet wired"),
        "stderr missing NotImplemented msg: {stderr}"
    );
    // Exit code 64 (EX_USAGE range, NotImplemented convention).
    assert_eq!(output.status.code(), Some(64), "exit code must be 64, got: {:?}; stderr={stderr}", output.status.code());
}

#[test]
fn drive_on_unknown_role_exits_nonzero_with_spawn_error() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    // NO minimal_kit — role cannot resolve.
    let task_path = write_task(
        root,
        r#"
[task]
role = "ghost-role"

[body]
text = "x"
"#,
    );

    let output = bin()
        .arg("drive")
        .arg(&task_path)
        .arg("--kit-root")
        .arg(root)
        .output()
        .expect("run kei-spawn drive");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("drive") || stderr.contains("role") || stderr.contains("ghost-role"),
        "stderr should reference the spawn failure: {stderr}"
    );
    // Spawn error surfaces as exit 1 (not 64); must NOT be success.
    let code = output.status.code();
    assert_ne!(code, Some(0), "unknown role must fail");
    assert_ne!(code, Some(64), "unknown role must not be NotImplemented: stderr={stderr}");
}
