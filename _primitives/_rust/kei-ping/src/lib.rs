// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! `kei-ping` — cross-window agent heartbeat. Auto-selects backend.
//!
//! Constructor Pattern: 1 trait + 2 impl-cubes (sqlite / redis) + 1 dispatcher.
//! Each cube ≤200 LOC, 1 responsibility.

pub mod model;
pub mod sqlite_store;
pub mod redis_store;
pub mod store;

pub use model::{Heartbeat, PingFilter};
pub use store::{auto_select, BackendKind, PingStore};
