// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! [`WireguardMode`] — the `NetworkMode` impl.
//!
//! Brings up a WireGuard interface via `wg-quick up <iface>` (config at
//! `/etc/wireguard/<iface>.conf` or `$WG_CONFIG_PATH`), tears it down via
//! `wg-quick down <iface>`, and reports peer status by parsing
//! `wg show <iface> dump`.
//!
//! Shell-out goes through the [`Runner`] seam so smoke tests substitute a
//! recording mock without touching the host. The async surface (NetworkMode
//! is `async_trait`) is bridged via `tokio::task::spawn_blocking` because
//! the underlying `Runner` is sync (mirror of the kei-llm-mlx pattern).

use crate::error::{Error, Result};
use crate::parse::parse_wg_dump;
use crate::runner::Runner;
use async_trait::async_trait;
use kei_runtime_core::traits::network::{NetworkConfig, NetworkMode, PeerStatus};
use kei_runtime_core::{Dna, DnaBuilder, HasDna};
use std::sync::Arc;

const DEFAULT_IFACE: &str = "wg0";
const ENV_IFACE: &str = "WG_IFACE";

/// Private-mesh WireGuard adapter.
pub struct WireguardMode {
    dna: Dna,
    parent: Option<Dna>,
    runner: Arc<dyn Runner + Send + Sync>,
    iface: String,
}

impl WireguardMode {
    /// Explicit constructor — used by smoke tests with a [`MockRunner`] and
    /// by callers that already know the iface (e.g. multi-interface hosts).
    pub fn new(
        runner: Arc<dyn Runner + Send + Sync>,
        parent: Option<Dna>,
        iface: impl Into<String>,
    ) -> Result<Self> {
        let dna = DnaBuilder::new("primitive")
            .caps(["PR", "AP", "WG"])
            .scope("keiseikit.dev/primitives/kei-net-wireguard")
            .body(b"wireguard-wg-quick-v1")
            .build()?;
        Ok(Self {
            dna,
            parent,
            runner,
            iface: iface.into(),
        })
    }

    /// Build from environment: `$WG_IFACE` (default `wg0`).
    pub fn from_env(
        runner: Arc<dyn Runner + Send + Sync>,
        parent: Option<Dna>,
    ) -> Result<Self> {
        let iface = std::env::var(ENV_IFACE).unwrap_or_else(|_| DEFAULT_IFACE.to_string());
        Self::new(runner, parent, iface)
    }

    pub fn iface(&self) -> &str {
        &self.iface
    }
}

impl HasDna for WireguardMode {
    fn dna(&self) -> &Dna {
        &self.dna
    }
    fn parent_dna(&self) -> Option<&Dna> {
        self.parent.as_ref()
    }
}

/// Run `wg-quick <subcommand> <iface>` and return Ok on exit code 0.
///
/// Kept as a free fn so `network.rs` stays under 200 LOC and the failure
/// envelope is uniform between configure/teardown.
fn run_wg_quick(
    runner: &(dyn Runner + Send + Sync),
    sub: &str,
    iface: &str,
) -> Result<()> {
    let out = runner
        .run("wg-quick", &[sub, iface])
        .map_err(|e| Error::WgCmd(format!("wg-quick {sub} {iface}: {e}")))?;
    if !out.is_success() {
        return Err(Error::WgCmd(format!(
            "wg-quick {sub} {iface} exited code={:?} stderr={}",
            out.code,
            out.stderr.trim()
        )));
    }
    Ok(())
}

/// Run `wg show <iface> dump` and return parsed peer rows.
fn run_wg_show_dump(
    runner: &(dyn Runner + Send + Sync),
    iface: &str,
) -> Result<Vec<PeerStatus>> {
    let out = runner
        .run("wg", &["show", iface, "dump"])
        .map_err(|e| Error::WgCmd(format!("wg show {iface} dump: {e}")))?;
    if !out.is_success() {
        return Err(Error::WgCmd(format!(
            "wg show {iface} dump exited code={:?} stderr={}",
            out.code,
            out.stderr.trim()
        )));
    }
    Ok(parse_wg_dump(&out.stdout))
}

#[async_trait]
impl NetworkMode for WireguardMode {
    fn mode_name(&self) -> &'static str {
        "wireguard"
    }

    /// Bring the interface up. `cfg` is accepted for API parity with the
    /// `NetworkMode` trait but the configuration source is the on-disk
    /// `/etc/wireguard/<iface>.conf` (or `$WG_CONFIG_PATH`); `wg-quick`
    /// reads it directly.
    async fn configure(&self, _cfg: &NetworkConfig) -> kei_runtime_core::Result<()> {
        let runner = self.runner.clone();
        let iface = self.iface.clone();
        tokio::task::spawn_blocking(move || run_wg_quick(&*runner, "up", &iface))
            .await
            .map_err(|e| {
                kei_runtime_core::Error::Network(format!("spawn_blocking: {e}"))
            })?
            .map_err(kei_runtime_core::Error::from)
    }

    async fn teardown(&self) -> kei_runtime_core::Result<()> {
        let runner = self.runner.clone();
        let iface = self.iface.clone();
        tokio::task::spawn_blocking(move || run_wg_quick(&*runner, "down", &iface))
            .await
            .map_err(|e| {
                kei_runtime_core::Error::Network(format!("spawn_blocking: {e}"))
            })?
            .map_err(kei_runtime_core::Error::from)
    }

    async fn peers(&self) -> kei_runtime_core::Result<Vec<PeerStatus>> {
        let runner = self.runner.clone();
        let iface = self.iface.clone();
        tokio::task::spawn_blocking(move || run_wg_show_dump(&*runner, &iface))
            .await
            .map_err(|e| {
                kei_runtime_core::Error::Network(format!("spawn_blocking: {e}"))
            })?
            .map_err(kei_runtime_core::Error::from)
    }

    fn is_public(&self) -> bool {
        false
    }
}
