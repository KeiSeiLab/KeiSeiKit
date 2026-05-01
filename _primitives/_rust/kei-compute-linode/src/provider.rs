// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! `LinodeCompute` — `ComputeProvider` impl over `LinodeClient`.
//!
//! - DNA: `primitive::PR-AP-LN::<scope8>::<body8>-<nonce>` for the
//!   provider itself; `vm-managed::LN-<TYPE_UPPER>-<REGION_UPPER>::…`
//!   for each provisioned VM.
//! - Tier policy: only the slugs in [`TIERS`] are accepted.
//! - Cost: USD micro-cents/hour, on-demand pricing as of 2026-04
//!   (RULE 0.4: pricing is per Linode public pricing page; values
//!   carried as constants inline so this crate is self-contained).

use crate::api::{
    CreateInstanceRequest, InstanceMetadata, LinodeClient,
};
use crate::cloud_init;
use crate::error::Error;
use async_trait::async_trait;
use kei_runtime_core::{
    dna::{Dna, DnaBuilder, HasDna},
    error::Result as RtResult,
    traits::compute::{ComputeProvider, VmHandle, VmSpec, VmStatus},
};

/// Allowed Linode tier slugs (subset relevant to the kit's workloads).
pub const TIERS: &[&str] = &[
    "g6-nanode-1",
    "g6-standard-1",
    "g6-standard-2",
    "g6-standard-4",
    "g6-dedicated-2",
    "g6-dedicated-4",
];

/// Provider impl. Holds the HTTP client + its DNA serial.
pub struct LinodeCompute {
    dna: Dna,
    client: LinodeClient,
    default_image: String,
}

impl LinodeCompute {
    /// Build with explicit client + default image (e.g. `linode/debian12`).
    pub fn new(client: LinodeClient, default_image: impl Into<String>) -> Result<Self, Error> {
        let dna = DnaBuilder::new("primitive")
            .caps(["PR", "AP", "LN"])
            .scope("keiseikit.dev/compute/linode")
            .body(b"linode-api-v4")
            .build()?;
        Ok(Self {
            dna,
            client,
            default_image: default_image.into(),
        })
    }

    /// Map Linode `status` strings to the trait's `VmStatus`.
    pub fn map_status(s: &str) -> VmStatus {
        match s {
            "running" => VmStatus::Running,
            "provisioning" | "booting" => VmStatus::Provisioning,
            "shutting_down" | "offline" => VmStatus::Stopped,
            "deleting" => VmStatus::Destroyed,
            _ => VmStatus::Failed,
        }
    }

    fn require_tier(tier: &str) -> Result<(), Error> {
        if TIERS.iter().any(|t| *t == tier) {
            Ok(())
        } else {
            Err(Error::InvalidTier(tier.to_string()))
        }
    }

    fn vm_dna(spec: &VmSpec, body: &[u8]) -> Result<Dna, Error> {
        let type_cap = tier_cap(&spec.tier);
        let region_cap = region_cap(&spec.region);
        let dna = DnaBuilder::new("vm-managed")
            .caps(["LN", &type_cap, &region_cap])
            .scope(format!(
                "keiseikit.dev/vms/linode/{}/{}",
                spec.region, spec.tier
            ))
            .body(body)
            .build()?;
        Ok(dna)
    }
}

fn tier_cap(tier: &str) -> String {
    // "g6-standard-2" -> "G6STANDARD2"; cap segments are uppercase
    // ASCII alphanumeric only, so we strip dashes.
    tier.replace('-', "").to_uppercase()
}

fn region_cap(region: &str) -> String {
    region.replace('-', "").to_uppercase()
}

impl HasDna for LinodeCompute {
    fn dna(&self) -> &Dna {
        &self.dna
    }
    fn parent_dna(&self) -> Option<&Dna> {
        None
    }
}

