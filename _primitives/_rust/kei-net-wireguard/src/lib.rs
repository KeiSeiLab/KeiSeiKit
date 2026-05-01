// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! kei-net-wireguard — Wave 9 NetworkMode impl over `wg-quick`/`wg`.
//!
//! Constructor Pattern: 4 cubes
//!   * `error`   — Error/Result + conversions into `kei_runtime_core::Error`
//!   * `runner`  — `Runner` shell-out seam + `RunOutput` + `SystemRunner`
//!   * `parse`   — `parse_wg_dump` (peer rows from `wg show <iface> dump`)
//!   * `network` — `WireguardMode` (the `NetworkMode` impl)
//!
//! Brings up a WireGuard interface via `wg-quick up <iface>` (config at
//! `/etc/wireguard/<iface>.conf` or `$WG_CONFIG_PATH`), tears down via
//! `wg-quick down <iface>`, and reports peer status by parsing the
//! tab-separated output of `wg show <iface> dump`. The interface name
//! defaults to `wg0` and may be overridden via `$WG_IFACE`.
//!
//! Mode is private — `is_public()` is `false` (WireGuard is a private
//! mesh, not a public ingress). Mirror of the kei-net-tailscale sibling.

pub mod error;
pub mod network;
pub mod parse;
pub mod runner;

pub use error::{Error, Result};
pub use network::WireguardMode;
pub use parse::parse_wg_dump;
pub use runner::{RunOutput, Runner, SystemRunner};
