//! FilesystemStore — local `.git` repo, no remotes.
//!
//! Reuses git2 for branch/commit so behavior parity with remote stores is
//! maintained. `push`/`pull` are intentional no-ops.
//!
//! v0.14.1 hardening: `full()` now rejects absolute paths and `..` components
//! (CVE-class: path traversal via MCP `write`/`read` tool inputs).

use crate::store_trait::MemoryStore;
use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Component, Path, PathBuf};

pub struct FilesystemStore {
    pub root: PathBuf,
}

impl FilesystemStore {
    pub fn new(root: impl Into<PathBuf>) -> Result<Self> {
        let root = root.into();
        fs::create_dir_all(&root).with_context(|| format!("mkdir {}", root.display()))?;
        ensure_repo(&root)?;
        Ok(Self { root })
    }

    fn full(&self, rel: &str) -> Result<PathBuf> {
        safe_join(&self.root, rel)
    }
}

/// Reject absolute paths and any `..` component BEFORE joining.
/// `PathBuf::join("/etc/passwd")` would otherwise replace the base
/// entirely — that turned kei-store's MCP `write` tool into an
/// unrestricted filesystem writer.
pub(crate) fn safe_join(root: &Path, rel: &str) -> Result<PathBuf> {
    let p = Path::new(rel);
    if p.is_absolute() {
        bail!("path traversal rejected: absolute path {:?}", rel);
    }
    for component in p.components() {
        match component {
            Component::ParentDir => {
                bail!("path traversal rejected: parent-dir component in {:?}", rel);
            }
            Component::Prefix(_) | Component::RootDir => {
                bail!("path traversal rejected: root/prefix component in {:?}", rel);
            }
            _ => {}
        }
    }
    Ok(root.join(rel))
}

fn ensure_repo(root: &Path) -> Result<()> {
    if root.join(".git").exists() {
        return Ok(());
    }
    git2::Repository::init(root).context("git init")?;
    Ok(())
}

impl MemoryStore for FilesystemStore {
    fn read(&self, path: &str) -> Result<Vec<u8>> {
        fs::read(self.full(path)?).with_context(|| format!("read {}", path))
    }

    fn write(&self, path: &str, bytes: &[u8]) -> Result<()> {
        let full = self.full(path)?;
        if let Some(parent) = full.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&full, bytes).with_context(|| format!("write {}", path))
    }

    fn list(&self, dir: &str) -> Result<Vec<String>> {
        let full = self.full(dir)?;
        if !full.exists() {
            return Ok(Vec::new());
        }
        let mut out = Vec::new();
        for e in fs::read_dir(&full)? {
            let e = e?;
            if e.file_type()?.is_file() {
                if let Some(name) = e.file_name().to_str() {
                    out.push(name.to_string());
                }
            }
        }
        out.sort();
        Ok(out)
    }

    fn branch(&self, name: &str) -> Result<()> {
        let repo = git2::Repository::open(&self.root)?;
        if repo.find_branch(name, git2::BranchType::Local).is_ok() {
            return Ok(());
        }
        if let Ok(head) = repo.head().and_then(|h| h.peel_to_commit()) {
            repo.branch(name, &head, false)?;
        }
        // If there is no HEAD yet (empty repo), silently no-op; first commit
        // will be on default branch.
        Ok(())
    }

    fn commit(&self, message: &str) -> Result<String> {
        let repo = git2::Repository::open(&self.root)?;
        let mut index = repo.index()?;
        index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;
        let tree_oid = index.write_tree()?;
        let tree = repo.find_tree(tree_oid)?;
        let sig = git2::Signature::now("kei-store", "kei-store@local")?;
        let parent = repo.head().ok().and_then(|h| h.peel_to_commit().ok());
        let parents: Vec<&git2::Commit> = parent.iter().collect();
        let oid = repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &parents)?;
        Ok(oid.to_string())
    }

    fn push(&self, _branch: &str) -> Result<()> {
        Ok(())
    }

    fn pull(&self, _branch: &str) -> Result<()> {
        Ok(())
    }

    fn backend_name(&self) -> &'static str {
        "filesystem"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_absolute_path_rejected() {
        let tmp = tempfile::tempdir().unwrap();
        let store = FilesystemStore::new(tmp.path().join("root")).unwrap();
        let err = store.write("/etc/passwd", b"nope").unwrap_err();
        let s = format!("{err:#}");
        assert!(s.contains("absolute"), "unexpected err: {s}");
    }

    #[test]
    fn test_parent_dir_rejected() {
        let tmp = tempfile::tempdir().unwrap();
        let store = FilesystemStore::new(tmp.path().join("root")).unwrap();
        let err = store.write("../../.ssh/authorized_keys", b"nope").unwrap_err();
        let s = format!("{err:#}");
        assert!(s.contains("parent-dir"), "unexpected err: {s}");
    }

    #[test]
    fn test_normal_path_ok() {
        let tmp = tempfile::tempdir().unwrap();
        let store = FilesystemStore::new(tmp.path().join("root")).unwrap();
        store.write("traces/session.jsonl", b"ok").unwrap();
        let bytes = store.read("traces/session.jsonl").unwrap();
        assert_eq!(&bytes, b"ok");
    }

    #[test]
    fn test_read_absolute_path_rejected() {
        let tmp = tempfile::tempdir().unwrap();
        let store = FilesystemStore::new(tmp.path().join("root")).unwrap();
        let err = store.read("/etc/passwd").unwrap_err();
        let s = format!("{err:#}");
        assert!(s.contains("absolute"), "unexpected err: {s}");
    }
}
