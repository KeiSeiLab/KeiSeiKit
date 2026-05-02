// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Denis Parfionovich
//
//! [`BaremetalCompute`] — `ComputeProvider` for user-owned hardware.
//!
//! Differences vs cloud providers:
//! * `create()` does not provision — it registers an SSH connection and
//!   runs the cloud-init shell remotely.
//! * `destroy()` deregisters; user hardware is never powered off.
//! * `resize()`/`start()`/`stop()` return `NotImplemented`.
//! * `cost_per_hour_microcents()` is always 0 (user-owned).

use crate::error::{Error as BmError, Result as BmResult};
use crate::ssh::{is_safe_host, is_safe_user, ping, run_remote, SshTarget};
use kei_runtime_core::traits::compute::{ComputeProvider, VmHandle, VmSpec, VmStatus};
use kei_runtime_core::{Dna, DnaBuilder, HasDna};

/// Bare-metal provider. Stateless w.r.t. registered boxes — the SSH
/// endpoint is encoded in `VmHandle.external_id` (`ssh://user@host[:port]`).
pub struct BaremetalCompute {
    dna: Dna,
    parent: Option<Dna>,
    /// Default SSH key path applied when a `VmSpec` does not override it
    /// via labels. `None` = rely on the user's `~/.ssh/config`.
    default_key_path: Option<String>,
}

impl BaremetalCompute {
    pub fn new(parent: Option<Dna>, default_key_path: Option<String>) -> BmResult<Self> {
        let dna = DnaBuilder::new("primitive")
            .caps(["PR", "AP", "BM"])
            .scope("keiseikit.dev/primitives/kei-compute-baremetal")
            .body(b"baremetal-ssh-v1")
            .build()?;
        Ok(Self {
            dna,
            parent,
            default_key_path,
        })
    }

    fn key_path_for_spec(&self, spec: &VmSpec) -> Option<String> {
        spec.labels
            .iter()
            .find(|(k, _)| k == "ssh_key_path")
            .map(|(_, v)| v.clone())
            .or_else(|| self.default_key_path.clone())
    }

    fn target_for_spec(&self, spec: &VmSpec) -> BmResult<SshTarget> {
        // Region carries `user@host[:port]`. We accept optional `ssh://` prefix.
        let raw = spec.region.trim().trim_start_matches("ssh://");
        let (user, host_port) = raw
            .split_once('@')
            .ok_or_else(|| BmError::InvalidExternalId(spec.region.clone()))?;
        let (host, port) = match host_port.rsplit_once(':') {
            Some((h, p)) => (
                h.to_string(),
                Some(
                    p.parse::<u16>()
                        .map_err(|_| BmError::InvalidExternalId(spec.region.clone()))?,
                ),
            ),
            None => (host_port.to_string(), None),
        };
        if !is_safe_user(user) {
            return Err(BmError::InvalidRegion(format!("user '{user}' fails sanity check")));
        }
        if !is_safe_host(&host) {
            return Err(BmError::InvalidRegion(format!("host '{host}' fails sanity check")));
        }
        let mut t = SshTarget::new(user, host);
        if let Some(p) = port {
            t = t.with_port(p);
        }
        if let Some(k) = self.key_path_for_spec(spec) {
            t = t.with_key(k);
        }
        Ok(t)
    }

    fn target_for_handle(&self, h: &VmHandle) -> BmResult<SshTarget> {
        let raw = h
            .external_id
            .trim()
            .strip_prefix("ssh://")
            .ok_or_else(|| BmError::InvalidExternalId(h.external_id.clone()))?;
        let (user, host_port) = raw
            .split_once('@')
            .ok_or_else(|| BmError::InvalidExternalId(h.external_id.clone()))?;
        let (host, port) = match host_port.rsplit_once(':') {
            Some((h2, p)) => (
                h2.to_string(),
                Some(
                    p.parse::<u16>()
                        .map_err(|_| BmError::InvalidExternalId(h.external_id.clone()))?,
                ),
            ),
            None => (host_port.to_string(), None),
        };
        if !is_safe_user(user) {
            return Err(BmError::InvalidRegion(format!("user '{user}' fails sanity check")));
        }
        if !is_safe_host(&host) {
            return Err(BmError::InvalidRegion(format!("host '{host}' fails sanity check")));
        }
        let mut t = SshTarget::new(user, host);
        if let Some(p) = port {
            t = t.with_port(p);
        }
        if let Some(k) = &self.default_key_path {
            t = t.with_key(k.clone());
        }
        Ok(t)
    }
}

impl HasDna for BaremetalCompute {
    fn dna(&self) -> &Dna {
        &self.dna
    }
    fn parent_dna(&self) -> Option<&Dna> {
        self.parent.as_ref()
    }
}