#[async_trait]
impl ComputeProvider for LinodeCompute {
    fn provider_name(&self) -> &'static str {
        "linode"
    }

    async fn create(&self, spec: &VmSpec) -> RtResult<VmHandle> {
        Self::require_tier(&spec.tier).map_err(kei_runtime_core::Error::from)?;
        // Encode cloud-init for metadata.user_data (Linode-specific).
        let user_data_b64 = base64_encode(&spec.cloud_init);
        let label = format!("kei-{}", spec.user_dna.nonce());
        let req = CreateInstanceRequest {
            label: label.clone(),
            region: spec.region.clone(),
            type_: spec.tier.clone(),
            image: self.default_image.clone(),
            root_pass: None,
            authorized_keys: Some(vec![spec.ssh_pubkey.clone()]),
            stackscript_data: None,
            metadata: Some(InstanceMetadata {
                user_data: user_data_b64,
            }),
            tags: None,
        };
        let resp = self.client.create_instance(&req).await.map_err(kei_runtime_core::Error::from)?;
        let body = format!(
            r#"{{"id":{},"label":"{}","tier":"{}","region":"{}"}}"#,
            resp.id, resp.label, resp.type_, resp.region
        );
        let dna = Self::vm_dna(spec, body.as_bytes()).map_err(kei_runtime_core::Error::from)?;
        Ok(VmHandle {
            dna,
            external_id: resp.id.to_string(),
            provider: "linode".into(),
            region: resp.region,
            tier: resp.type_,
            ipv4: resp.ipv4.first().cloned(),
            ipv6: resp.ipv6,
            tailscale_ip: None,
            created_at_ms: now_ms(),
        })
    }

    async fn destroy(&self, h: &VmHandle) -> RtResult<()> {
        let id = parse_id(&h.external_id)?;
        self.client.delete_instance(id).await.map_err(kei_runtime_core::Error::from)?;
        Ok(())
    }

    async fn resize(&self, h: &VmHandle, new_tier: &str) -> RtResult<VmHandle> {
        Self::require_tier(new_tier).map_err(kei_runtime_core::Error::from)?;
        let id = parse_id(&h.external_id)?;
        self.client.resize(id, new_tier).await.map_err(kei_runtime_core::Error::from)?;
        let mut next = h.clone();
        next.tier = new_tier.to_string();
        Ok(next)
    }

    async fn status(&self, h: &VmHandle) -> RtResult<VmStatus> {
        let id = parse_id(&h.external_id)?;
        let resp = self.client.get_instance(id).await.map_err(kei_runtime_core::Error::from)?;
        Ok(Self::map_status(&resp.status))
    }

    async fn stop(&self, h: &VmHandle) -> RtResult<()> {
        let id = parse_id(&h.external_id)?;
        self.client.shutdown(id).await.map_err(kei_runtime_core::Error::from)?;
        Ok(())
    }

    async fn start(&self, h: &VmHandle) -> RtResult<()> {
        let id = parse_id(&h.external_id)?;
        self.client.boot(id).await.map_err(kei_runtime_core::Error::from)?;
        Ok(())
    }

    fn cost_per_hour_microcents(&self, tier: &str) -> u64 {
        match tier {
            // Pricing per linode.com/pricing (on-demand, 2026-04 snapshot).
            // Monthly USD / 730 hours, expressed as USD micro-cents (×1e6 / 100 = ×10_000).
            // $5/mo  → ~$0.0068/h → 685 micro-cents/h
            "g6-nanode-1" => 685,
            // $12/mo → ~$0.01644/h → 1644
            "g6-standard-1" => 1644,
            // $24/mo → ~$0.03287/h → 3287
            "g6-standard-2" => 3287,
            // $48/mo → ~$0.06575/h → 6575
            "g6-standard-4" => 6575,
            // $36/mo → ~$0.04931/h → 4931
            "g6-dedicated-2" => 4931,
            // $72/mo → ~$0.09863/h → 9863
            "g6-dedicated-4" => 9863,
            _ => 0,
        }
    }
}

fn parse_id(s: &str) -> RtResult<i64> {
    s.parse::<i64>().map_err(|e| {
        kei_runtime_core::Error::Provider(format!("invalid linode id '{s}': {e}"))
    })
}

fn now_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

fn base64_encode(s: &str) -> String {
    use base64::engine::general_purpose::STANDARD;
    use base64::Engine as _;
    // Re-use renderer's encoder if a CloudInitTemplate was used; here
    // we accept any pre-rendered string and encode it raw.
    let _ = cloud_init::render; // keep module path live for clarity
    STANDARD.encode(s.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_dna_carries_ln_cap() {
        let cli = LinodeClient::new("tkn");
        let p = LinodeCompute::new(cli, "linode/debian12").expect("dna");
        let caps = p.dna().caps();
        assert!(caps.contains("LN"), "caps={caps}");
        assert!(caps.contains("PR"));
        assert!(caps.contains("AP"));
        assert_eq!(p.provider_name(), "linode");
    }

    #[test]
    fn map_status_covers_known_states() {
        assert_eq!(LinodeCompute::map_status("running"), VmStatus::Running);
        assert_eq!(LinodeCompute::map_status("provisioning"), VmStatus::Provisioning);
        assert_eq!(LinodeCompute::map_status("booting"), VmStatus::Provisioning);
        assert_eq!(LinodeCompute::map_status("shutting_down"), VmStatus::Stopped);
        assert_eq!(LinodeCompute::map_status("offline"), VmStatus::Stopped);
        assert_eq!(LinodeCompute::map_status("deleting"), VmStatus::Destroyed);
        assert_eq!(LinodeCompute::map_status("???"), VmStatus::Failed);
    }

    #[test]
    fn cost_table_matches_pricing_constants() {
        let cli = LinodeClient::new("tkn");
        let p = LinodeCompute::new(cli, "linode/debian12").expect("dna");
        assert_eq!(p.cost_per_hour_microcents("g6-nanode-1"), 685);
        assert_eq!(p.cost_per_hour_microcents("g6-standard-1"), 1644);
        assert_eq!(p.cost_per_hour_microcents("g6-standard-2"), 3287);
        assert_eq!(p.cost_per_hour_microcents("g6-standard-4"), 6575);
        assert_eq!(p.cost_per_hour_microcents("g6-dedicated-2"), 4931);
        assert_eq!(p.cost_per_hour_microcents("g6-dedicated-4"), 9863);
        assert_eq!(p.cost_per_hour_microcents("unknown"), 0);
    }

    #[test]
    fn invalid_tier_rejected() {
        assert!(LinodeCompute::require_tier("g6-standard-1").is_ok());
        let e = LinodeCompute::require_tier("g99-bogus").unwrap_err();
        match e {
            Error::InvalidTier(s) => assert_eq!(s, "g99-bogus"),
            other => panic!("expected InvalidTier, got {other:?}"),
        }
    }
}
