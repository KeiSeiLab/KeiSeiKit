// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Runner trait — the seam every `wg`/`wg-quick` shell-out goes through.
//!
//! Constructor Pattern: ALL subprocess invocation lives here. `network.rs`
//! takes `Arc<dyn Runner + Send + Sync>` so unit/smoke tests substitute a
//! fixture-backed mock without touching the host system. Mirrors the
//! `kei-llm-mlx::runner` pattern (sync trait; the async glue lives at the
//! call site via `tokio::task::spawn_blocking`).

use anyhow::{anyhow, Context, Result};
use std::process::Command;

/// Captured one-shot subprocess result. `code = None` means the child was
/// killed by signal.
#[derive(Debug, Clone)]
pub struct RunOutput {
    pub stdout: String,
    pub stderr: String,
    pub code: Option<i32>,
}

impl RunOutput {
    pub fn ok(stdout: impl Into<String>) -> Self {
        Self {
            stdout: stdout.into(),
            stderr: String::new(),
            code: Some(0),
        }
    }

    pub fn fail(code: i32, stderr: impl Into<String>) -> Self {
        Self {
            stdout: String::new(),
            stderr: stderr.into(),
            code: Some(code),
        }
    }

    pub fn is_success(&self) -> bool {
        self.code == Some(0)
    }
}

/// The seam. Implementors: [`SystemRunner`] (real host) or test mocks.
pub trait Runner {
    fn run(&self, cmd: &str, args: &[&str]) -> Result<RunOutput>;
}

/// Real-host runner — the only production user of `Command::new` in the
/// crate.
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

/// Helper for callers that want a "must succeed or anyhow!" wrapper. Kept
/// public so `network.rs` can re-use it (and so smoke tests can assert on
/// failure-path messaging).
pub fn check_success(cmd: &str, args: &[&str], out: &RunOutput) -> Result<()> {
    if out.is_success() {
        return Ok(());
    }
    Err(anyhow!(
        "{cmd} {args:?} exited code={:?} stderr={}",
        out.code,
        out.stderr.trim()
    ))
}
