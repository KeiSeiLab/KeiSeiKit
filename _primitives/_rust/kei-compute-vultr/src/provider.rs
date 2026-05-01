// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! `VultrCompute` — `ComputeProvider` impl for Vultr Cloud v2.
//!
//! DNA caps tag is `VL` (per spec/DNA-CONVENTION.md). Tier table covers
//! the 6 cheapest single-CPU plans (vc2 / vhf, 1-4 GB) — extendable as
//! needed. Cost numbers are µ-cents per hour, converted from Vultr's
//! published $/month rate divided by 730 hours/month and rounded.

use crate::api::{CreateInstanceRequest, VultrClient};
use crate::error::{Error, Result};
use async_trait::async_trait;
use kei_runtime_core::{
    ComputeProvider, Dna, DnaBuilder, HasDna, VmHandle, VmSpec, VmStatus,
};

const PROVIDER: &str = "vultr";

/// Validated Vultr tier id list. Anything else fails `validate_tier`.
const KNOWN_TIERS: &[&str] = &[
    "vc2-1c-1gb",
    "vc2-2c-2gb",
    "vc2-2c-4gb",
    "vhf-1c-1gb",
    "vhf-2c-2gb",
    "vhf-2c-4gb",
];

pub struct VultrCompute {
    dna: Dna,
    client: VultrClient,
    /// Optional Vultr SSH key id (already uploaded out-of-band).
    pub default_sshkey_id: Option<String>,
    /// OS id used at create time when caller doesn't override (Debian 12 = 2136).
    pub default_os_id: u64,
}

impl VultrCompute {
    /// Construct from `VULTR_API_KEY` (env). Errors if missing.
    pub fn from_env() -> Result<Self> {
        let token = std::env::var("VULTR_API_KEY").map_err(|_| Error::MissingToken)?;
        Self::with_token(token)
    }

    pub fn with_token(token: impl Into<String>) -> Result<Self> {
        let client = VultrClient::new(token);
        Self::with_client(client)
    }

    pub fn with_client(client: VultrClient) -> Result<Self> {
        let dna = DnaBuilder::new("primitive")
            .caps(["PR", "AP", "VL"])
            .scope("keiseikit.dev/primitives/compute/vultr")
            .body(b"kei-compute-vultr@0.1.0")
            .build()?;
        Ok(Self {
            dna,
            client,
            default_sshkey_id: None,
            default_os_id: 2136, // Debian 12 x64
        })
    }

    pub fn client(&self) -> &VultrClient {
        &self.client
    }

    fn validate_tier(tier: &str) -> Result<()> {
        if KNOWN_TIERS.contains(&tier) {
            Ok(())
        } else {
            Err(Error::UnknownTier(tier.to_string()))
        }
    }

    fn map_status(s: &str) -> VmStatus {
        match s {
            "active" => VmStatus::Running,
            "pending" => VmStatus::Provisioning,
            "halted" | "stopped" => VmStatus::Stopped,
            _ => VmStatus::Failed,
        }
    }

    fn vm_dna(spec: &VmSpec) -> Result<Dna> {
        let caps_tier = spec.tier.replace('-', "").to_uppercase();
        let caps_region = spec.region.to_uppercase();
        let body = format!(
            "tier={};region={};ssh_pubkey_sha={};cloud_init_len={}",
            spec.tier,
            spec.region,
            short_hash(spec.ssh_pubkey.as_bytes()),
            spec.cloud_init.len(),
        );
        let dna = DnaBuilder::new("vm-managed")
            .caps([
                "VL".to_string(),
                caps_tier,
                caps_region,
            ])
            .scope(format!("keiseikit.dev/vms/vultr/{}", spec.region))
            .body(body)
            .build()?;
        Ok(dna)
    }
}

fn short_hash(bytes: &[u8]) -> String {
    // 8-char rolling DJB2 — enough for inclusion in a DNA body, not
    // collision-resistant. Real identity comes from DnaBuilder.body_sha.
    let mut h: u64 = 5381;
    for &b in bytes {
        h = h.wrapping_mul(33).wrapping_add(b as u64);
    }
    format!("{:08x}", h & 0xFFFF_FFFF)
}

impl HasDna for VultrCompute {
    fn dna(&self) -> &Dna {
        &self.dna
    }
    fn parent_dna(&self) -> Option<&Dna> {
        None
    }
}

