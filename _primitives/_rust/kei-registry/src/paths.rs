//! Path resolution for CLI flags.
//!
//! Constructor Pattern: this cube owns CLI default-path policy. Two
//! callers (handlers + scan_orchestrator) share these helpers so the
//! "where does the registry SQLite live by default" decision exists in
//! one place.

use std::path::PathBuf;

/// Resolve `--db <path>` or default to `~/.claude/registry.sqlite`.
/// Falls back to `/tmp/registry.sqlite` if `$HOME` is unset.
pub fn resolve_db(db: Option<PathBuf>) -> PathBuf {
    db.unwrap_or_else(|| {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        PathBuf::from(home).join(".claude").join("registry.sqlite")
    })
}

/// Resolve `--kit-root` or default to current directory.
pub fn resolve_kit_root(root: Option<PathBuf>) -> PathBuf {
    root.unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
}

/// Resolve `--rules-root` or default to `~/.claude/rules`.
pub fn resolve_rules_root(root: Option<PathBuf>) -> PathBuf {
    root.unwrap_or_else(|| {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        PathBuf::from(home).join(".claude").join("rules")
    })
}

/// Resolve `--hooks-root` or default to `~/.claude/hooks`.
pub fn resolve_hooks_root(root: Option<PathBuf>) -> PathBuf {
    root.unwrap_or_else(|| {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        PathBuf::from(home).join(".claude").join("hooks")
    })
}
