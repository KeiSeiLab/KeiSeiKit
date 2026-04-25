//! `grep` tool — regex search over files.
//!
//! Composition: walkdir over root → optional glob filter → regex match →
//! emit either `files_with_matches` (one path per line) or `content`
//! (`path:line_no:line_text`). Limits: 1000 lines, 100 files.
//!
//! Sandbox: when an absolute `path` is supplied, it must resolve INSIDE
//! `project_root` (canonicalised). When omitted, the search root is
//! `project_root` itself.
//!
//! Uses the workspace `regex` dep (no PCRE, ASCII-flavoured Rust regex).

use super::glob_tool::compile_glob;
use super::path_sandbox;
use super::read::validate_path_lexical;
use super::types::ToolError;
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const MAX_LINES: usize = 1000;
const MAX_FILES: usize = 100;

#[derive(Debug, Deserialize)]
struct Input {
    pattern: String,
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    glob: Option<String>,
    #[serde(default)]
    output_mode: Option<String>,
}

pub async fn run(raw: Value, project_root: &Path) -> Result<String, ToolError> {
    let input: Input = serde_json::from_value(raw)
        .map_err(|e| ToolError::InvalidInput(e.to_string()))?;
    let root: PathBuf = match input.path.as_deref() {
        Some(p) if p.starts_with('/') => {
            validate_path_lexical(p)?;
            path_sandbox::enforce_project_root(p, project_root)?
        }
        Some(_) | None => project_root.to_path_buf(),
    };
    let needle = Regex::new(&input.pattern)
        .map_err(|e| ToolError::InvalidInput(format!("invalid regex: {e}")))?;
    let glob_re = match input.glob.as_deref() {
        Some(g) => Some(compile_glob(g)?),
        None => None,
    };
    let mode = input
        .output_mode
        .unwrap_or_else(|| "files_with_matches".to_string());
    let root_str = root.to_string_lossy().to_string();
    tokio::task::spawn_blocking(move || scan(&root_str, &needle, glob_re.as_ref(), &mode))
        .await
        .map_err(|e| ToolError::Internal(format!("scan join: {e}")))
}

/// Walk and dispatch to the requested output mode.
fn scan(root: &str, needle: &Regex, glob_re: Option<&Regex>, mode: &str) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut total_lines = 0usize;
    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path().to_string_lossy().to_string();
        if glob_re.map_or(false, |g| !g.is_match(&path)) {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(entry.path()) else {
            continue;
        };
        if !scan_one_file(&text, &path, needle, mode, &mut out, &mut total_lines) {
            return out.join("\n");
        }
    }
    out.join("\n")
}

/// Scan one file's text, mutating `out`/`total_lines`. Returns false when
/// the global cap was reached and the caller should stop walking.
fn scan_one_file(
    text: &str,
    path: &str,
    needle: &Regex,
    mode: &str,
    out: &mut Vec<String>,
    total_lines: &mut usize,
) -> bool {
    if mode == "content" {
        for (i, line) in text.lines().enumerate() {
            if needle.is_match(line) {
                out.push(format!("{}:{}:{}", path, i + 1, line));
                *total_lines += 1;
                if *total_lines >= MAX_LINES {
                    return false;
                }
            }
        }
    } else if text.lines().any(|l| needle.is_match(l)) {
        out.push(path.to_string());
        if out.len() >= MAX_FILES {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn grep_finds_in_project_root() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("a.txt");
        tokio::fs::write(&path, "alpha\nbeta\ngamma").await.unwrap();
        let raw = serde_json::json!({
            "pattern": "beta",
            "output_mode": "content",
        });
        let out = run(raw, dir.path()).await.unwrap();
        assert!(out.contains("a.txt:2:beta"));
    }

    #[tokio::test]
    async fn grep_files_mode_only_path() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("b.txt");
        tokio::fs::write(&path, "hit\nmiss").await.unwrap();
        let raw = serde_json::json!({"pattern": "hit"});
        let out = run(raw, dir.path()).await.unwrap();
        assert!(out.contains("b.txt"));
        assert!(!out.contains(":1:hit"));
    }

    #[tokio::test]
    async fn grep_invalid_regex_errors() {
        let dir = tempdir().unwrap();
        let raw = serde_json::json!({"pattern": "[unclosed"});
        let err = run(raw, dir.path()).await.unwrap_err();
        assert!(matches!(err, ToolError::InvalidInput(_)));
    }

    #[tokio::test]
    async fn grep_rejects_path_outside_project_root() {
        let dir = tempdir().unwrap();
        let raw = serde_json::json!({
            "pattern": "x",
            "path": "/tmp",
        });
        let res = run(raw, dir.path()).await;
        assert!(matches!(res, Err(ToolError::OutsideRoot(_))));
    }
}
