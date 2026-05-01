//! `edit` tool — string replacement with uniqueness check + atomic write.
//!
//! Composition: read existing file → verify `old_string` occurrence count
//! → replace → atomic_write back via shared `atomic_io::atomic_write`. No
//! new I/O logic — composes `read` and `write` cube primitives.
//!
//! Semantics: when `replace_all = false` (default), `old_string` MUST
//! match exactly once. When `replace_all = true`, every occurrence is
//! replaced and the count is reported in the success message.
//!
//! Sandbox: same lexical + chroot + basename rules as `write.rs`.

use super::atomic_io::atomic_write;
use super::path_sandbox;
use super::read::validate_path_lexical;
use super::types::ToolError;
use super::write::deny_system_dirs;
use serde::Deserialize;
use serde_json::Value;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Input {
    path: String,
    old_string: String,
    new_string: String,
    #[serde(default)]
    replace_all: bool,
}

pub async fn run(raw: Value, project_root: &Path) -> Result<String, ToolError> {
    let input: Input = serde_json::from_value(raw)
        .map_err(|e| ToolError::InvalidInput(e.to_string()))?;
    validate_path_lexical(&input.path)?;
    deny_system_dirs(&input.path)?;
    let canon = path_sandbox::check_all(&input.path, project_root)?;
    if input.old_string.is_empty() {
        return Err(ToolError::InvalidInput("old_string is empty".into()));
    }
    if input.old_string == input.new_string {
        return Err(ToolError::InvalidInput(
            "old_string equals new_string".into(),
        ));
    }
    let original = tokio::fs::read_to_string(&canon).await?;
    let count = count_occurrences(&original, &input.old_string);
    let replaced = perform_replace(&original, &input, count)?;
    atomic_write(&canon, replaced.as_bytes()).await?;
    Ok(format!(
        "{} replacement(s) in {}",
        if input.replace_all { count } else { 1 },
        input.path
    ))
}

/// Count non-overlapping occurrences of `needle` in `hay`.
pub(crate) fn count_occurrences(hay: &str, needle: &str) -> usize {
    if needle.is_empty() {
        return 0;
    }
    let mut n = 0usize;
    let mut start = 0usize;
    while let Some(idx) = hay[start..].find(needle) {
        n += 1;
        start += idx + needle.len();
    }
    n
}

/// Apply the replacement, enforcing the unique-match rule when needed.
fn perform_replace(original: &str, input: &Input, count: usize) -> Result<String, ToolError> {
    if count == 0 {
        return Err(ToolError::NotUnique(format!(
            "old_string not found in {}",
            input.path
        )));
    }
    if !input.replace_all && count > 1 {
        return Err(ToolError::NotUnique(format!(
            "old_string matched {count} times; pass replace_all=true or add more context"
        )));
    }
    if input.replace_all {
        Ok(original.replace(&input.old_string, &input.new_string))
    } else {
        Ok(original.replacen(&input.old_string, &input.new_string, 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn count_finds_three() {
        assert_eq!(count_occurrences("aaabaaab", "aaab"), 2);
        assert_eq!(count_occurrences("foo bar foo bar foo", "foo"), 3);
    }

    #[test]
    fn count_empty_needle_zero() {
        assert_eq!(count_occurrences("hi", ""), 0);
    }

    #[tokio::test]
    async fn duplicate_old_string_without_replace_all_fails() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("x.txt");
        tokio::fs::write(&path, "foo foo").await.unwrap();
        let raw = serde_json::json!({
            "path": path.to_str().unwrap(),
            "old_string": "foo",
            "new_string": "bar",
        });
        let err = run(raw, dir.path()).await.unwrap_err();
        assert!(matches!(err, ToolError::NotUnique(_)));
    }

    #[tokio::test]
    async fn unique_replace_succeeds_atomically() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("y.txt");
        tokio::fs::write(&path, "alpha beta gamma").await.unwrap();
        let raw = serde_json::json!({
            "path": path.to_str().unwrap(),
            "old_string": "beta",
            "new_string": "BETA",
        });
        let msg = run(raw, dir.path()).await.unwrap();
        assert!(msg.contains("1 replacement"));
        let after = tokio::fs::read_to_string(&path).await.unwrap();
        assert_eq!(after, "alpha BETA gamma");
    }

    #[tokio::test]
    async fn replace_all_changes_every_occurrence() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("z.txt");
        tokio::fs::write(&path, "foo foo foo").await.unwrap();
        let raw = serde_json::json!({
            "path": path.to_str().unwrap(),
            "old_string": "foo",
            "new_string": "bar",
            "replace_all": true,
        });
        let msg = run(raw, dir.path()).await.unwrap();
        assert!(msg.contains("3 replacement"));
    }

    #[tokio::test]
    async fn rejects_path_outside_project_root() {
        let dir = tempdir().unwrap();
        let raw = serde_json::json!({
            "path": "/tmp/anything.txt",
            "old_string": "a",
            "new_string": "b",
        });
        let err = run(raw, dir.path()).await.unwrap_err();
        assert!(matches!(err, ToolError::OutsideRoot(_)));
    }
}
