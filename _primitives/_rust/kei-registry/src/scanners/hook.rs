//! Hook scanner — walks `<hooks-root>/*.sh`.
//!
//! Constructor Pattern: this cube knows only the flat `~/.claude/hooks/`
//! directory layout. Body = raw shell script bytes; name = filename stem;
//! caps = `shell`.

use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

use super::{Found, Scanner};
use crate::block::BlockType;

/// `<hooks-root>/<name>.sh` adapter. Configurable root because hooks live
/// outside the kit repo under `~/.claude/hooks/`.
pub struct HookScanner;

impl Scanner for HookScanner {
    fn scan(&self, root: &Path) -> Result<Vec<Found>> {
        if !root.is_dir() {
            return Ok(Vec::new());
        }
        let mut found = Vec::new();
        for entry in fs::read_dir(root)? {
            let entry = entry?;
            let path = entry.path();
            if !is_sh_file(&path) {
                continue;
            }
            if let Some(f) = scan_one_hook(&path)? {
                found.push(f);
            }
        }
        found.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(found)
    }
}

fn is_sh_file(p: &Path) -> bool {
    p.is_file() && p.extension().and_then(|e| e.to_str()) == Some("sh")
}

fn scan_one_hook(file: &Path) -> Result<Option<Found>> {
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
        block_type: BlockType::Hook,
        name,
        path,
        body,
        caps: "shell".to_string(),
    }))
}

fn canonical_str(p: &Path) -> String {
    p.canonicalize()
        .unwrap_or_else(|_| PathBuf::from(p))
        .to_string_lossy()
        .to_string()
}
