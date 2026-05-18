//! `read` tool — file retrieval with line numbers.
//!
//! Composition: validate input → reject relative / `..` / outside
//! `project_root` / blocked-basename → read file → render `cat -n`-style
//! output. No new I/O logic — standard `tokio::fs` plus `path_sandbox`.
//!
//! Sandbox guarantees:
//!   - rejects relative paths and any path containing `..`
//!   - rejects paths that resolve OUTSIDE `project_root` (canonicalised)
//!   - rejects sensitive basenames (`.env`, `id_rsa*`, `*.pem`, …)
//!   - rejects `~/.zshrc`-class dotfiles even via project-root symlink
//!   - rejects non-utf8 file contents (binary returns an error message)
//!   - rejects files larger than `MAX_BYTES` (10 MiB)

use super::path_sandbox;
use super::types::ToolError;
use serde::Deserialize;
use serde_json::Value;
use std::path::Path;

/// Hard cap on file size returned to the model.
const MAX_BYTES: u64 = 10 * 1024 * 1024;

/// Default line limit when caller does not specify one.
const DEFAULT_LIMIT: usize = 2000;

#[derive(Debug, Deserialize)]
struct Input {
    path: String,
    #[serde(default)]
    offset: Option<usize>,
    #[serde(default)]
    limit: Option<usize>,
}

pub async fn run(raw: Value, project_root: &Path) -> Result<String, ToolError> {
    let input: Input = serde_json::from_value(raw)
        .map_err(|e| ToolError::InvalidInput(e.to_string()))?;
    validate_path_lexical(&input.path)?;
    let canon = path_sandbox::check_all(&input.path, project_root)?;
    let meta = tokio::fs::metadata(&canon).await?;
    if meta.len() > MAX_BYTES {
        return Err(ToolError::TooLarge(format!(
            "{} bytes (cap {})",
            meta.len(),
            MAX_BYTES
        )));
    }
    let bytes = tokio::fs::read(&canon).await?;
    let text = String::from_utf8(bytes)
        .map_err(|_| ToolError::InvalidInput("file is not valid UTF-8".into()))?;
    Ok(render(&text, input.offset, input.limit))
}

/// Lexical-only path checks (cheap pre-filter before canonicalisation).
/// Reject relative paths, parent traversal, and empty paths.
///
pub(crate) fn validate_path_lexical(path: &str) -> Result<(), ToolError> {
    if path.is_empty() {
        return Err(ToolError::InvalidInput("empty path".into()));
    }
    if !path.starts_with('/') {
        return Err(ToolError::PathDenied(format!("not absolute: {path}")));
    }
    if path.split('/').any(|seg| seg == "..") {
        return Err(ToolError::PathDenied(format!("contains '..': {path}")));
    }
    Ok(())
}

/// Render lines `cat -n`-style, honouring offset/limit windowing.
fn render(text: &str, offset: Option<usize>, limit: Option<usize>) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let start = offset.unwrap_or(1).max(1).saturating_sub(1);
    let take = limit.unwrap_or(DEFAULT_LIMIT);
    let end = (start + take).min(lines.len());
    let mut out = String::new();
    for (i, line) in lines[start..end].iter().enumerate() {
        out.push_str(&format!("{:>6}\t{}\n", start + i + 1, line));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn rejects_relative_path() {
        assert!(matches!(
            validate_path_lexical("relative/file"),
            Err(ToolError::PathDenied(_))
        ));
    }

    #[test]
    fn rejects_parent_traversal() {
        assert!(matches!(
            validate_path_lexical("/etc/../passwd"),
            Err(ToolError::PathDenied(_))
        ));
    }

    #[test]
    fn accepts_clean_absolute() {
        assert!(validate_path_lexical("/tmp/hello.txt").is_ok());
    }

    #[test]
    fn render_numbers_lines() {
        let out = render("a\nb\nc", None, None);
        assert!(out.contains("     1\ta"));
        assert!(out.contains("     3\tc"));
    }

    #[tokio::test]
    async fn rejects_path_outside_project_root() {
        let dir = tempdir().unwrap();
        let raw = serde_json::json!({"path": "/etc/hosts"});
        let res = run(raw, dir.path()).await;
        assert!(matches!(res, Err(ToolError::OutsideRoot(_))));
    }

    #[tokio::test]
    async fn rejects_dotenv_inside_project_root() {
        let dir = tempdir().unwrap();
        let envp = dir.path().join(".env");
        tokio::fs::write(&envp, "SECRET=1").await.unwrap();
        let raw = serde_json::json!({"path": envp.to_str().unwrap()});
        let res = run(raw, dir.path()).await;
        assert!(matches!(res, Err(ToolError::PathDenied(_))));
    }

    #[tokio::test]
    async fn accepts_path_inside_project_root() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("hello.txt");
        tokio::fs::write(&p, "alpha\nbeta\n").await.unwrap();
        let raw = serde_json::json!({"path": p.to_str().unwrap()});
        let out = run(raw, dir.path()).await.unwrap();
        assert!(out.contains("alpha"));
    }
}
