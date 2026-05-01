//! `write` tool — atomic file write.
//!
//! Composition: validate path → enforce `project_root` chroot + basename
//! deny → ensure parent dir → atomic_write to staging tempfile + rename.
//! Same-directory rename is atomic on POSIX and Windows so partial writes
//! never appear.
//!
//! Sandbox: lexical pre-check (`validate_path_lexical`) +
//! `path_sandbox::check_all` (chroot + basename + home-rc). The legacy
//! `deny_system_dirs` substring check stays as a Layer-3 belt-and-
//! suspenders for the system-dir corner cases.

use super::atomic_io::atomic_write;
use super::path_sandbox;
use super::read::validate_path_lexical;
use super::types::ToolError;
use serde::Deserialize;
use serde_json::Value;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Input {
    path: String,
    content: String,
}

pub async fn run(raw: Value, project_root: &Path) -> Result<String, ToolError> {
    let input: Input = serde_json::from_value(raw)
        .map_err(|e| ToolError::InvalidInput(e.to_string()))?;
    validate_path_lexical(&input.path)?;
    deny_system_dirs(&input.path)?;
    let canon = path_sandbox::check_all(&input.path, project_root)?;
    if let Some(parent) = canon.parent() {
        if !parent.as_os_str().is_empty() {
            tokio::fs::create_dir_all(parent).await?;
        }
    }
    atomic_write(&canon, input.content.as_bytes()).await?;
    Ok(format!(
        "wrote {} bytes to {}",
        input.content.len(),
        input.path
    ))
}

/// Reject writes to root-level system directories. Belt-and-suspenders
/// alongside `path_sandbox::check_all`; catches the `/etc/x` case
/// before canonicalisation fails noisily.
pub(crate) fn deny_system_dirs(path: &str) -> Result<(), ToolError> {
    // Whitelist macOS / Linux temp dirs FIRST — they live under `/var/...`
    // canonically (or `/private/var/...` on macOS) and `tempfile::tempdir`
    // tests must succeed. Without this, ALL Rust integration tests using
    // tempdir fail with "system dir: /var/folders/...".
    const TEMP_WHITELIST: &[&str] = &[
        "/var/folders/", "/private/var/folders/", "/tmp/", "/private/tmp/",
    ];
    for ok in TEMP_WHITELIST {
        if path.starts_with(ok) {
            return Ok(());
        }
    }
    const FORBIDDEN: &[&str] = &[
        "/etc/", "/var/", "/usr/", "/bin/", "/sbin/", "/boot/",
        "/private/etc/", "/private/var/",
        "/System/", "/Library/LaunchDaemons/",
    ];
    for prefix in FORBIDDEN {
        if path.starts_with(prefix) {
            return Err(ToolError::PathDenied(format!("system dir: {path}")));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn deny_system_dirs_blocks_etc() {
        assert!(matches!(
            deny_system_dirs("/etc/passwd"),
            Err(ToolError::PathDenied(_))
        ));
    }

    #[test]
    fn deny_system_dirs_blocks_private_etc() {
        assert!(matches!(
            deny_system_dirs("/private/etc/passwd"),
            Err(ToolError::PathDenied(_))
        ));
    }

    #[test]
    fn deny_system_dirs_allows_tmp() {
        assert!(deny_system_dirs("/tmp/file").is_ok());
    }

    #[tokio::test]
    async fn run_rejects_system_dir() {
        let dir = tempdir().unwrap();
        let raw = serde_json::json!({"path": "/etc/x", "content": "y"});
        let err = run(raw, dir.path()).await.unwrap_err();
        assert!(matches!(err, ToolError::PathDenied(_)));
    }

    #[tokio::test]
    async fn run_rejects_path_outside_project_root() {
        let dir = tempdir().unwrap();
        let raw = serde_json::json!({"path": "/tmp/elsewhere.txt", "content": "x"});
        let err = run(raw, dir.path()).await.unwrap_err();
        assert!(matches!(err, ToolError::OutsideRoot(_)));
    }

    #[tokio::test]
    async fn run_rejects_dotenv_inside_project_root() {
        let dir = tempdir().unwrap();
        let envp = dir.path().join(".env");
        let raw = serde_json::json!({"path": envp.to_str().unwrap(), "content": "x"});
        let err = run(raw, dir.path()).await.unwrap_err();
        assert!(matches!(err, ToolError::PathDenied(_)));
    }

    #[tokio::test]
    async fn run_writes_inside_project_root() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("note.txt");
        let raw = serde_json::json!({
            "path": p.to_str().unwrap(),
            "content": "hello",
        });
        let msg = run(raw, dir.path()).await.unwrap();
        assert!(msg.contains("wrote 5 bytes"));
        let s = tokio::fs::read_to_string(&p).await.unwrap();
        assert_eq!(s, "hello");
    }
}
