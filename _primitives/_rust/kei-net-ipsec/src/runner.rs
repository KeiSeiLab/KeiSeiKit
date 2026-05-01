// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! Runner trait — the seam every `swanctl` invocation goes through.
//!
//! Constructor Pattern: ALL subprocess invocation lives here. The
//! [`crate::network::IpsecMode`] cube accepts an `Arc<dyn Runner + Send +
//! Sync>` so unit tests substitute [`MockRunner`] without spawning real
//! `swanctl` and without root privileges.
//!
//! Mirrors the W59 `kei-llm-mlx::runner` pattern (sync trait, sanitized
//! fixture stems, in-memory override map). The trait stays sync because
//! every `swanctl` shell-out is whole-output capture (no streaming).

use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use std::process::Command;

/// Captured one-shot subprocess result. `code = None` means the child was
/// killed by signal (rare in tests; [`SystemRunner`] only fills it via
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

/// Single seam. Implementors: [`SystemRunner`] (real host) or
/// [`MockRunner`] (override-map backed).
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

/// Sanitize `(cmd, args)` into a fixture-stem key. Bytes outside
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

/// In-memory fixture-backed runner for tests. Overrides keyed by
/// [`fixture_stem`].
pub struct MockRunner {
    pub overrides: HashMap<String, std::result::Result<RunOutput, String>>,
    pub calls: std::sync::Mutex<Vec<(String, Vec<String>)>>,
}

impl MockRunner {
    pub fn new() -> Self {
        Self { overrides: HashMap::new(), calls: std::sync::Mutex::new(Vec::new()) }
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

    /// Snapshot of `(cmd, args)` invocations recorded so far.
    pub fn recorded(&self) -> Vec<(String, Vec<String>)> {
        self.calls.lock().expect("mock calls lock").clone()
    }
}

impl Default for MockRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl Runner for MockRunner {
    fn run(&self, cmd: &str, args: &[&str]) -> Result<RunOutput> {
        {
            let mut g = self.calls.lock().expect("mock calls lock");
            g.push((cmd.to_string(), args.iter().map(|s| s.to_string()).collect()));
        }
        let stem = fixture_stem(cmd, args);
        if let Some(slot) = self.overrides.get(&stem) {
            return match slot {
                Ok(r) => Ok(r.clone()),
                Err(e) => Err(anyhow!("{e}")),
            };
        }
        Err(anyhow!("mock fixture missing for stem `{stem}`"))
    }
}
