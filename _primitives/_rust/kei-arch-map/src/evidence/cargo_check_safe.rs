//! Wave 4 #50C/#53 — whitelisted `cargo check` evidence kind.
//!
//! `CargoCheckSafe` runs `cargo check --workspace --offline --message-format=json`
//! ONLY on workspaces whose `manifest_dir` (relative to repo root) appears in
//! `allowed_paths`. External / untrusted manifests are refused with FAIL.
//!
//! ⚠ build.rs RCE: `cargo check` compiles + runs build scripts. The allowlist
//! exists so a Plan author can declare "I trust this workspace" explicitly.
//! For untrusted manifests use `CargoCheckClean` (manifest-validate only).

use super::cargo_check::resolve_cargo;
use super::path_resolve;
use serde::Deserialize;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use wait_timeout::ChildExt;

const TIMEOUT: Duration = Duration::from_secs(300);

#[derive(Debug, Deserialize)]
struct Diag {
    reason: String,
    #[serde(default)]
    message: Option<DiagMessage>,
}

#[derive(Debug, Deserialize)]
struct DiagMessage {
    #[serde(default)]
    level: String,
}

struct DrainedOutput {
    status: std::process::ExitStatus,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

/// Verify `manifest_dir` (relative to root, after canonicalize) appears in
/// `allowed_paths`. Each allowlist entry is resolved against `root` and
/// canonicalized so symlink trickery cannot smuggle an external path past
/// the gate.
fn verify_allowlist(
    manifest_canon: &Path,
    allowed: &[PathBuf],
    root: &Path,
) -> Result<(), String> {
    if allowed.is_empty() {
        return Err(
            "cargo_check_safe refused: allowed_paths is empty (explicit whitelist required)"
                .to_string(),
        );
    }
    for entry in allowed {
        let resolved = path_resolve::resolve(entry, root);
        if let Ok(canon) = resolved.canonicalize() {
            if manifest_canon == canon {
                return Ok(());
            }
        }
    }
    Err(format!(
        "cargo_check_safe refused: {} not in allowlist ({} entries)",
        manifest_canon.display(),
        allowed.len()
    ))
}

fn spawn(dir: &Path) -> Result<std::process::Child, String> {
    let cargo = resolve_cargo()?;
    Command::new(&cargo)
        .args([
            "check",
            "--workspace",
            "--offline",
            "--message-format=json",
        ])
        .current_dir(dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("spawn cargo: {}", e))
}

fn drain<R: Read + Send + 'static>(mut r: R) -> thread::JoinHandle<Vec<u8>> {
    thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = r.read_to_end(&mut buf);
        buf
    })
}

fn wait_capped(mut child: std::process::Child) -> Result<DrainedOutput, String> {
    let stdout = child.stdout.take().expect("stdout piped");
    let stderr = child.stderr.take().expect("stderr piped");
    let stdout_h = drain(stdout);
    let stderr_h = drain(stderr);
    match child.wait_timeout(TIMEOUT) {
        Ok(Some(status)) => {
            let out = stdout_h.join().unwrap_or_default();
            let err = stderr_h.join().unwrap_or_default();
            Ok(DrainedOutput { status, stdout: out, stderr: err })
        }
        Ok(None) => {
            let _ = child.kill();
            let _ = child.wait();
            let _ = stdout_h.join();
            let _ = stderr_h.join();
            Err(format!("cargo check exceeded {}s", TIMEOUT.as_secs()))
        }
        Err(e) => Err(format!("wait_timeout: {}", e)),
    }
}

fn count_errors(stdout: &[u8]) -> u64 {
    let s = String::from_utf8_lossy(stdout);
    let mut errs = 0u64;
    for line in s.lines() {
        if line.is_empty() {
            continue;
        }
        if let Ok(d) = serde_json::from_str::<Diag>(line) {
            if d.reason == "compiler-message" {
                if let Some(m) = d.message {
                    if m.level == "error" {
                        errs += 1;
                    }
                }
            }
        }
    }
    errs
}

fn run_and_classify(child: std::process::Child) -> (bool, String) {
    let out = match wait_capped(child) {
        Ok(o) => o,
        Err(e) => return (false, e),
    };
    let errs = count_errors(&out.stdout);
    if errs == 0 && out.status.success() {
        return (true, String::new());
    }
    if errs == 0 && !out.status.success() {
        let tail: String = String::from_utf8_lossy(&out.stderr).chars().take(200).collect();
        return (false, format!("cargo non-zero exit; stderr: {}", tail));
    }
    (false, format!("cargo check: {} compiler-error(s)", errs))
}

/// Entry point. Runs `cargo check --workspace` on `manifest_dir` ONLY if
/// `manifest_dir` (after canonicalize) matches one of `allowed_paths`.
pub fn check(
    manifest_dir: &Path,
    allowed_paths: &[PathBuf],
    root: &Path,
) -> (bool, String) {
    let manifest_canon = match path_resolve::resolve_confined(manifest_dir, root) {
        Ok(p) => p,
        Err(e) => return (false, e),
    };
    if !manifest_canon.join("Cargo.toml").exists() {
        return (false, format!("no Cargo.toml at {}", manifest_canon.display()));
    }
    if let Err(e) = verify_allowlist(&manifest_canon, allowed_paths, root) {
        return (false, e);
    }
    let child = match spawn(&manifest_canon) {
        Ok(c) => c,
        Err(e) => return (false, e),
    };
    run_and_classify(child)
}
