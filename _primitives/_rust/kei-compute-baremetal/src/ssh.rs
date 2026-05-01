// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Denis Parfionovich
//
//! Minimal async SSH wrapper. Shells out to the system `ssh` binary —
//! no Rust SSH client linked in. Two operations only: `ping` (returns
//! Ok if remote shell exits 0) and `run_remote` (returns stdout).
//!
//! `binary` field exists so unit tests can swap `ssh` for `echo` without
//! reaching the network. Production callers always use the default `ssh`.

use crate::error::{Error, Result};
use tokio::process::Command;

/// SSH endpoint. `port` is optional; default is 22 when None.
#[derive(Debug, Clone)]
pub struct SshTarget {
    pub user: String,
    pub host: String,
    pub port: Option<u16>,
    pub key_path: Option<String>,
    /// Override the binary name. Production: `"ssh"`. Tests: `"echo"`.
    pub binary: String,
}

impl SshTarget {
    pub fn new(user: impl Into<String>, host: impl Into<String>) -> Self {
        Self {
            user: user.into(),
            host: host.into(),
            port: None,
            key_path: None,
            binary: "ssh".into(),
        }
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn with_key(mut self, path: impl Into<String>) -> Self {
        self.key_path = Some(path.into());
        self
    }

    pub fn external_id(&self) -> String {
        match self.port {
            Some(p) => format!("ssh://{}@{}:{}", self.user, self.host, p),
            None => format!("ssh://{}@{}", self.user, self.host),
        }
    }

    fn build_cmd(&self, remote: &str) -> Command {
        let mut cmd = Command::new(&self.binary);
        cmd.arg("-o")
            .arg("BatchMode=yes")
            .arg("-o")
            .arg("ConnectTimeout=5")
            .arg("-o")
            .arg("StrictHostKeyChecking=accept-new");
        if let Some(p) = self.port {
            cmd.arg("-p").arg(p.to_string());
        }
        if let Some(k) = &self.key_path {
            cmd.arg("-i").arg(k);
        }
        cmd.arg(format!("{}@{}", self.user, self.host));
        cmd.arg(remote);
        cmd
    }
}

/// Probe reachability — runs `echo ok` over SSH. Ok iff exit 0.
pub async fn ping(t: &SshTarget) -> Result<()> {
    let status = t
        .build_cmd("echo ok")
        .status()
        .await
        .map_err(|e| Error::ConnectionFailed {
            host: t.host.clone(),
            detail: e.to_string(),
        })?;
    if !status.success() {
        return Err(Error::ConnectionFailed {
            host: t.host.clone(),
            detail: format!("exit {:?}", status.code()),
        });
    }
    Ok(())
}

/// Run an arbitrary shell snippet on the remote box. Returns combined
/// stdout (UTF-8 lossy) on success.
pub async fn run_remote(t: &SshTarget, cmd: &str) -> Result<String> {
    let out = t
        .build_cmd(cmd)
        .output()
        .await
        .map_err(|e| Error::ConnectionFailed {
            host: t.host.clone(),
            detail: e.to_string(),
        })?;
    if !out.status.success() {
        return Err(Error::ConnectionFailed {
            host: t.host.clone(),
            detail: format!(
                "exit {:?} stderr={}",
                out.status.code(),
                String::from_utf8_lossy(&out.stderr)
            ),
        });
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Substitutes `echo` for `ssh` so the test never opens a socket.
    /// `echo` ignores all SSH-flag args and prints the trailing argument
    /// (the remote command string), so success exit code is what we check.
    #[tokio::test]
    async fn ping_succeeds_with_echo_binary() {
        let mut t = SshTarget::new("root", "127.0.0.1");
        t.binary = "echo".into();
        ping(&t).await.expect("echo always exits 0");
    }

    #[tokio::test]
    async fn run_remote_returns_stdout_with_echo_binary() {
        let mut t = SshTarget::new("root", "127.0.0.1");
        t.binary = "echo".into();
        let out = run_remote(&t, "hello-from-remote")
            .await
            .expect("echo exit 0");
        // `echo` reflects all its argv joined by spaces; the trailing
        // remote-cmd is the last token, so it MUST appear in output.
        assert!(out.contains("hello-from-remote"), "stdout was: {out:?}");
    }

    #[test]
    fn external_id_includes_port_when_set() {
        let t = SshTarget::new("alice", "box.example.com").with_port(2222);
        assert_eq!(t.external_id(), "ssh://alice@box.example.com:2222");
    }

    #[test]
    fn external_id_omits_port_when_default() {
        let t = SshTarget::new("alice", "box.example.com");
        assert_eq!(t.external_id(), "ssh://alice@box.example.com");
    }
}
