//! Atomic file write — tempfile-in-same-dir + rename.
//!
//! Composition: shared by `write.rs` and `edit.rs` (and mirrored by
//! `handlers/tool_apply.rs` until that cube refactors to import this).
//! The same-directory rename is atomic on POSIX and Windows, so partial
//! writes never appear at the destination path.
//!
//! Constructor Pattern: one fn, no state, ≤30 LOC active body. The
//! tempfile name encodes a nanosecond timestamp so concurrent writes to
//! the same destination collide deterministically (last-rename-wins) and
//! never overwrite each other's staging files.

use super::types::ToolError;
use std::path::Path;

/// Stage `bytes` to `<dir>/<basename>.<nanos>.tmp` then rename onto
/// `dest`. Caller is responsible for `create_dir_all(parent)` first.
pub async fn atomic_write(dest: &Path, bytes: &[u8]) -> Result<(), ToolError> {
    let parent = dest.parent().unwrap_or_else(|| Path::new("."));
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let suffix = format!(".{}.tmp", nanos);
    let file_name = dest
        .file_name()
        .ok_or_else(|| ToolError::InvalidInput("path has no filename".into()))?;
    let mut staged_name = file_name.to_owned();
    staged_name.push(&suffix);
    let staging = parent.join(staged_name);
    tokio::fs::write(&staging, bytes).await?;
    tokio::fs::rename(&staging, dest).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn writes_then_reads_back() {
        let dir = tempdir().unwrap();
        let dest = dir.path().join("hello.txt");
        atomic_write(&dest, b"world").await.unwrap();
        let s = tokio::fs::read_to_string(&dest).await.unwrap();
        assert_eq!(s, "world");
    }

    #[tokio::test]
    async fn overwrites_existing() {
        let dir = tempdir().unwrap();
        let dest = dir.path().join("h.txt");
        tokio::fs::write(&dest, b"old").await.unwrap();
        atomic_write(&dest, b"new").await.unwrap();
        let s = tokio::fs::read_to_string(&dest).await.unwrap();
        assert_eq!(s, "new");
    }

    #[tokio::test]
    async fn rejects_filename_only_path() {
        let res = atomic_write(Path::new("/"), b"x").await;
        assert!(matches!(res, Err(ToolError::InvalidInput(_))));
    }
}