#[async_trait::async_trait]
impl ComputeProvider for BaremetalCompute {
    fn provider_name(&self) -> &'static str {
        "baremetal"
    }

    async fn create(&self, spec: &VmSpec) -> kei_runtime_core::Result<VmHandle> {
        let target = self.target_for_spec(spec).map_err(BmError::from)?;
        // Cloud-init equivalent: run the user's shell snippet remotely.
        if !spec.cloud_init.trim().is_empty() {
            run_remote(&target, &spec.cloud_init)
                .await
                .map_err(BmError::from)?;
        } else {
            ping(&target).await.map_err(BmError::from)?;
        }
        let host_sha = sha8(target.host.as_bytes());
        let body = format!("{}::{}::{}", target.user, target.host, target.port.unwrap_or(22));
        let vm_dna = DnaBuilder::new("vm-managed")
            .caps(["BM", host_sha.to_uppercase().as_str()])
            .scope(format!("keiseikit.dev/vms/baremetal/{}", target.host))
            .body(body.as_bytes())
            .build()
            .map_err(BmError::from)?;
        Ok(VmHandle {
            dna: vm_dna,
            external_id: target.external_id(),
            provider: "baremetal".to_string(),
            region: spec.region.clone(),
            tier: spec.tier.clone(),
            ipv4: Some(target.host.clone()),
            ipv6: None,
            tailscale_ip: None,
            created_at_ms: now_ms(),
        })
    }

    async fn destroy(&self, _h: &VmHandle) -> kei_runtime_core::Result<()> {
        // No hardware action — pure deregistration. Caller drops the handle.
        Ok(())
    }

    async fn resize(&self, _h: &VmHandle, _new_tier: &str) -> kei_runtime_core::Result<VmHandle> {
        Err(BmError::NotImplemented { op: "resize" }.into())
    }

    async fn status(&self, h: &VmHandle) -> kei_runtime_core::Result<VmStatus> {
        let target = self.target_for_handle(h).map_err(BmError::from)?;
        match ping(&target).await {
            Ok(()) => Ok(VmStatus::Running),
            Err(_) => Ok(VmStatus::Stopped),
        }
    }

    async fn stop(&self, _h: &VmHandle) -> kei_runtime_core::Result<()> {
        Err(BmError::NotImplemented { op: "stop" }.into())
    }

    async fn start(&self, _h: &VmHandle) -> kei_runtime_core::Result<()> {
        Err(BmError::NotImplemented { op: "start" }.into())
    }

    fn cost_per_hour_microcents(&self, _tier: &str) -> u64 {
        0 // user owns the hardware — zero marginal cost to KeiSei
    }
}

/// First 8 hex chars of SHA-256(input). Stable host-fingerprint for DNA caps.
fn sha8(data: &[u8]) -> String {
    use std::hash::{Hash, Hasher};
    // Tiny FNV-1a 64-bit; adequate for an 8-hex DNA cap fingerprint.
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    data.hash(&mut hasher);
    format!("{:08x}", (hasher.finish() & 0xFFFF_FFFF) as u32)
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
    use kei_runtime_core::DnaBuilder;

    fn user_dna() -> Dna {
        DnaBuilder::new("user")
            .cap("EM")
            .scope("keiseikit.dev/users")
            .body(b"alice")
            .build()
            .unwrap()
    }

    #[test]
    fn dna_present_with_bm_cap() {
        let c = BaremetalCompute::new(None, None).unwrap();
        assert_eq!(c.dna().role(), "primitive");
        let caps = c.dna().caps();
        assert!(caps.contains("BM"), "caps must contain BM tag, got {caps}");
        assert_eq!(c.provider_name(), "baremetal");
    }

    #[test]
    fn cost_is_always_zero() {
        let c = BaremetalCompute::new(None, None).unwrap();
        assert_eq!(c.cost_per_hour_microcents("any"), 0);
        assert_eq!(c.cost_per_hour_microcents(""), 0);
    }

    #[tokio::test]
    async fn resize_start_stop_return_not_implemented() {
        let c = BaremetalCompute::new(None, None).unwrap();
        let handle = VmHandle {
            dna: DnaBuilder::new("vm-managed")
                .cap("BM")
                .scope("k")
                .body(b"x")
                .build()
                .unwrap(),
            external_id: "ssh://root@10.0.0.1".into(),
            provider: "baremetal".into(),
            region: "self-hosted".into(),
            tier: "host-1c-1gb".into(),
            ipv4: Some("10.0.0.1".into()),
            ipv6: None,
            tailscale_ip: None,
            created_at_ms: 0,
        };
        assert!(c.resize(&handle, "bigger").await.is_err());
        assert!(c.start(&handle).await.is_err());
        assert!(c.stop(&handle).await.is_err());
    }

    #[test]
    fn target_parse_handles_user_at_host_and_port() {
        let c = BaremetalCompute::new(None, None).unwrap();
        let spec = VmSpec {
            user_dna: user_dna(),
            region: "root@box.example.com:2222".into(),
            tier: "host-1c-1gb".into(),
            ssh_pubkey: String::new(),
            cloud_init: String::new(),
            labels: vec![],
        };
        let t = c.target_for_spec(&spec).unwrap();
        assert_eq!(t.user, "root");
        assert_eq!(t.host, "box.example.com");
        assert_eq!(t.port, Some(2222));
    }

    #[test]
    fn target_rejects_injection_in_region() {
        let c = BaremetalCompute::new(None, None).unwrap();
        for bad in &[
            "-ProxyCommand=evil@host",
            "root@-evil-host",
            "root@host:name",
        ] {
            let spec = VmSpec {
                user_dna: user_dna(),
                region: bad.to_string(),
                tier: "host-1c-1gb".into(),
                ssh_pubkey: String::new(),
                cloud_init: String::new(),
                labels: vec![],
            };
            assert!(c.target_for_spec(&spec).is_err(), "should reject: {bad}");
        }
    }
}
