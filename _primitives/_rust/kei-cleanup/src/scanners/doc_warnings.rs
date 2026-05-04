//! Doc-warnings scanner — see CLEANUP-RUNTIME-SPEC.md §3.2 + Appendix A.
//!
//! Shells out to `cargo doc --no-deps --workspace -- -D warnings`, then
//! parses stderr for `warning:` lines; each warning that carries a
//! `--> <file>:<line>:<col>` follow-up becomes one Finding.
//!
//! Pipe-drain pattern from kei-arch-map cargo_check.rs (Phase 1.6) so a
//! large workspace doesn't deadlock the child on a 64 KiB pipe buffer.
//! Wall-clock cap 120 s via `wait-timeout`.

use crate::config::Config;
use crate::error::CleanupError;
use crate::report::{Confidence, Finding, FindingKind, Location, Severity};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use wait_timeout::ChildExt;

const TIMEOUT: Duration = Duration::from_secs(120);

/// Public scanner entry — see Appendix A row "doc_warnings".
pub fn scan(workspace: &Path, _cfg: &Config) -> Result<Vec<Finding>, CleanupError> {
    if !has_cargo() {
        return Err(CleanupError::ToolNotFound("cargo not on PATH".into()));
    }
    let stderr_text = run_cargo_doc(workspace)?;
    Ok(parse_warnings(&stderr_text, workspace))
}

fn has_cargo() -> bool {
    Command::new("cargo")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn run_cargo_doc(workspace: &Path) -> Result<String, CleanupError> {
    let mut child = Command::new("cargo")
        .args(["doc", "--no-deps", "--workspace", "--", "-D", "warnings"])
        .current_dir(workspace)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let stdout = child.stdout.take().expect("stdout piped");
    let stderr = child.stderr.take().expect("stderr piped");
    let stdout_h = drain(stdout);
    let stderr_h = drain(stderr);
    wait_and_collect(child, stdout_h, stderr_h)
}

fn wait_and_collect(
    mut child: std::process::Child,
    stdout_h: thread::JoinHandle<Vec<u8>>,
    stderr_h: thread::JoinHandle<Vec<u8>>,
) -> Result<String, CleanupError> {
    match child.wait_timeout(TIMEOUT) {
        Ok(Some(_status)) => {
            let _ = stdout_h.join();
            let err = stderr_h.join().unwrap_or_default();
            Ok(String::from_utf8_lossy(&err).to_string())
        }
        Ok(None) => {
            let _ = child.kill();
            let _ = child.wait();
            let _ = stdout_h.join();
            let _ = stderr_h.join();
            Err(CleanupError::ToolFailure {
                tool: "cargo doc".into(),
                detail: format!("timeout after {}s", TIMEOUT.as_secs()),
            })
        }
        Err(e) => Err(CleanupError::ToolFailure {
            tool: "cargo doc".into(),
            detail: format!("wait_timeout: {e}"),
        }),
    }
}

fn drain<R: Read + Send + 'static>(mut r: R) -> thread::JoinHandle<Vec<u8>> {
    thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = r.read_to_end(&mut buf);
        buf
    })
}

/// Parse cargo stderr for `warning:` and following `--> path:line:col`.
fn parse_warnings(stderr: &str, workspace: &Path) -> Vec<Finding> {
    let mut out = Vec::new();
    let mut last_warn: Option<String> = None;
    for line in stderr.lines() {
        let trim = line.trim_start();
        if let Some(rest) = trim.strip_prefix("warning: ") {
            last_warn = Some(rest.to_string());
            continue;
        }
        if let Some(rest) = trim.strip_prefix("--> ") {
            if let Some(msg) = last_warn.take() {
                if let Some((path, lineno)) = parse_location(rest) {
                    out.push(make_finding(workspace, &path, lineno, &msg));
                }
            }
        }
    }
    out
}

fn parse_location(s: &str) -> Option<(PathBuf, u32)> {
    let mut parts = s.split(':');
    let path = parts.next()?.trim();
    let line: u32 = parts.next()?.trim().parse().ok()?;
    Some((PathBuf::from(path), line))
}

fn make_finding(workspace: &Path, path: &Path, line: u32, msg: &str) -> Finding {
    let rel = path
        .strip_prefix(workspace)
        .map(Path::to_path_buf)
        .unwrap_or_else(|_| path.to_path_buf());
    let severity = severity_from_msg(msg);
    Finding {
        kind: FindingKind::DocWarning,
        severity,
        location: Location { file: rel, line: Some(line) },
        message: format!("cargo doc: {msg}"),
        fix_hint: Some("repair intra-doc link or qualify the path".into()),
        rule_ref: Some("CLEANUP §3.2 doc_warnings".into()),
        confidence: Confidence::High,
    }
}

fn severity_from_msg(msg: &str) -> Severity {
    let lower = msg.to_ascii_lowercase();
    if lower.contains("broken") || lower.contains("unresolved") {
        Severity::Medium
    } else {
        Severity::Low
    }
}
