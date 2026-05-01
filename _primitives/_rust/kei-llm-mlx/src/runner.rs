//! Runner trait — the seam every shell-out goes through.
//!
//! Constructor Pattern: ALL subprocess invocation lives here. Every other
//! cube (`generate`, `stream`, `server`, `discovery`) accepts a `&dyn Runner`
//! so unit tests substitute `MockRunner` without touching the host system
//! and without invoking real `mlx_lm`.
//!
//! Mirrors the W56 `kei-machine-probe` pattern (sync trait, sanitized
//! fixture stems). Tokio is held as a workspace dep for future streaming
//! transport but the trait surface stays sync — every mlx_lm shell-out is
//! whole-output capture, not interactive PTY.

use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Captured one-shot subprocess result. `code = None` means the child was
/// killed by signal (rare in tests; SystemRunner only fills it via
/// `.code()`).
#[derive(Debug, Clone)]
pub struct RunOutput {
    pub stdout: String,
    pub stderr: String,
    pub code: Option<i32>,
}

impl RunOutput {
    pub fn ok(stdout: impl Into<String>) -> Self {
        Self { stdout: stdout.into(), stderr: String::new(), code: Some(0) }
    }
    pub fn fail(code: i32, stderr: impl Into<String>) -> Self {
        Self { stdout: String::new(), stderr: stderr.into(), code: Some(code) }
    }
    pub fn is_success(&self) -> bool {
        self.code == Some(0)
    }
}

/// Single seam. Implementors: `SystemRunner` (real host) or `MockRunner`
/// (fixture-backed).
pub trait Runner {
    fn run(&self, cmd: &str, args: &[&str]) -> Result<RunOutput>;
}

/// Real-host runner. ONLY production user of `std::process::Command::new`
/// inside this crate.
pub struct SystemRunner;

impl Runner for SystemRunner {
    fn run(&self, cmd: &str, args: &[&str]) -> Result<RunOutput> {
        let out = Command::new(cmd)
            .args(args)
            .output()
            .with_context(|| format!("spawn {cmd}"))?;
        Ok(RunOutput {
            stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
            code: out.status.code(),
        })
    }
}

/// Sanitize `(cmd, args)` into a fixture filename stem. Bytes outside
/// `[A-Za-z0-9._-]` collapse to `_`. Keeps stems short and shell-agnostic.
pub fn fixture_stem(cmd: &str, args: &[&str]) -> String {
    let mut joined = cmd.to_string();
    for a in args {
        joined.push(' ');
        joined.push_str(a);
    }
    joined
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

/// In-memory fixture-backed runner for tests. Two layers:
/// 1. `overrides` — programmatic per-stem `RunOutput` map (preferred for
///    unit tests).
/// 2. `dir` — optional fallback directory holding `<stem>.stdout` files
///    (kept for parity with W56; rarely needed here).
pub struct MockRunner {
    pub dir: Option<PathBuf>,
    pub overrides: HashMap<String, Result<RunOutput, String>>,
}

impl MockRunner {
    pub fn new() -> Self {
        Self { dir: None, overrides: HashMap::new() }
    }

    pub fn from_dir(dir: impl AsRef<Path>) -> Self {
        Self { dir: Some(dir.as_ref().to_path_buf()), overrides: HashMap::new() }
    }

    pub fn with_ok(mut self, stem: &str, body: &str) -> Self {
        self.overrides.insert(stem.to_string(), Ok(RunOutput::ok(body)));
        self
    }

    pub fn with_run(mut self, stem: &str, run: RunOutput) -> Self {
        self.overrides.insert(stem.to_string(), Ok(run));
        self
    }

    pub fn with_err(mut self, stem: &str, msg: &str) -> Self {
        self.overrides.insert(stem.to_string(), Err(msg.to_string()));
        self
    }
}

impl Default for MockRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl Runner for MockRunner {
    fn run(&self, cmd: &str, args: &[&str]) -> Result<RunOutput> {
        let stem = fixture_stem(cmd, args);
        if let Some(slot) = self.overrides.get(&stem) {
            return match slot {
                Ok(r) => Ok(r.clone()),
                Err(e) => Err(anyhow!("{e}")),
            };
        }
        if let Some(dir) = &self.dir {
            let path = dir.join(format!("{stem}.stdout"));
            if path.exists() {
                let body = std::fs::read_to_string(&path)
                    .with_context(|| format!("read fixture {}", path.display()))?;
                return Ok(RunOutput::ok(body));
            }
        }
        Err(anyhow!("mock fixture missing for stem `{stem}`"))
    }
}
