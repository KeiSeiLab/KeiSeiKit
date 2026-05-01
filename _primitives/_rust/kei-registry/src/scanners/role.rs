//! Role scanner — walks `<kit-root>/_roles/*.toml`.
//!
//! Constructor Pattern: this cube knows the flat `_roles/` directory
//! convention. Body = raw TOML; name = filename stem; maps to
//! BlockType::Atom; caps = empty.

use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

use super::{Found, Scanner};
use crate::block::BlockType;

/// `<kit-root>/_roles/<name>.toml` adapter.
pub struct RoleScanner;

impl Scanner for RoleScanner {
    fn scan(&self, root: &Path) -> Result<Vec<Found>> {
        let roles_root = root.join("_roles");
        if !roles_root.is_dir() {
            return Ok(Vec::new());
        }
        let mut found = Vec::new();
        for entry in fs::read_dir(&roles_root)? {
            let entry = entry?;
            let path = entry.path();
            if !is_toml_file(&path) {
                continue;
            }
            if let Some(f) = scan_one_role(&path)? {
                found.push(f);
            }
        }
        found.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(found)
    }
}

fn is_toml_file(p: &Path) -> bool {
    p.is_file() && p.extension().and_then(|e| e.to_str()) == Some("toml")
}

fn scan_one_role(file: &Path) -> Result<Option<Found>> {
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
        block_type: BlockType::Atom,
        name,
        path,
        body,
        caps: String::new(),
    }))
}

fn canonical_str(p: &Path) -> String {
    p.canonicalize()
        .unwrap_or_else(|_| PathBuf::from(p))
        .to_string_lossy()
        .to_string()
}
