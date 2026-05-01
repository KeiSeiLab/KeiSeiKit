// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! [`IpsecMode`] — DNA-bearing [`NetworkMode`] impl that brings a
//! strongSwan child SA up / down via `swanctl`.
//!
//! Mechanism (per RFC: each step is a single `swanctl` invocation routed
//! through [`Runner`] for testability):
//!
//! * `configure`  → `swanctl --load-all` (refresh `/etc/swanctl/`),
//!                  then `swanctl --initiate --child <child_name>`.
//! * `teardown`   → `swanctl --terminate --child <child_name>`.
//! * `peers`      → `swanctl --list-sas`, parsed by [`crate::parse`].
//!
//! `is_public() = true`. Sibling tailscale / wireguard NetworkMode adapters
//! return `false`.

use crate::error::{Error, Result as IpsecResult};
use crate::parse::parse_sas_output;
use crate::runner::Runner;
use async_trait::async_trait;
use kei_runtime_core::traits::network::{NetworkConfig, NetworkMode, PeerStatus};
use kei_runtime_core::{Dna, DnaBuilder, HasDna, Result as CoreResult};
use std::sync::Arc;

/// Default config root for `swanctl` (overridable via env
/// `SWANCTL_CONFIG_DIR`). Used informationally — strongSwan reads this
/// path automatically; we surface the env var so operators can move it.
pub const DEFAULT_CONFIG_DIR: &str = "/etc/swanctl";

/// Default child SA name (overridable via env `IPSEC_CHILD_NAME`).
pub const DEFAULT_CHILD_NAME: &str = "home";

/// strongSwan / swanctl `NetworkMode`. Construction injects a
/// [`Runner`] so unit tests can swap in [`crate::runner::MockRunner`].
pub struct IpsecMode {
    dna: Dna,
    parent: Option<Dna>,
    runner: Arc<dyn Runner + Send + Sync>,
    child_name: String,
    config_dir: String,
}

impl IpsecMode {
    /// Construct with explicit runner + child name.
    pub fn new(
        runner: Arc<dyn Runner + Send + Sync>,
        parent: Option<Dna>,
        child_name: impl Into<String>,
    ) -> IpsecResult<Self> {
        let dna = DnaBuilder::new("primitive")
            .caps(["PR", "AP", "IP"])
            .scope("keiseikit.dev/primitives/kei-net-ipsec")
            .body(b"ipsec-strongswan-v1")
            .build()?;
        Ok(Self {
            dna,
            parent,
            runner,
            child_name: child_name.into(),
            config_dir: DEFAULT_CONFIG_DIR.into(),
        })
    }

    /// Construct from environment: `IPSEC_CHILD_NAME` (default `home`)
    /// and `SWANCTL_CONFIG_DIR` (default `/etc/swanctl`).
    pub fn from_env(
        runner: Arc<dyn Runner + Send + Sync>,
        parent: Option<Dna>,
    ) -> IpsecResult<Self> {
        let child = std::env::var("IPSEC_CHILD_NAME").unwrap_or_else(|_| DEFAULT_CHILD_NAME.into());
        let cfg = std::env::var("SWANCTL_CONFIG_DIR")
            .unwrap_or_else(|_| DEFAULT_CONFIG_DIR.into());
        let mut m = Self::new(runner, parent, child)?;
        m.config_dir = cfg;
        Ok(m)
    }

    /// Inspect the child SA name this mode operates on.
    pub fn child_name(&self) -> &str {
        &self.child_name
    }

    /// Inspect the swanctl config directory (informational).
    pub fn config_dir(&self) -> &str {
        &self.config_dir
    }

    fn invoke(&self, args: &[&str]) -> IpsecResult<String> {
        let out = self
            .runner
            .run("swanctl", args)
            .map_err(|e| Error::SwanctlFailed(e.to_string()))?;
        if !out.is_success() {
            return Err(Error::SwanctlFailed(format!(
                "swanctl {} exited code={:?} stderr={}",
                args.join(" "),
                out.code,
                out.stderr.trim()
            )));
        }
        Ok(out.stdout)
    }
}

impl HasDna for IpsecMode {
    fn dna(&self) -> &Dna {
        &self.dna
    }
    fn parent_dna(&self) -> Option<&Dna> {
        self.parent.as_ref()
    }
}

#[async_trait]
impl NetworkMode for IpsecMode {
    fn mode_name(&self) -> &'static str {
        "ipsec"
    }

    async fn configure(&self, _cfg: &NetworkConfig) -> CoreResult<()> {
        // Refresh strongSwan in-memory config from disk, then bring the
        // child SA up. `--load-all` is idempotent; running it before
        // `--initiate` covers the common case of an operator having
        // edited `swanctl.conf` since the daemon last refreshed.
        self.invoke(&["--load-all"]).map_err(|e: Error| -> kei_runtime_core::Error { e.into() })?;
        self.invoke(&["--initiate", "--child", &self.child_name])
            .map_err(|e: Error| -> kei_runtime_core::Error { e.into() })?;
        Ok(())
    }

    async fn teardown(&self) -> CoreResult<()> {
        self.invoke(&["--terminate", "--child", &self.child_name])
            .map_err(|e: Error| -> kei_runtime_core::Error { e.into() })?;
        Ok(())
    }

    async fn peers(&self) -> CoreResult<Vec<PeerStatus>> {
        let stdout = self.invoke(&["--list-sas"]).map_err(|e: Error| -> kei_runtime_core::Error { e.into() })?;
        Ok(parse_sas_output(&stdout))
    }

    fn is_public(&self) -> bool {
        true
    }
}
