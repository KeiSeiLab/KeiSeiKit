//! Rule scanner — walks `<rules-root>/*.md`.
//!
//! Constructor Pattern: this cube knows only the flat `~/.claude/rules/`
//! directory layout. Body = raw markdown; name = filename stem; caps = `md`.

use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

use super::{Found, Scanner};
use crate::block::BlockType;

/// `<rules-root>/<name>.md` adapter. The rules root is configurable via
/// CLI flag because rules live OUTSIDE the kit repository (they are a
/// user-global concern under `~/.claude/`).
pub struct RuleScanner;

impl Scanner for RuleScanner {
    fn scan(&self, root: &Path) -> Result<Vec<Found>> {
        if !root.is_dir() {
            return Ok(Vec::new());
        }
        let mut found = Vec::new();
        for entry in fs::read_dir(root)? {
            let entry = entry?;
            let path = entry.path();
            if !is_md_file(&path) {
                continue;
            }
            if let Some(f) = scan_one_rule(&path)? {
                found.push(f);
            }
        }
        found.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(found)
    }
}

fn is_md_file(p: &Path) -> bool {
    p.is_file() && p.extension().and_then(|e| e.to_str()) == Some("md")
}

fn scan_one_rule(file: &Path) -> Result<Option<Found>> {
    let body = match fs::read(file) {
        Ok(b) => b,
        Err(_) => return Ok(None),
    };
    let name = file
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();
    let path = canonical_str(file);
    Ok(Some(Found {
        block_type: BlockType::Rule,
        name,
        path,
        body,
        caps: "md".to_string(),
    }))
}

fn canonical_str(p: &Path) -> String {
    p.canonicalize()
        .unwrap_or_else(|_| PathBuf::from(p))
        .to_string_lossy()
        .to_string()
}
