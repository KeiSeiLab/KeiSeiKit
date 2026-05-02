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

/// Validate SSH username: alphanumeric + `-_.`, not starting with `-`, max 64 chars.
pub fn is_safe_user(user: &str) -> bool {
    if user.is_empty() || user.len() > 64 || user.starts_with('-') {
        return false;
    }
    user.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.')
}

/// Validate SSH hostname: alphanumeric + `-_.`, not starting with `-`, max 64 chars.
/// IPv4 dot-notation is covered by the alphanumeric+dot rule.
pub fn is_safe_host(host: &str) -> bool {
    if host.is_empty() || host.len() > 64 || host.starts_with('-') {
        return false;
    }
    host.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.')
}

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
        let strict = if std::env::var("KEI_BAREMETAL_ACCEPT_NEW").as_deref() == Ok("1") {
            "accept-new"
        } else {
            "yes"
        };
        let mut cmd = Command::new(&self.binary);
        cmd.arg("-o")
            .arg("BatchMode=yes")
            .arg("-o")
            .arg("ConnectTimeout=5")
            .arg("-o")
            .arg(format!("StrictHostKeyChecking={strict}"));
        if let Some(p) = self.port {
            cmd.arg("-p").arg(p.to_string());
        }
        if let Some(k) = &self.key_path {
            cmd.arg("-i").arg(k);
        }
        // `--` stops flag parsing; guards against user/host that look like flags (CVE-2023-51385).
        cmd.arg("--").arg(format!("{}@{}", self.user, self.host));
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
        let out = run_remote(&t, "hello-from-remote").await.expect("echo exit 0");
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

    #[test]
    fn safe_user_accepts_valid() {
        assert!(is_safe_user("root"));
        assert!(is_safe_user("alice-b"));
        assert!(is_safe_user("user_123"));
    }

    #[test]
    fn safe_user_rejects_injection() {
        assert!(!is_safe_user("-ProxyCommand=evil"));
        assert!(!is_safe_user("a@b"));
        assert!(!is_safe_user("a/b"));
        assert!(!is_safe_user(""));
        assert!(!is_safe_user(&"a".repeat(65)));
    }

    #[test]
    fn safe_host_accepts_valid() {
        assert!(is_safe_host("box.example.com"));
        assert!(is_safe_host("10.0.0.1"));
        assert!(is_safe_host("my-server"));
    }

    #[test]
    fn safe_host_rejects_injection() {
        assert!(!is_safe_host("-evil"));
        assert!(!is_safe_host("host name"));
        assert!(!is_safe_host("host:22"));
        assert!(!is_safe_host(""));
    }
}
