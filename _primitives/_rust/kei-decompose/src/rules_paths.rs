//! Path resolution helpers for `decompose-rules` subcommand.
//!
//! Resolves optional CLI arguments to concrete filesystem paths, with
//! defaults derived from `HOME` and `KEI_FRAGMENTS_DIR` env vars.
//!
//! Constructor Pattern: path resolution is one responsibility.

use std::path::PathBuf;

pub fn resolve_rules_dir(opt: Option<PathBuf>) -> PathBuf {
    opt.unwrap_or_else(|| expand_home("~/.claude/rules"))
}

pub fn resolve_db_path(opt: Option<PathBuf>) -> PathBuf {
    opt.unwrap_or_else(|| expand_home("~/.claude/registry.sqlite"))
}

pub fn resolve_fragments_dir(opt: Option<PathBuf>) -> PathBuf {
    if let Some(p) = opt { return p; }
    if let Ok(v) = std::env::var("KEI_FRAGMENTS_DIR") {
        if !v.is_empty() { return PathBuf::from(v); }
    }
    expand_home("~/.claude/registry-fragments")
}

pub fn expand_home(path: &str) -> PathBuf {
    if let Some(rest) = path.strip_prefix("~/") {
        if let Some(home) = std::env::var_os("HOME") {
            return PathBuf::from(home).join(rest);
        }
    }
    PathBuf::from(path)
}
