// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! [`Runner`] — minimal sync abstraction over a process invocation. The
//! `OpenvpnMode` impl holds a `Arc<dyn Runner + Send + Sync>` so tests
//! can substitute a mock recorder without touching `systemctl`. Real
//! deployments use [`SystemRunner`] which shells out via
//! `std::process::Command`.
//!
//! The trait is intentionally synchronous — `systemctl start/stop` is
//! a sub-second blocking call and we do NOT want to drag a Tokio
//! runtime through the runner abstraction. The async `NetworkMode`
//! method wraps the call in `tokio::task::spawn_blocking` if the
//! caller is on a multi-thread runtime; for the smoke tests we call
//! it directly inline.

use crate::error::{Error, Result};
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunOutput {
    pub status: i32,
    pub stdout: String,
    pub stderr: String,
}

impl RunOutput {
    pub fn ok(&self) -> bool {
        self.status == 0
    }
}

pub trait Runner: Send + Sync {
    /// Invoke `program` with `args`. Returns the captured outcome.
    /// Implementations MUST capture stdout + stderr and the integer
    /// exit code; they MUST NOT panic on non-zero exit.
    fn run(&self, program: &str, args: &[&str]) -> Result<RunOutput>;
}

/// Real backend: `std::process::Command` shell-out. Used in production.
#[derive(Debug, Default)]
pub struct SystemRunner;

impl SystemRunner {
    pub fn new() -> Self {
        SystemRunner
    }
}

impl Runner for SystemRunner {
    fn run(&self, program: &str, args: &[&str]) -> Result<RunOutput> {
        let out = Command::new(program).args(args).output().map_err(Error::Io)?;
        let status = out.status.code().unwrap_or(-1);
        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        Ok(RunOutput {
            status,
            stdout,
            stderr,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_output_ok_zero_exit() {
        let o = RunOutput {
            status: 0,
            stdout: String::new(),
            stderr: String::new(),
        };
        assert!(o.ok());
    }

    #[test]
    fn run_output_not_ok_nonzero_exit() {
        let o = RunOutput {
            status: 1,
            stdout: String::new(),
            stderr: "boom".into(),
        };
        assert!(!o.ok());
    }
}
