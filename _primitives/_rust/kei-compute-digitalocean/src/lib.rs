// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! kei-compute-digitalocean — DigitalOcean impl of [`kei_runtime_core::ComputeProvider`].
//!
//! Layout:
//! - [`error`]: local `Error`/`Result` mapping into the runtime-core error.
//! - [`client`]: thin async REST v2 wrapper (mockable base URL).
//! - [`backend`]: [`DigitalOceanBackend`] — DNA-bearing trait impl.
//!
//! Auth: bearer `DIGITALOCEAN_TOKEN`. Base URL defaults to
//! `https://api.digitalocean.com/v2` and is overridable for tests.

pub mod backend;
pub mod client;
pub mod error;

pub use backend::DigitalOceanBackend;
pub use client::DigitalOceanClient;
pub use error::{Error, Result};
