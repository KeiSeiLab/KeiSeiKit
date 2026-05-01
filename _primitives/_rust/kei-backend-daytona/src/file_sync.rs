//! Push/pull `~/.keiseikit` to/from a sandbox.
//!
//! Hermes uses bulk multipart uploads and a tar-stream for downloads; we
//! ship a simpler per-file path. Bulk transports are tracked as P1.2.x
//! follow-ups. Deltas are computed via mtime comparison: a file syncs only
//! if local-mtime differs from the recorded last-sync mtime.

use crate::backend::SandboxHandle;
use crate::client::DaytonaClient;
use crate::error::{DaytonaError, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Tracks last-known mtime per file so we don't push unchanged files.
#[derive(Debug, Default, Clone)]
pub struct SyncState {
    /// Map of relative path → last-synced mtime as nanos-since-epoch.
    seen: HashMap<String, u128>,
}

impl SyncState {
    pub fn new() -> Self {
        Self::default()
    }

    /// True if `path` mtime differs from previously-seen value.
    pub fn is_dirty(&self, rel: &str, mtime_nanos: u128) -> bool {
        self.seen.get(rel).copied() != Some(mtime_nanos)
    }

    /// Mark a path as synced at the given mtime.
    pub fn mark(&mut self, rel: &str, mtime_nanos: u128) {
        self.seen.insert(rel.to_string(), mtime_nanos);
    }
}

/// Bidirectional sync for a single sandbox handle.
pub struct FileSync<'a> {
    client: &'a DaytonaClient,
    handle: &'a SandboxHandle,
    /// Local root (e.g. `~/.keiseikit`).
    local_root: PathBuf,
    /// Remote root path (e.g. `/root/.keiseikit`).
    remote_root: String,
    state: SyncState,
}

impl<'a> FileSync<'a> {
    pub fn new(
        client: &'a DaytonaClient,
        handle: &'a SandboxHandle,
        local_root: PathBuf,
        remote_root: impl Into<String>,
    ) -> Self {
        Self {
            client,
            handle,
            local_root,
            remote_root: remote_root.into(),
            state: SyncState::new(),
        }
    }

    /// Push every file under `local_root` whose mtime has changed since the
    /// last successful push. Returns the count of files actually uploaded.
    pub async fn push(&mut self) -> Result<usize> {
        let mut pushed = 0usize;
        let entries = collect_files(&self.local_root)?;
        for (abs, rel) in entries {
            let mtime = mtime_nanos(&abs)?;
            if !self.state.is_dirty(&rel, mtime) {
                continue;
            }
            let body = fs::read(&abs).map_err(|e| DaytonaError::Unknown(e.to_string()))?;
            let remote = format!("{}/{}", self.remote_root, rel);
            self.client.upload_file(&self.handle.name, &remote, body).await?;
            self.state.mark(&rel, mtime);
            pushed += 1;
        }
        Ok(pushed)
    }

    /// Pull a single remote file back to the local tree.
    pub async fn pull(&self, rel: &str) -> Result<()> {
        let remote = format!("{}/{}", self.remote_root, rel);
        let bytes = self.client.download_file(&self.handle.name, &remote).await?;
        let local = self.local_root.join(rel);
        if let Some(parent) = local.parent() {
            fs::create_dir_all(parent).map_err(|e| DaytonaError::Unknown(e.to_string()))?;
        }
        fs::write(&local, &bytes).map_err(|e| DaytonaError::Unknown(e.to_string()))?;
        Ok(())
    }

    /// Inspect sync state (for tests / observability).
    pub fn state(&self) -> &SyncState {
        &self.state
    }
}

/// Walk `root` recursively and return `(abs_path, relative_path)` pairs.
fn collect_files(root: &Path) -> Result<Vec<(PathBuf, String)>> {
    let mut out = Vec::new();
    if !root.exists() {
        return Ok(out);
    }
    walk(root, root, &mut out)?;
    Ok(out)
}

fn walk(root: &Path, dir: &Path, out: &mut Vec<(PathBuf, String)>) -> Result<()> {
    let rd = fs::read_dir(dir).map_err(|e| DaytonaError::Unknown(e.to_string()))?;
    for entry in rd {
        let entry = entry.map_err(|e| DaytonaError::Unknown(e.to_string()))?;
        let path = entry.path();
        let ft = entry.file_type().map_err(|e| DaytonaError::Unknown(e.to_string()))?;
        if ft.is_dir() {
            walk(root, &path, out)?;
        } else if ft.is_file() {
            let rel = path
                .strip_prefix(root)
                .map_err(|e| DaytonaError::Unknown(e.to_string()))?;
            out.push((path.clone(), rel.to_string_lossy().to_string()));
        }
    }
    Ok(())
}

/// mtime expressed as nanos since UNIX epoch; 0 if filesystem doesn't expose it.
fn mtime_nanos(path: &Path) -> Result<u128> {
    let meta = fs::metadata(path).map_err(|e| DaytonaError::Unknown(e.to_string()))?;
    let mtime = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    let dur = mtime
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    Ok(dur.as_nanos())
}
