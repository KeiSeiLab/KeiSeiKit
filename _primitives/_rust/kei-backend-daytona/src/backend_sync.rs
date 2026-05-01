//! Push/pull helpers used by `DaytonaBackend`'s lifecycle hooks.
//!
//! Kept in a separate cube so `backend.rs` stays under the 200-LOC
//! Constructor-Pattern limit. The functions here accept the pieces they
//! need explicitly — they do NOT borrow `DaytonaBackend` — so they can be
//! unit-tested without the full backend assembled.
//!
//! Sync errors are logged via `eprintln!` and swallowed; the lifecycle
//! does not abort because of a sync failure (the sandbox is still usable
//! and a later retry can resync).

use crate::backend::{SandboxHandle, SyncConfig};
use crate::client::DaytonaClient;
use crate::error::Result;
use crate::file_sync::FileSync;

/// Sentinel file pulled back from the sandbox on `release(persist=true)`.
///
/// Bulk-tree pull (full directory walk) is tracked as a follow-up. Pulling
/// a single marker file is sufficient to acknowledge the sandbox produced
/// state and lets the local side decide whether to do a deeper sync.
const PULL_SENTINEL: &str = ".keiseikit-state";

/// Push the configured local tree into the sandbox.
///
/// No-op when `cfg` is `None`. Returns `Ok(())` even when the underlying
/// `FileSync::push` fails — the error is logged to stderr.
pub async fn push_if_configured(
    client: &DaytonaClient,
    handle: &SandboxHandle,
    cfg: Option<&SyncConfig>,
) -> Result<()> {
    let Some(cfg) = cfg else {
        return Ok(());
    };
    let mut sync = FileSync::new(
        client,
        handle,
        cfg.local_root.clone(),
        cfg.remote_root.clone(),
    );
    if let Err(e) = sync.push().await {
        eprintln!(
            "kei-backend-daytona: push to sandbox {} failed: {e}",
            handle.name
        );
    }
    Ok(())
}

/// Pull the sentinel marker file back from the sandbox.
///
/// No-op when `cfg` is `None`. Errors are logged but do not propagate —
/// release semantics are dominated by the stop/delete call, not the pull.
pub async fn pull_if_configured(
    client: &DaytonaClient,
    handle: &SandboxHandle,
    cfg: Option<&SyncConfig>,
) {
    let Some(cfg) = cfg else {
        return;
    };
    let sync = FileSync::new(
        client,
        handle,
        cfg.local_root.clone(),
        cfg.remote_root.clone(),
    );
    if let Err(e) = sync.pull(PULL_SENTINEL).await {
        eprintln!(
            "kei-backend-daytona: pull from sandbox {} failed: {e}",
            handle.name
        );
    }
}
