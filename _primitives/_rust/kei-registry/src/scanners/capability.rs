//! Capability scanner — walks `<kit-root>/_capabilities/<group>/<name>/capability.toml`.
//!
//! Constructor Pattern: this cube knows the nested `_capabilities/` layout.
//! Body = raw TOML; name = `[capability].name` from TOML, fallback = dir
//! stem; maps to BlockType::Atom; caps = category field if present.

use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use super::{Found, Scanner};
use crate::block::BlockType;

/// `<kit-root>/_capabilities/<group>/<name>/capability.toml` adapter.
pub struct CapabilityScanner;

impl Scanner for CapabilityScanner {
    fn scan(&self, root: &Path) -> Result<Vec<Found>> {
        let cap_root = root.join("_capabilities");
        if !cap_root.is_dir() {
            return Ok(Vec::new());
        }
        let mut found = Vec::new();
        for entry in WalkDir::new(&cap_root)
            .follow_links(false)
            .into_iter()
            .filter_map(Result::ok)
        {
            let path = entry.path();
            if !is_capability_toml(path) {
                continue;
            }
            if let Some(f) = scan_one_capability(path)? {
                found.push(f);
            }
        }
        found.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(found)
    }
}

fn is_capability_toml(p: &Path) -> bool {
    p.is_file()
        && p.file_name().and_then(|n| n.to_str()) == Some("capability.toml")
}

fn scan_one_capability(file: &Path) -> Result<Option<Found>> {
    let body = match fs::read(file) {
        Ok(b) => b,
        Err(_) => return Ok(None),
    };
    let fallback = dir_stem(file);
    let (name, caps) = parse_capability_toml(&body, &fallback);
    let path = canonical_str(file);
    Ok(Some(Found {
        block_type: BlockType::Atom,
        name,
        path,
        body,
        caps,
    }))
}

/// Extract `[capability].name` and `[capability].category` from TOML.
/// Returns (name, caps) — name falls back to `fallback`, caps to empty.
fn parse_capability_toml(body: &[u8], fallback: &str) -> (String, String) {
    let txt = match std::str::from_utf8(body) {
        Ok(s) => s,
        Err(_) => return (fallback.to_string(), String::new()),
    };
    let value: toml::Value = match txt.parse() {
        Ok(v) => v,
        Err(_) => return (fallback.to_string(), String::new()),
    };
    let cap_table = value.get("capability");
    let name = cap_table
        .and_then(|t| t.get("name"))
        .and_then(|v| v.as_str())
        .unwrap_or(fallback)
        .to_string();
    let caps = cap_table
        .and_then(|t| t.get("category"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    (name, caps)
}

fn dir_stem(file: &Path) -> String {
    file.parent()
        .and_then(|p| p.file_name())
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string()
}

fn canonical_str(p: &Path) -> String {
    p.canonicalize()
        .unwrap_or_else(|_| PathBuf::from(p))
        .to_string_lossy()
        .to_string()
}
