//! # kei-backend-daytona
//!
//! Daytona serverless backend with hibernation support — HERMES-MIGRATION P1.2.
//!
//! Why a separate crate:
//! - Hermes uses the Daytona Python SDK; no Rust SDK exists upstream, so we
//!   hit the public REST API directly.
//! - The Constructor Pattern keeps each layer (client / types / backend /
//!   file-sync) in its own ≤200-LOC cube.
//!
//! ## Cost note
//!
//! Daytona's free tier covers **2 concurrent sandboxes** with **30-min idle
//! hibernate**. Anything past that is paid. Before any production launch
//! plug this backend into `kei-cost-guardian` so we never invent a sandbox
//! we cannot afford.
//!
//! ## Quick start
//!
//! ```ignore
//! use kei_backend_daytona::{DaytonaClient, DaytonaBackend, Backend};
//!
//! # async fn ex() -> kei_backend_daytona::Result<()> {
//! let client = DaytonaClient::new(
//!     std::env::var("DAYTONA_API_KEY").unwrap(),
//!     "https://app.daytona.io/api",
//! )?;
//! let backend = DaytonaBackend::new(client, "ubuntu:24.04");
//! let handle = backend.acquire("task-42").await?;
//! let out = backend.exec(&handle, "echo hi").await?;
//! assert_eq!(out.exit_code, 0);
//! backend.release(handle, /* persist */ true).await?;
//! # Ok(())
//! # }
//! ```

pub mod backend;
pub mod backend_sync;
pub mod client;
pub mod cost_guard;
pub mod error;
pub mod file_sync;
pub mod toolbox;
pub mod types;

pub use backend::{Backend, DaytonaBackend, SandboxHandle, SyncConfig};
pub use client::DaytonaClient;
pub use cost_guard::{pre_create_check, CostGuardError, FREE_TIER_CAP};
pub use error::{DaytonaError, Result};
pub use file_sync::{FileSync, SyncState};
pub use types::{
    CreateSandboxSpec, ExecOutput, Resources, Sandbox, SandboxState,
};
