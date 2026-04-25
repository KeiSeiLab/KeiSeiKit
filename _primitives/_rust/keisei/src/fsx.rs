//! Filesystem helpers shared across adapters.
//!
//! Constructor Pattern: single responsibility — own the write-then-rename
//! pattern. Every adapter shares the exact same crash-safe write,
//! regardless of extension.
//!
//! Uses `tempfile::NamedTempFile::persist` so:
//!   - on Windows, a locked target no longer leaks a stale `.tmp` file
//!     (the temp file is cleaned up on drop if `persist` failed);
//!   - on crash mid-write, the original target is preserved intact;
//!   - cross-filesystem persist gracefully falls back to copy-then-remove
//!     via `tempfile`'s own logic.

use crate::error::Result;
use std::io::Write;
use std::path::Path;

/// Atomic write. Temp file lives in the target's parent dir, then is
/// persisted (renamed) onto the target. Uses `tempfile::NamedTempFile`
/// under the hood.
pub fn write_atomic(target: &Path, content: &str) -> Result<()> {
    let parent = target
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    std::fs::create_dir_all(parent)?;
    let mut tmp = tempfile::NamedTempFile::new_in(parent)?;
    tmp.write_all(content.as_bytes())?;
    tmp.flush()?;
    tmp.persist(target).map_err(|e| e.error)?;
    Ok(())
}

/// Convenience: serialize a `serde_json::Value` as pretty JSON and
/// atomically write it. Every adapter that targets a JSON file uses
/// this — keeps the serialization shape identical across adapters.
pub fn write_atomic_json(target: &Path, doc: &serde_json::Value) -> Result<()> {
    let text = serde_json::to_string_pretty(doc)?;
    write_atomic(target, &text)
}
