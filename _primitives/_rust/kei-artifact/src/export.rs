//! v0.16: schema-registry export.
//!
//! Writes the current list of registered schema names as JSON at a path the
//! assembler's manifest validator reads to accept custom-registered schemas
//! without a rebuild.
//!
//! Format: `{"schemas": ["spec", "plan", ...]}` with a trailing newline.
//!
//! Constructor Pattern: one cube, one responsibility. Tests live inline —
//! `render()` is pure, so we exercise it without a Store.

use crate::artifact::list_schemas;
use crate::store::Store;
use anyhow::Result;
use std::path::{Path, PathBuf};

/// Write the current schemas registry to `override_path` or the default
/// umbrella path. Returns the number of schemas written + the final path.
pub fn write(store: &Store, override_path: Option<&Path>) -> Result<(usize, PathBuf)> {
    let names = list_schemas(store)?;
    let json = render(&names);
    let target = override_path
        .map(Path::to_path_buf)
        .unwrap_or_else(default_path);
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&target, json)?;
    Ok((names.len(), target))
}

/// Serialize `names` as `{"schemas": ["a", "b"]}\n`.
pub fn render(names: &[String]) -> String {
    let quoted: Vec<String> = names.iter().map(|n| format!("\"{n}\"")).collect();
    format!("{{\"schemas\": [{}]}}\n", quoted.join(", "))
}

/// `~/.claude/agents/artifacts/schemas.json` (consumed by the assembler).
pub fn default_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".claude/agents/artifacts/schemas.json")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_empty_list() {
        assert_eq!(render(&[]), "{\"schemas\": []}\n");
    }

    #[test]
    fn render_five_builtins() {
        let names: Vec<String> =
            ["spec", "plan", "patch", "review", "research"].iter().map(|s| s.to_string()).collect();
        assert_eq!(
            render(&names),
            "{\"schemas\": [\"spec\", \"plan\", \"patch\", \"review\", \"research\"]}\n"
        );
    }

    #[test]
    fn write_creates_file_and_parent_dir() {
        let tmp = tempfile::tempdir().unwrap();
        let store = Store::open_memory().unwrap();
        crate::schemas::register_builtins(&store).unwrap();
        crate::artifact::register_schema(
            &store,
            "custom",
            r#"{"type":"object","additionalProperties":false,"properties":{}}"#,
        )
        .unwrap();
        let target = tmp.path().join("nested/dir/schemas.json");
        let (n, path) = write(&store, Some(&target)).unwrap();
        assert_eq!(n, 6);
        assert_eq!(path, target);
        let body = std::fs::read_to_string(&target).unwrap();
        assert!(body.contains("custom"));
        assert!(body.contains("spec"));
    }
}
