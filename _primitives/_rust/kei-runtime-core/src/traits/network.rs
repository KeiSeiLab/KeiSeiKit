// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>

use crate::dna::HasDna;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub mode: String,                  // "tailscale" | "wireguard" | ...
    pub hostname: String,
    pub auth_secret_env: Option<String>,
    pub allowed_ports: Vec<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerStatus {
    pub addr: String,
    pub last_seen_ms: i64,
    pub bytes_rx: u64,
    pub bytes_tx: u64,
}

#[async_trait::async_trait]
pub trait NetworkMode: HasDna + Send + Sync {
    fn mode_name(&self) -> &'static str;

    async fn configure(&self, cfg: &NetworkConfig) -> Result<()>;
    async fn teardown(&self) -> Result<()>;
    async fn peers(&self) -> Result<Vec<PeerStatus>>;

    /// True if this mode exposes a public IP (public + cftunnel) or
    /// false if private-only (tailscale/wireguard/localhost).
    fn is_public(&self) -> bool;
}
