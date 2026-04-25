//! Dynamic artifact-schema whitelist loader.
//!
//! v0.16: the assembler previously hardcoded the 5 builtin schema names.
//! That blocked any user who registered a custom schema via
//! `kei-artifact register-schema` — the assembler would reject manifests
//! referencing it. This cube loads the current registry from the export
//! file written by `kei-artifact export-schemas`.
//!
//! Priority (first hit wins):
//!   1. `$AGENT_ROOT/artifacts/schemas.json` (derived from `blocks_dir.parent()`)
//!   2. `~/.claude/agents/artifacts/schemas.json`
//!   3. Built-in fallback (5 names)
//!
//! Export file format: `{"schemas": ["spec", "plan", ...]}`. Builtins are
//! always unioned in, so a hand-crafted export cannot drop a core schema.
//!
//! Constructor Pattern: no dependency on serde_json — minimal hand-parser
//! keeps the assembler lean and free of transitive deps.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// Canonical artifact schema names shipped by `kei-artifact`.
///
/// MIRROR OF `kei-artifact/src/schemas.rs::BUILTIN` (by design — assembler
/// crate must not link to the runtime primitive). Drift is detected by the
/// `builtin_schemas_do_not_drift` test in `validator.rs`.
pub const BUILTIN: &[&str] = &["spec", "plan", "patch", "review", "research"];

/// Union of builtins + any names found in an on-disk export, as a sorted set.
pub fn load(blocks_dir: &Path) -> BTreeSet<String> {
    load_with_home(blocks_dir, std::env::var("HOME").ok().as_deref())
}

/// Test-friendly variant that accepts an explicit HOME override.
pub fn load_with_home(blocks_dir: &Path, home: Option<&str>) -> BTreeSet<String> {
    let mut out: BTreeSet<String> = BUILTIN.iter().map(|s| (*s).to_string()).collect();
    for path in candidate_paths(blocks_dir, home) {
        if let Some(names) = read_export(&path) {
            out.extend(names);
            break;
        }
    }
    out
}

fn candidate_paths(blocks_dir: &Path, home: Option<&str>) -> Vec<PathBuf> {
    let mut v = Vec::new();
    if let Some(root) = blocks_dir.parent() {
        v.push(root.join("artifacts/schemas.json"));
    }
    if let Some(h) = home {
        v.push(PathBuf::from(h).join(".claude/agents/artifacts/schemas.json"));
    }
    v
}

fn read_export(path: &Path) -> Option<Vec<String>> {
    let text = std::fs::read_to_string(path).ok()?;
    parse_export(&text)
}

/// Minimal parser for `{"schemas": ["a", "b"]}`. Tolerant of whitespace.
pub fn parse_export(text: &str) -> Option<Vec<String>> {
    let body = text.trim();
    let key = "\"schemas\"";
    let i = body.find(key)?;
    let rest = &body[i + key.len()..].trim_start_matches(|c: char| c == ':' || c.is_whitespace());
    let open = rest.find('[')?;
    let close = rest[open..].find(']')?;
    let inner = &rest[open + 1..open + close];
    let mut names = Vec::new();
    for tok in inner.split(',') {
        let t = tok.trim().trim_matches('"').trim();
        if !t.is_empty() {
            names.push(t.to_string());
        }
    }
    Some(names)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_happy_path() {
        let body = r#"{"schemas": ["spec", "plan", "custom-one"]}"#;
        assert_eq!(parse_export(body).unwrap(), vec!["spec", "plan", "custom-one"]);
    }

    #[test]
    fn parse_whitespace_and_newlines() {
        let body = "{\n  \"schemas\" : [\n    \"a\",\n    \"b\"\n  ]\n}\n";
        assert_eq!(parse_export(body).unwrap(), vec!["a", "b"]);
    }

    #[test]
    fn parse_rejects_malformed() {
        assert!(parse_export("{}").is_none());
        assert!(parse_export(r#"{"schemas":"spec"}"#).is_none());
    }

    #[test]
    fn load_falls_back_to_builtin_when_no_export() {
        let tmp = tempfile::tempdir().unwrap();
        let blocks_dir = tmp.path().join("_blocks");
        std::fs::create_dir_all(&blocks_dir).unwrap();
        // Isolated HOME (under tmp) — no real export file at that path.
        let home = tmp.path().to_string_lossy().to_string();
        let known = load_with_home(&blocks_dir, Some(&home));
        for s in BUILTIN {
            assert!(known.contains(*s));
        }
        assert_eq!(known.len(), BUILTIN.len());
    }

    #[test]
    fn load_unions_with_custom_export() {
        let tmp = tempfile::tempdir().unwrap();
        let blocks_dir = tmp.path().join("_blocks");
        std::fs::create_dir_all(&blocks_dir).unwrap();
        let export = tmp.path().join("artifacts/schemas.json");
        std::fs::create_dir_all(export.parent().unwrap()).unwrap();
        std::fs::write(
            &export,
            r#"{"schemas": ["spec", "plan", "patch", "review", "research", "runbook"]}"#,
        )
        .unwrap();
        let known = load_with_home(&blocks_dir, None);
        assert!(known.contains("runbook"));
        for s in BUILTIN {
            assert!(known.contains(*s));
        }
    }
}
