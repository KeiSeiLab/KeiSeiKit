// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! kei-net-ipsec — IPsec impl of [`kei_runtime_core::NetworkMode`] via
//! `swanctl` shell-out (strongSwan).
//!
//! Layout (Constructor Pattern: 1 file = 1 cube, ≤200 LOC each):
//! - [`error`]: local `Error`/`Result` mapping into the runtime-core error.
//! - [`runner`]: [`Runner`] trait + [`SystemRunner`] / [`MockRunner`] —
//!   single subprocess seam (mirror of `kei-llm-mlx::runner`).
//! - [`parse`]: SA-stanza parser for `swanctl --list-sas` text output.
//! - [`network`]: [`IpsecMode`] — DNA-bearing `NetworkMode` impl.
//!
//! Mode flags:
//! - `is_public() = true` (IPsec exposes a routable public path; sibling
//!   tailscale / wireguard adapters return `false`).
//!
//! Env:
//! - `SWANCTL_CONFIG_DIR` — override `/etc/swanctl/` config root.
//! - `IPSEC_CHILD_NAME` — child SA name to bring up / tear down (default
//!   `home`).

pub mod error;
pub mod network;
pub mod parse;
pub mod runner;

pub use error::{Error, Result};
pub use network::{IpsecMode, DEFAULT_CHILD_NAME, DEFAULT_CONFIG_DIR};
pub use parse::parse_sas_output;
pub use runner::{MockRunner, RunOutput, Runner, SystemRunner};
