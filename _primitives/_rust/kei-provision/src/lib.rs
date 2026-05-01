//! kei-provision — unified VPS provisioner (Hetzner + Vultr, extensible).
//!
//! Supersedes `_primitives/provision-hetzner.sh` + `_primitives/provision-vultr.sh`.
//!
//! Layers:
//!   `backend`           — `Backend` trait + `CreateOpts` + `ServerInfo`.
//!   `backends::hetzner` — adapts `hcloud server …` JSON output.
//!   `backends::vultr`   — adapts `vultr-cli instance …` JSON output.
//!   `exec`              — shared `std::process::Command` + env/cli checks.
//!
//! Tests inject a temp PATH containing a fake `hcloud` / `vultr-cli` that
//! emits canned JSON, so no cloud calls happen in CI.

pub mod b64;
pub mod backend;
pub mod backends;
pub mod exec;

pub use backend::{Backend, CreateOpts, ServerInfo};
pub use backends::resolve;
