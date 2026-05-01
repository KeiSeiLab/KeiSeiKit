// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! kei-compute-vultr — Vultr Cloud (v2) impl of `ComputeProvider`.
//!
//! Mirrors `kei-compute-hetzner` structurally. Wire format and tier table
//! per Vultr v2 docs (https://www.vultr.com/api/). User-data is base64
//! encoded by Vultr requirement.

pub mod api;
pub mod cloud_init;
pub mod error;
pub mod provider;

pub use api::VultrClient;
pub use cloud_init::CloudInitSpec;
pub use error::{Error, Result};
pub use provider::VultrCompute;
