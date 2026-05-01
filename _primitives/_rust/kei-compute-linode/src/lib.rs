// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! kei-compute-linode — ComputeProvider impl for Linode (Akamai Cloud) v4 API.
//!
//! Sibling of kei-compute-hetzner. Wave 2 atomar provider crate.
//!
//! - `api`        — thin HTTP client over `https://api.linode.com/v4`
//! - `cloud_init` — render + base64-encode user-data for `metadata.user_data`
//! - `provider`   — `LinodeCompute: ComputeProvider` (DNA + tier policy + status map)
//! - `error`      — local error type, mapped into `kei_runtime_core::Error`
//!
//! Auth: `Authorization: Bearer $LINODE_TOKEN` (env-only, RULE 0.8).

pub mod api;
pub mod cloud_init;
pub mod error;
pub mod provider;

pub use api::{
    CreateInstanceRequest, InstanceMetadata, InstanceResponse, LinodeClient,
};
pub use cloud_init::{render, render_base64, CloudInitTemplate};
pub use error::{Error, Result};
pub use provider::LinodeCompute;
