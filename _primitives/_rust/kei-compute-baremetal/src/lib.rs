// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Denis Parfionovich
//
//! kei-compute-baremetal — ComputeProvider impl for user-owned hardware.
//!
//! Registers an existing SSH-reachable box (VPS, dedicated server, lab box)
//! as a managed VM. `create()` runs the cloud-init shell over SSH; `destroy()`
//! deregisters but never powers off user hardware. `resize()` / `start()` /
//! `stop()` return `NotImplemented` — manual user action only.
//!
//! Auth: SSH key path is passed at construction (RULE 0.8 — never hardcoded).

pub mod error;
pub mod provider;
pub mod ssh;

pub use error::{Error, Result};
pub use provider::BaremetalCompute;
