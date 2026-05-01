// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! `OpenvpnMode` — `NetworkMode` impl over a host-resident OpenVPN
//! server managed by `systemd`.
//!
//! Constructor surface:
//!   * [`OpenvpnMode::with_runner`] — explicit runner, service name,
//!     and management-socket path. Used by smoke tests with a mock
//!     runner.
//!   * [`OpenvpnMode::from_env`]    — reads `OPENVPN_SERVICE_NAME`
//!     (default `server`) and `OPENVPN_CONFIG_PATH` (default
//!     `/etc/openvpn/server/<name>.conf`); the management-socket path
//!     is derived as `/var/run/openvpn/<name>.sock` unless an explicit
//!     `OPENVPN_MGMT_SOCKET` is set. Uses [`SystemRunner`].
//!
//! NetworkMode wire:
//!   * `configure(_)` → `systemctl start openvpn-server@<name>`
//!   * `teardown()`   → `systemctl stop  openvpn-server@<name>`
//!   * `peers()`      → if `mgmt_socket` is `Some(path)`, connect via
//!                       `tokio::net::UnixStream`, send `status 2\r\n`,
//!                       read until `\nEND\n` (or EOF), then
//!                       `parse_status_output`. If `None`, return
//!                       `Ok(vec![])`.
//!   * `is_public()`  → `true` (OpenVPN exposes a routable UDP/TCP
//!                       endpoint by default).

use crate::error::{Error, Result};
use crate::mgmt::parse_status_output;
use crate::runner::{Runner, SystemRunner};
use kei_runtime_core::traits::network::{NetworkConfig, NetworkMode, PeerStatus};
use kei_runtime_core::{Dna, DnaBuilder, HasDna};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

const DEFAULT_SERVICE_NAME: &str = "server";
const DEFAULT_CONFIG_DIR: &str = "/etc/openvpn/server";
const DEFAULT_MGMT_DIR: &str = "/var/run/openvpn";

pub struct OpenvpnMode {
    dna: Dna,
    parent: Option<Dna>,
    runner: Arc<dyn Runner + Send + Sync>,
    service_name: String,
    config_path: PathBuf,
    mgmt_socket: Option<PathBuf>,
}

impl OpenvpnMode {
    /// Build with explicit runner + service name + paths. `parent` is
    /// the DNA of the entity that spawned this mode (e.g. the
    /// orchestrator); pass `None` for a root invocation.
    pub fn with_runner(
        runner: Arc<dyn Runner + Send + Sync>,
        service_name: impl Into<String>,
        config_path: impl Into<PathBuf>,
        mgmt_socket: Option<PathBuf>,
        parent: Option<Dna>,
    ) -> Result<Self> {
        let dna = DnaBuilder::new("primitive")
            .caps(["PR", "AP", "OV"])
            .scope("keiseikit.dev/primitives/kei-net-openvpn")
            .body(b"openvpn-systemd-v1")
            .build()?;
        Ok(Self {
            dna,
            parent,
            runner,
            service_name: service_name.into(),
            config_path: config_path.into(),
            mgmt_socket,
        })
    }

    /// Build from env. Required: none (all have defaults).
    /// Recognised env:
    ///   * `OPENVPN_SERVICE_NAME` — default `server`
    ///   * `OPENVPN_CONFIG_PATH`  — default `/etc/openvpn/server/<name>.conf`
    ///   * `OPENVPN_MGMT_SOCKET`  — default `/var/run/openvpn/<name>.sock`
    pub fn from_env(parent: Option<Dna>) -> Result<Self> {
        let name = std::env::var("OPENVPN_SERVICE_NAME")
            .unwrap_or_else(|_| DEFAULT_SERVICE_NAME.to_string());
        let config_path: PathBuf = std::env::var("OPENVPN_CONFIG_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                PathBuf::from(format!("{DEFAULT_CONFIG_DIR}/{name}.conf"))
            });
        let mgmt_socket: Option<PathBuf> = std::env::var("OPENVPN_MGMT_SOCKET")
            .ok()
            .map(PathBuf::from)
            .or_else(|| Some(PathBuf::from(format!("{DEFAULT_MGMT_DIR}/{name}.sock"))));
        Self::with_runner(
            Arc::new(SystemRunner::new()),
            name,
            config_path,
            mgmt_socket,
            parent,
        )
    }

    pub fn service_name(&self) -> &str {
        &self.service_name
    }
    pub fn config_path(&self) -> &std::path::Path {
        &self.config_path
    }
    pub fn mgmt_socket(&self) -> Option<&std::path::Path> {
        self.mgmt_socket.as_deref()
    }

    fn unit_name(&self) -> String {
        format!("openvpn-server@{}", self.service_name)
    }

    fn invoke_systemctl(&self, verb: &str) -> Result<()> {
        let unit = self.unit_name();
        let out = self.runner.run("systemctl", &[verb, &unit])?;
        if !out.ok() {
            return Err(Error::SystemctlFailed(format!(
                "systemctl {verb} {unit} -> exit {} stderr={}",
                out.status,
                out.stderr.trim()
            )));
        }
        Ok(())
    }

    async fn read_status_via_socket(path: &std::path::Path) -> Result<Vec<PeerStatus>> {
        let mut stream = UnixStream::connect(path).await.map_err(Error::Io)?;
        stream
            .write_all(b"status 2\r\n")
            .await
            .map_err(Error::Io)?;
        // Drain until END marker or EOF. The OpenVPN management
        // interface keeps the connection open after `status 2`, so we
        // can't rely on EOF — break on the literal `\nEND\n` boundary.
        let mut buf = Vec::with_capacity(4096);
        let mut chunk = [0u8; 1024];
        loop {
            let n = stream.read(&mut chunk).await.map_err(Error::Io)?;
            if n == 0 {
                break;
            }
            buf.extend_from_slice(&chunk[..n]);
            if buf.windows(5).any(|w| w == b"\nEND\n") {
                break;
            }
            // Defensive cap so a chatty socket cannot exhaust memory.
            if buf.len() > 1_048_576 {
                return Err(Error::Parse(
                    "status 2 reply exceeded 1 MiB without END marker".into(),
                ));
            }
        }
        let text = String::from_utf8_lossy(&buf);
        parse_status_output(&text)
    }
}

impl HasDna for OpenvpnMode {
    fn dna(&self) -> &Dna {
        &self.dna
    }
    fn parent_dna(&self) -> Option<&Dna> {
        self.parent.as_ref()
    }
}

#[async_trait::async_trait]
impl NetworkMode for OpenvpnMode {
    fn mode_name(&self) -> &'static str {
        "openvpn"
    }

    async fn configure(&self, _cfg: &NetworkConfig) -> kei_runtime_core::Result<()> {
        self.invoke_systemctl("start")
            .map_err(kei_runtime_core::Error::from)
    }

    async fn teardown(&self) -> kei_runtime_core::Result<()> {
        self.invoke_systemctl("stop")
            .map_err(kei_runtime_core::Error::from)
    }

    async fn peers(&self) -> kei_runtime_core::Result<Vec<PeerStatus>> {
        let Some(sock) = self.mgmt_socket.clone() else {
            return Ok(Vec::new());
        };
        Self::read_status_via_socket(&sock)
            .await
            .map_err(kei_runtime_core::Error::from)
    }

    fn is_public(&self) -> bool {
        true
    }
}
