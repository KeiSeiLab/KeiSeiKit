//! `glob` tool — file discovery by glob pattern.
//!
//! Composition: walkdir over the search root → match each entry against
//! the pattern (translated to a `Regex`) → sort by mtime descending →
//! cap at 100 results. Returns one path per line.
//!
//! Sandbox: when an absolute `path` is supplied, it must resolve INSIDE
//! `project_root` (canonicalised). When omitted, the search root is
//! `project_root` itself.
//!
//! Pattern syntax: standard shell globs — `*` matches any chars except
//! `/`, `**` matches any chars including `/`, `?` matches any single
//! char, `[abc]` matches a character class.

use super::path_sandbox;
use super::read::validate_path_lexical;
use super::types::ToolError;
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use walkdir::WalkDir;

const MAX_RESULTS: usize = 100;

#[derive(Debug, Deserialize)]
struct Input {
    pattern: String,
    #[serde(default)]
    path: Option<String>,
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
    let regex = compile_glob(&input.pattern)?;
    let root_str = root.to_string_lossy().to_string();
    let matches = tokio::task::spawn_blocking(move || collect_matches(&root_str, &regex))
        .await
        .map_err(|e| ToolError::Internal(format!("walk join: {e}")))?;
    Ok(matches.join("\n"))
}

/// Walk `root`, return up to `MAX_RESULTS` paths matching `regex`,
/// sorted by mtime descending.
fn collect_matches(root: &str, regex: &Regex) -> Vec<String> {
    let mut hits: Vec<(SystemTime, PathBuf)> = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| {
            let p = e.path().to_string_lossy().to_string();
            if regex.is_match(&p) {
                let mtime = e.metadata().ok().and_then(|m| m.modified().ok())
                    .unwrap_or(SystemTime::UNIX_EPOCH);
                Some((mtime, e.into_path()))
            } else {
                None
            }
        })
        .collect();
    hits.sort_by(|a, b| b.0.cmp(&a.0));
    hits.truncate(MAX_RESULTS);
    hits.into_iter()
        .map(|(_, p)| p.to_string_lossy().to_string())
        .collect()
}

/// Translate a glob into an anchored regex.
pub(crate) fn compile_glob(pattern: &str) -> Result<Regex, ToolError> {
    let mut out = String::from("^");
    let chars: Vec<char> = pattern.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        match c {
            '*' => {
                if i + 1 < chars.len() && chars[i + 1] == '*' {
                    out.push_str(".*");
                    i += 1;
                } else {
                    out.push_str("[^/]*");
                }
            }
            '?' => out.push_str("[^/]"),
            '.' | '+' | '(' | ')' | '|' | '^' | '$' | '{' | '}' | '\\' => {
                out.push('\\');
                out.push(c);
            }
            '[' | ']' => out.push(c),
            _ => out.push(c),
        }
        i += 1;
    }
    out.push('$');
    Regex::new(&out).map_err(|e| ToolError::InvalidInput(format!("invalid glob: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn glob_star_matches_one_segment() {
        let re = compile_glob("*.rs").unwrap();
        assert!(re.is_match("foo.rs"));
        assert!(!re.is_match("dir/foo.rs"));
    }

    #[test]
    fn glob_double_star_matches_dirs() {
        let re = compile_glob("**/*.rs").unwrap();
        assert!(re.is_match("a/b/c.rs"));
        assert!(re.is_match("/x.rs"));
    }

    #[test]
    fn glob_question_one_char() {
        let re = compile_glob("a?c").unwrap();
        assert!(re.is_match("abc"));
        assert!(!re.is_match("ac"));
        assert!(!re.is_match("abbc"));
    }

    #[tokio::test]
    async fn run_finds_file_inside_project_root() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("hit.txt");
        tokio::fs::write(&path, "x").await.unwrap();
        let raw = serde_json::json!({"pattern": "**/hit.txt"});
        let out = run(raw, dir.path()).await.unwrap();
        assert!(out.contains("hit.txt"));
    }

    #[tokio::test]
    async fn run_rejects_path_outside_project_root() {
        let dir = tempdir().unwrap();
        let raw = serde_json::json!({
            "pattern": "*.txt",
            "path": "/tmp",
        });
        let res = run(raw, dir.path()).await;
        assert!(matches!(res, Err(ToolError::OutsideRoot(_))));
    }
}
