// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! [`DigitalOceanBackend`] — DNA-bearing [`ComputeProvider`] impl.
//!
//! Maps DO droplet operations onto the runtime-core trait surface. The
//! `external_id` on a [`VmHandle`] is the droplet's numeric `id` formatted
//! as a string.

use crate::client::{CreateDropletSpec, DigitalOceanClient, Droplet};
use crate::error::{Error, Result as DoResult};
use async_trait::async_trait;
use kei_runtime_core::traits::compute::{ComputeProvider, VmHandle, VmSpec, VmStatus};
use kei_runtime_core::{Dna, DnaBuilder, HasDna, Result as CoreResult};
use std::time::{SystemTime, UNIX_EPOCH};

/// Default DO image when caller does not encode it in `tier`.
pub const DEFAULT_IMAGE: &str = "ubuntu-24-04-x64";

/// DigitalOcean backend. `parent` is the operator/owner DNA (optional).
#[derive(Debug, Clone)]
pub struct DigitalOceanBackend {
    dna: Dna,
    parent: Option<Dna>,
    client: DigitalOceanClient,
}

impl DigitalOceanBackend {
    /// Build a backend with a fresh DNA serial. `image` defaults to
    /// [`DEFAULT_IMAGE`] when callers pass `None`.
    pub fn new(client: DigitalOceanClient, parent: Option<Dna>) -> DoResult<Self> {
        let dna = DnaBuilder::new("primitive")
            .caps(["PR", "AP", "DO"])
            .scope("keiseikit.dev/primitives/kei-compute-digitalocean")
            .body(b"do-v2")
            .build()?;
        Ok(Self { dna, parent, client })
    }

    fn now_ms() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0)
    }

    fn parse_id(h: &VmHandle) -> DoResult<u64> {
        h.external_id
            .parse::<u64>()
            .map_err(|e| Error::Api(format!("bad external_id {:?}: {}", h.external_id, e)))
    }

    fn make_handle(d: &Droplet, region: &str, tier: &str, user_dna: &Dna) -> VmHandle {
        let ipv4 = d
            .networks
            .v4
            .iter()
            .find(|n| n.kind == "public")
            .map(|n| n.ip_address.clone());
        let ipv6 = d.networks.v6.iter().find(|n| n.kind == "public").map(|n| n.ip_address.clone());
        VmHandle {
            dna: user_dna.clone(),
            external_id: d.id.to_string(),
            provider: "digitalocean".into(),
            region: if region.is_empty() { d.region.slug.clone() } else { region.into() },
            tier: tier.into(),
            ipv4,
            ipv6,
            tailscale_ip: None,
            created_at_ms: Self::now_ms(),
        }
    }
}

impl HasDna for DigitalOceanBackend {
    fn dna(&self) -> &Dna {
        &self.dna
    }
    fn parent_dna(&self) -> Option<&Dna> {
        self.parent.as_ref()
    }
}

/// Map raw DO status string → [`VmStatus`].
///
/// DO statuses (from API docs): `new`, `active`, `off`, `archive`.
/// We treat the rare `archive` as `Stopped` (off-but-preserved). Anything
/// unrecognized maps to `Failed` so callers don't silently misread state.
pub fn map_status(raw: &str) -> VmStatus {
    match raw {
        "new" => VmStatus::Provisioning,
        "active" => VmStatus::Running,
        "off" | "archive" => VmStatus::Stopped,
        _ => VmStatus::Failed,
    }
}

#[async_trait]
impl ComputeProvider for DigitalOceanBackend {
    fn provider_name(&self) -> &'static str {
        "digitalocean"
    }

    async fn create(&self, spec: &VmSpec) -> CoreResult<VmHandle> {
        let create = CreateDropletSpec {
            name: format!("kei-{}", spec.user_dna.nonce()),
            region: spec.region.clone(),
            size: spec.tier.clone(),
            image: DEFAULT_IMAGE.into(),
            ssh_keys: if spec.ssh_pubkey.is_empty() {
                Vec::new()
            } else {
                vec![spec.ssh_pubkey.clone()]
            },
            user_data: if spec.cloud_init.is_empty() {
                None
            } else {
                Some(spec.cloud_init.clone())
            },
            tags: spec.labels.iter().map(|(k, v)| format!("{k}={v}")).collect(),
        };
        let d = self.client.create_droplet(&create).await.map_err(|e: Error| -> kei_runtime_core::Error { e.into() })?;
        Ok(Self::make_handle(&d, &spec.region, &spec.tier, &spec.user_dna))
    }

    async fn destroy(&self, h: &VmHandle) -> CoreResult<()> {
        let id = Self::parse_id(h).map_err(|e: Error| -> kei_runtime_core::Error { e.into() })?;
        self.client.delete(id).await.map_err(|e: Error| -> kei_runtime_core::Error { e.into() })?;
        Ok(())
    }

    async fn resize(&self, _h: &VmHandle, _new_tier: &str) -> CoreResult<VmHandle> {
        Err(kei_runtime_core::Error::Provider(
            "digitalocean resize: power_off + resize action not implemented in v0.1".into(),
        ))
    }

    async fn status(&self, h: &VmHandle) -> CoreResult<VmStatus> {
        let id = Self::parse_id(h).map_err(|e: Error| -> kei_runtime_core::Error { e.into() })?;
        match self.client.get_droplet(id).await {
            Ok(d) => Ok(map_status(&d.status)),
            Err(Error::NotFound(_)) => Ok(VmStatus::Destroyed),
            Err(e) => Err(e.into()),
        }
    }

    async fn stop(&self, h: &VmHandle) -> CoreResult<()> {
        let id = Self::parse_id(h).map_err(|e: Error| -> kei_runtime_core::Error { e.into() })?;
        self.client.shutdown(id).await.map_err(|e: Error| -> kei_runtime_core::Error { e.into() })?;
        Ok(())
    }

    async fn start(&self, h: &VmHandle) -> CoreResult<()> {
        let id = Self::parse_id(h).map_err(|e: Error| -> kei_runtime_core::Error { e.into() })?;
        self.client.power_on(id).await.map_err(|e: Error| -> kei_runtime_core::Error { e.into() })?;
        Ok(())
    }

    fn cost_per_hour_microcents(&self, tier: &str) -> u64 {
        // DO pricing snapshot 2026-04-28 [E1: digitalocean.com/pricing/droplets]
        // Values are USD micro-cents (1 USD = 1e8 microcents).
        match tier {
            "s-1vcpu-512mb-10gb" => 595,    // $0.00595/hr  (~$4/mo)
            "s-1vcpu-1gb" => 893,           // $0.00893/hr  (~$6/mo)
            "s-1vcpu-2gb" => 1786,          // $0.01786/hr  (~$12/mo)
            "s-2vcpu-2gb" => 2679,          // $0.02679/hr  (~$18/mo)
            "s-2vcpu-4gb" => 3571,          // $0.03571/hr  (~$24/mo)
            "s-4vcpu-8gb" => 7143,          // $0.07143/hr  (~$48/mo)
            _ => 1786,                      // unknown tier: assume $12/mo equiv
        }
    }
}
