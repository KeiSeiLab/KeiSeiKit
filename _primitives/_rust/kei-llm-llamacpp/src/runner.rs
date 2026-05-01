//! Runner trait — the ONLY surface that owns subprocess lifecycle.
//!
//! All `tokio::process::Command` invocations flow through this trait.
//! Tests inject `MockRunner` which returns canned `RunOutput` from a
//! fixture queue; production uses `RealRunner` which spawns the binary.
//!
//! Mirrors the Wave 56 kei-machine-probe pattern. Uses Rust 1.75+
//! native `async fn in trait` (no `async-trait` dep) — workspace
//! `rust-version = "1.75"` permits this.

use crate::error::{Error, Result};
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};

/// Result of a one-shot `<bin> <args>` invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunOutput {
    pub stdout: String,
    pub stderr: String,
    pub code: i32,
}

/// Handle to a spawned `llama-server` (or any long-lived child).
/// Drop sends SIGKILL. The child id is recorded for diagnostics.
#[derive(Debug)]
pub struct ServerHandle {
    pub pid: u32,
    pub port: u16,
    child: Option<Child>,
    /// Mock-mode kill flag — flipped on Drop when no real child held.
    /// Tests assert this side-effect via the flag handle.
    kill_flag: Option<Arc<Mutex<bool>>>,
}

impl ServerHandle {
    /// Construct a real handle backed by a tokio Child.
    pub fn from_child(child: Child, port: u16) -> Self {
        let pid = child.id().unwrap_or(0);
        Self { pid, port, child: Some(child), kill_flag: None }
    }

    /// Mock-mode constructor: no child held; Drop flips `flag` to true.
    pub fn mock(pid: u32, port: u16, flag: Arc<Mutex<bool>>) -> Self {
        Self { pid, port, child: None, kill_flag: Some(flag) }
    }
}

impl Drop for ServerHandle {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.start_kill();
        }
        if let Some(flag) = &self.kill_flag {
            if let Ok(mut guard) = flag.lock() {
                *guard = true;
            }
        }
    }
}

/// Pinned-future return type to keep the Runner trait object-safe.
type BoxFut<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// All process spawns must implement this trait.
pub trait Runner: Send + Sync {
    /// Run `<bin> <args>` to completion, capturing stdout+stderr.
    fn run<'a>(&'a self, bin: &'a str, args: &'a [String]) -> BoxFut<'a, Result<RunOutput>>;

    /// Spawn `<bin> <args>` and collect stdout line-by-line.
    fn run_stream<'a>(
        &'a self,
        bin: &'a str,
        args: &'a [String],
    ) -> BoxFut<'a, Result<Vec<String>>>;

    /// Spawn `<bin> <args>` as a long-lived child; return a handle.
    fn spawn_server<'a>(
        &'a self,
        bin: &'a str,
        args: &'a [String],
        port: u16,
    ) -> BoxFut<'a, Result<ServerHandle>>;
}

/// Production runner — real `tokio::process::Command` invocations.
pub struct RealRunner;

impl Runner for RealRunner {
    fn run<'a>(&'a self, bin: &'a str, args: &'a [String]) -> BoxFut<'a, Result<RunOutput>> {
        Box::pin(real_run(bin, args))
    }

    fn run_stream<'a>(
        &'a self,
        bin: &'a str,
        args: &'a [String],
    ) -> BoxFut<'a, Result<Vec<String>>> {
        Box::pin(real_run_stream(bin, args))
    }

    fn spawn_server<'a>(
        &'a self,
        bin: &'a str,
        args: &'a [String],
        port: u16,
    ) -> BoxFut<'a, Result<ServerHandle>> {
        Box::pin(real_spawn_server(bin, args, port))
    }
}

async fn real_run(bin: &str, args: &[String]) -> Result<RunOutput> {
    let out = Command::new(bin)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;
    Ok(RunOutput {
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        code: out.status.code().unwrap_or(-1),
    })
}

async fn real_run_stream(bin: &str, args: &[String]) -> Result<Vec<String>> {
    let mut child = Command::new(bin)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| Error::ParseFailed { reason: "no stdout pipe".into() })?;
    let mut lines = BufReader::new(stdout).lines();
    let mut out = Vec::new();
    while let Some(line) = lines.next_line().await? {
        out.push(line);
    }
    let _ = child.wait().await?;
    Ok(out)
}

async fn real_spawn_server(bin: &str, args: &[String], port: u16) -> Result<ServerHandle> {
    let child = Command::new(bin)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    Ok(ServerHandle::from_child(child, port))
}

/// Path-or-name resolver — used by `which(1)`-style discovery in
/// `discovery.rs`. Lives here because it is a process-helper.
pub fn bin_in_path(name: &str) -> Option<PathBuf> {
    let path_var = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path_var) {
        let candidate = dir.join(name);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}
