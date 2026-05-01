// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! kei-svc-systemd — ServiceManager impl for Linux systemd.

pub mod error;
pub mod manager;
pub mod templates;

pub use error::{Error, Result};
pub use manager::SystemdManager;
pub use templates::{render_service, render_timer};
