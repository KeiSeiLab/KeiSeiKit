// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>

use crate::dna::{Dna, HasDna};
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmSpec {
    pub user_dna: Dna,
    pub region: String,
    pub tier: String,
    pub ssh_pubkey: String,
    pub cloud_init: String,
    pub labels: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmHandle {
    pub dna: Dna,
    pub external_id: String,
    pub provider: String,
    pub region: String,
    pub tier: String,
    pub ipv4: Option<String>,
    pub ipv6: Option<String>,
    pub tailscale_ip: Option<String>,
    pub created_at_ms: i64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VmStatus {
    Provisioning,
    Ready,
    Running,
    Stopped,
    Failed,
    Destroyed,
}

#[async_trait::async_trait]
pub trait ComputeProvider: HasDna + Send + Sync {
    fn provider_name(&self) -> &'static str;

    async fn create(&self, spec: &VmSpec) -> Result<VmHandle>;
    async fn destroy(&self, h: &VmHandle) -> Result<()>;
    async fn resize(&self, h: &VmHandle, new_tier: &str) -> Result<VmHandle>;
    async fn status(&self, h: &VmHandle) -> Result<VmStatus>;
    async fn stop(&self, h: &VmHandle) -> Result<()>;
    async fn start(&self, h: &VmHandle) -> Result<()>;

    /// USD micro-cents per hour for the current tier — used by CostGuard.
    fn cost_per_hour_microcents(&self, tier: &str) -> u64;
}
