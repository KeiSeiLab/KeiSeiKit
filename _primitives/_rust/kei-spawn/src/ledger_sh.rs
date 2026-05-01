//! Thin subprocess wrapper around the `kei-ledger` binary.
//!
//! kei-ledger is a bin-only crate (no lib.rs at the time kei-spawn was
//! introduced). We shell to it rather than replicate SQL — same process
//! model users expect, same DB file, same env contract (`KEI_LEDGER_DB`).
//!
//! Every call surfaces stderr on failure so orchestrator sees the real
//! ledger error (branch too long, duplicate id, etc.), not a wrapped one.
//!
//! # Security — `$PATH` hijack (CWE-427)
//!
//! The final fallback in [`ledger_bin`] is the bare name `"kei-ledger"`,
//! which `std::process::Command` resolves by walking `$PATH`. On a shared
//! or compromised machine an attacker-controlled directory earlier on
//! `$PATH` can provide a rogue `kei-ledger` that silently captures ledger
//! writes. To mitigate:
//!
//! * Set `KEI_LEDGER_BIN` to an **absolute path** in production / CI
//!   (e.g. `/usr/local/bin/kei-ledger` or the cargo-install path), or
//! * Run integration tests via `cargo test` which populates the
//!   `CARGO_BIN_EXE_kei-ledger` env var with the workspace-built binary.
//!
//! The env-override path is checked first in [`ledger_bin`] precisely so
//! a trusted operator can pin the binary and sidestep `$PATH` resolution.

use anyhow::{anyhow, Result};
use std::process::Command;

/// Resolve `kei-ledger` executable. Env override → CARGO env (tests) → PATH.
///
/// Lookup order:
/// 1. `KEI_LEDGER_BIN` — operator-pinned absolute path (recommended for
///    production; mitigates `$PATH` hijack per CWE-427, see module docs).
/// 2. `CARGO_BIN_EXE_kei-ledger` — set by `cargo test` for the workspace
///    binary under integration testing.
/// 3. `"kei-ledger"` — last-resort bare name; resolved via `$PATH` by
///    `std::process::Command`. Acceptable on single-user dev machines;
///    pin via `KEI_LEDGER_BIN` in any multi-user or CI context.
pub fn ledger_bin() -> String {
    if let Ok(b) = std::env::var("KEI_LEDGER_BIN") {
        return b;
    }
    // CARGO_BIN_EXE_kei-ledger is set for integration tests under workspace.
    if let Ok(b) = std::env::var("CARGO_BIN_EXE_kei-ledger") {
        return b;
    }
    "kei-ledger".into()
}

/// Test / sandbox escape hatch: when set, every ledger call is a no-op.
/// Integration tests use this to avoid needing the real kei-ledger binary
/// on PATH. Production callers MUST NOT set this env var.
fn is_noop() -> bool {
    std::env::var("KEI_SPAWN_LEDGER_NOOP")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Run `kei-ledger fork` with DNA + worktree metadata.
pub fn fork(
    id: &str,
    branch: &str,
    parent: Option<&str>,
    spec_sha: &str,
    worktree: Option<&str>,
    dna: Option<&str>,
) -> Result<()> {
    if is_noop() {
        let _ = (id, branch, parent, spec_sha, worktree, dna);
        return Ok(());
    }
    let mut cmd = Command::new(ledger_bin());
    cmd.args(["fork", id, branch, "--spec-sha", spec_sha]);
    if let Some(p) = parent {
        cmd.args(["--parent", p]);
    }
    if let Some(w) = worktree {
        cmd.args(["--worktree", w]);
    }
    if let Some(d) = dna {
        cmd.args(["--dna", d]);
    }
    run(&mut cmd, "fork")
}

/// Run `kei-ledger done <id> --summary <s>`.
pub fn done(id: &str, summary: &str) -> Result<()> {
    if is_noop() {
        let _ = (id, summary);
        return Ok(());
    }
    let mut cmd = Command::new(ledger_bin());
    cmd.args(["done", id, "--summary", summary]);
    run(&mut cmd, "done")
}

/// Run `kei-ledger fail <id> --reason <r>`.
pub fn fail(id: &str, reason: &str) -> Result<()> {
    if is_noop() {
        let _ = (id, reason);
        return Ok(());
    }
    let mut cmd = Command::new(ledger_bin());
    cmd.args(["fail", id, "--reason", reason]);
    run(&mut cmd, "fail")
}

/// Run `kei-ledger list --status running`. Returns raw stdout lines.
pub fn list_running() -> Result<String> {
    if is_noop() {
        return Ok(String::from("(noop: KEI_SPAWN_LEDGER_NOOP=1)\n"));
    }
    let mut cmd = Command::new(ledger_bin());
    cmd.args(["list", "--status", "running"]);
    let out = cmd.output().map_err(|e| anyhow!("spawn kei-ledger: {e}"))?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        return Err(anyhow!("kei-ledger list failed: {stderr}"));
    }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

fn run(cmd: &mut Command, stage: &str) -> Result<()> {
    let out = cmd
        .output()
        .map_err(|e| anyhow!("spawn kei-ledger {stage}: {e}"))?;
    if out.status.success() {
        return Ok(());
    }
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    Err(anyhow!("kei-ledger {stage} failed: {stderr}"))
}

#[cfg(test)]
mod tests {
    //! Regression coverage for [`ledger_bin`] lookup precedence.
    //!
    //! Env vars are process-global; serialize with a local mutex so
    //! parallel cargo-test workers don't trample each other.
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn with_env<F: FnOnce()>(kei_bin: Option<&str>, cargo_bin: Option<&str>, f: F) {
        let _g = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let prev_kei = std::env::var("KEI_LEDGER_BIN").ok();
        let prev_cargo = std::env::var("CARGO_BIN_EXE_kei-ledger").ok();
        match kei_bin {
            Some(v) => std::env::set_var("KEI_LEDGER_BIN", v),
            None => std::env::remove_var("KEI_LEDGER_BIN"),
        }
        match cargo_bin {
            Some(v) => std::env::set_var("CARGO_BIN_EXE_kei-ledger", v),
            None => std::env::remove_var("CARGO_BIN_EXE_kei-ledger"),
        }
        f();
        match prev_kei {
            Some(v) => std::env::set_var("KEI_LEDGER_BIN", v),
            None => std::env::remove_var("KEI_LEDGER_BIN"),
        }
        match prev_cargo {
            Some(v) => std::env::set_var("CARGO_BIN_EXE_kei-ledger", v),
            None => std::env::remove_var("CARGO_BIN_EXE_kei-ledger"),
        }
    }

    #[test]
    fn ledger_bin_env_overrides_default() {
        with_env(Some("/opt/pinned/kei-ledger"), None, || {
            assert_eq!(ledger_bin(), "/opt/pinned/kei-ledger");
        });
    }

    #[test]
    fn ledger_bin_cargo_env_used_when_kei_unset() {
        with_env(None, Some("/tmp/cargo-built/kei-ledger"), || {
            assert_eq!(ledger_bin(), "/tmp/cargo-built/kei-ledger");
        });
    }

    #[test]
    fn ledger_bin_falls_back_to_bare_name() {
        with_env(None, None, || {
            assert_eq!(ledger_bin(), "kei-ledger");
        });
    }

    #[test]
    fn ledger_bin_env_wins_over_cargo_env() {
        with_env(
            Some("/opt/pinned/kei-ledger"),
            Some("/tmp/cargo-built/kei-ledger"),
            || {
                assert_eq!(ledger_bin(), "/opt/pinned/kei-ledger");
            },
        );
    }
}