#[async_trait]
impl ComputeProvider for VultrCompute {
    fn provider_name(&self) -> &'static str {
        PROVIDER
    }

    async fn create(&self, spec: &VmSpec) -> kei_runtime_core::Result<VmHandle> {
        Self::validate_tier(&spec.tier).map_err(kei_runtime_core::Error::from)?;
        let dna = Self::vm_dna(spec).map_err(kei_runtime_core::Error::from)?;
        // Vultr requires base64-encoded user_data.
        let user_data = if spec.cloud_init.is_empty() {
            None
        } else {
            Some(crate::cloud_init::encode_base64(spec.cloud_init.as_bytes()))
        };
        let label = dna.nonce();
        let req = CreateInstanceRequest {
            region: spec.region.clone(),
            plan: spec.tier.clone(),
            label: label.clone(),
            hostname: label,
            os_id: Some(self.default_os_id),
            iso_id: None,
            user_data,
            sshkey_id: self
                .default_sshkey_id
                .as_ref()
                .map(|k| vec![k.clone()])
                .unwrap_or_default(),
            tags: spec
                .labels
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect(),
        };
        let r = self
            .client
            .create_instance(&req)
            .await
            .map_err(kei_runtime_core::Error::from)?;
        Ok(VmHandle {
            dna,
            external_id: r.instance.id,
            provider: PROVIDER.to_string(),
            region: r.instance.region,
            tier: r.instance.plan,
            ipv4: ip_or_none(&r.instance.main_ip),
            ipv6: ip_or_none(&r.instance.v6_main_ip),
            tailscale_ip: None,
            created_at_ms: now_ms(),
        })
    }

    async fn destroy(&self, h: &VmHandle) -> kei_runtime_core::Result<()> {
        self.client
            .delete_instance(&h.external_id)
            .await
            .map_err(kei_runtime_core::Error::from)
    }

    async fn resize(
        &self,
        h: &VmHandle,
        new_tier: &str,
    ) -> kei_runtime_core::Result<VmHandle> {
        Self::validate_tier(new_tier).map_err(kei_runtime_core::Error::from)?;
        let r = self
            .client
            .change_plan(&h.external_id, new_tier)
            .await
            .map_err(kei_runtime_core::Error::from)?;
        let mut new_handle = h.clone();
        new_handle.tier = r.instance.plan;
        Ok(new_handle)
    }

    async fn status(&self, h: &VmHandle) -> kei_runtime_core::Result<VmStatus> {
        let r = self
            .client
            .get_instance(&h.external_id)
            .await
            .map_err(kei_runtime_core::Error::from)?;
        Ok(Self::map_status(&r.instance.status))
    }

    async fn stop(&self, h: &VmHandle) -> kei_runtime_core::Result<()> {
        self.client
            .halt_instance(&h.external_id)
            .await
            .map_err(kei_runtime_core::Error::from)
    }

    async fn start(&self, h: &VmHandle) -> kei_runtime_core::Result<()> {
        self.client
            .start_instance(&h.external_id)
            .await
            .map_err(kei_runtime_core::Error::from)
    }

    fn cost_per_hour_microcents(&self, tier: &str) -> u64 {
        match tier {
            "vc2-1c-1gb" => 833,
            "vc2-2c-2gb" => 1667,
            "vc2-2c-4gb" => 3333,
            "vhf-1c-1gb" => 1083,
            "vhf-2c-2gb" => 2750,
            "vhf-2c-4gb" => 5000,
            _ => 0,
        }
    }
}

fn ip_or_none(s: &str) -> Option<String> {
    if s.is_empty() || s == "0.0.0.0" || s == "::" {
        None
    } else {
        Some(s.to_string())
    }
}

fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_tiers_accepts_known_rejects_unknown() {
        for t in KNOWN_TIERS {
            VultrCompute::validate_tier(t).expect("known tier ok");
        }
        let err = VultrCompute::validate_tier("nonsense").unwrap_err();
        assert!(matches!(err, Error::UnknownTier(_)));
    }

    #[test]
    fn map_status_covers_documented_values() {
        assert_eq!(VultrCompute::map_status("active"), VmStatus::Running);
        assert_eq!(VultrCompute::map_status("pending"), VmStatus::Provisioning);
        assert_eq!(VultrCompute::map_status("halted"), VmStatus::Stopped);
        assert_eq!(VultrCompute::map_status("stopped"), VmStatus::Stopped);
        assert_eq!(VultrCompute::map_status("suspended"), VmStatus::Failed);
    }

    #[test]
    fn cost_known_tiers_nonzero_unknown_zero() {
        let c = VultrCompute::with_token("t").expect("ok");
        for t in KNOWN_TIERS {
            assert!(c.cost_per_hour_microcents(t) > 0, "tier {t} must cost > 0");
        }
        assert_eq!(c.cost_per_hour_microcents("nonsense"), 0);
    }

    #[test]
    fn dna_present_on_constructor() {
        let c = VultrCompute::with_token("t").expect("ok");
        let dna = c.dna();
        assert_eq!(dna.role(), "primitive");
        let caps = dna.caps();
        assert!(caps.contains("VL"), "caps must contain VL: got {caps}");
        assert!(caps.contains("PR"));
        assert!(caps.contains("AP"));
    }
}
