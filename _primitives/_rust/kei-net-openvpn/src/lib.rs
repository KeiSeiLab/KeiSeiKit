// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! kei-net-openvpn — `NetworkMode` impl over a host-resident OpenVPN server.
//!
//! Lifecycle:
//!   * `configure` → `systemctl start openvpn-server@<name>`
//!   * `teardown`  → `systemctl stop  openvpn-server@<name>`
//!   * `peers`     → connect to the management-interface UNIX socket
//!                   (`/var/run/openvpn/<name>.sock` by default), send
//!                   `status 2`, parse `CLIENT_LIST,...` rows.
//!
//! `is_public` returns `true` — OpenVPN is typically routed over a
//! public UDP/TCP endpoint (unlike `tailscale` / `wireguard`-private
//! analogs in the sibling crates).
//!
//! Constructor Pattern (5 cubes, each <200 LOC, one responsibility):
//!   * `error.rs`   — crate error + `From<Error> for kei_runtime_core::Error`
//!   * `runner.rs`  — `Runner` trait abstracting `systemctl` + `SystemRunner`
//!                    (real `std::process::Command`-backed impl)
//!   * `mgmt.rs`    — pure CSV-ish status parser (`parse_status_output`)
//!   * `network.rs` — `OpenvpnMode` struct + `NetworkMode` impl + DNA wiring
//!
//! DNA (literal):
//!     DnaBuilder::new("primitive")
//!         .caps(["PR", "AP", "OV"])
//!         .scope("keiseikit.dev/primitives/kei-net-openvpn")
//!         .body(b"openvpn-systemd-v1")
//!         .build()?
//!
//! Env overrides:
//!   * `OPENVPN_CONFIG_PATH`  — path to `<name>.conf`. Default
//!                              `/etc/openvpn/server/<name>.conf`.
//!   * `OPENVPN_SERVICE_NAME` — `<name>` instance for the
//!                              `openvpn-server@<name>` systemd unit
//!                              and management-socket basename.
//!                              Default `server`.

pub mod error;
pub mod mgmt;
pub mod network;
pub mod runner;

pub use error::{Error, Result};
pub use mgmt::{parse_status_output, ClientRow};
pub use network::OpenvpnMode;
pub use runner::{RunOutput, Runner, SystemRunner};
