//! Backend trait + shared data types for the unified provisioner.
//!
//! A `Backend` shells out to an external CLI (hcloud / vultr-cli / future
//! aws / doctl / linode-cli). All IO is through the `Backend` methods;
//! `main.rs` never touches `std::process::Command` directly.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Opts passed to `Backend::create`. Fields are `Option` because every
/// backend has different defaults (hetzner = cx22/fsn1/debian-12; vultr =
/// vc2-1c-1gb/ams/resolve-Debian-12). Backend fills blanks.
#[derive(Debug, Default, Clone)]
pub struct CreateOpts {
    /// e.g. `cx22` (hetzner), `vc2-1c-1gb` (vultr).
    pub server_type: Option<String>,
    /// e.g. `fsn1` (hetzner), `ams` (vultr).
    pub location: Option<String>,
    /// e.g. `debian-12` (hetzner), `2136` (vultr os-id).
    pub image: Option<String>,
    /// SSH key id/name (backend-specific).
    pub ssh_key: Option<String>,
    /// Firewall name (hetzner) / group id (vultr).
    pub firewall: Option<String>,
    /// Cloud-init user-data file path.
    pub user_data_path: Option<PathBuf>,
}

/// Normalized server info across all backends.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub id: String,
    pub name: String,
    pub ipv4: Option<String>,
    pub status: String,
    /// Raw backend JSON for details the normalized fields drop
    /// (region, plan, power status, datacenter, etc).
    pub backend_specific: serde_json::Value,
}

/// Implemented by each cloud provider adapter.
pub trait Backend {
    /// Short identifier — `"hetzner"`, `"vultr"`.
    fn name(&self) -> &'static str;

    /// Create a server with `name` or return existing (idempotent).
    fn create(&self, name: &str, opts: &CreateOpts) -> Result<ServerInfo>;

    /// `Ok(None)` if absent; never fails on absence.
    fn status(&self, name: &str) -> Result<Option<ServerInfo>>;

    /// Idempotent: absent server = Ok(()). `force` skips confirm prompt.
    fn destroy(&self, name: &str, force: bool) -> Result<()>;

    /// All servers owned by the current token.
    fn list(&self) -> Result<Vec<ServerInfo>>;
}
