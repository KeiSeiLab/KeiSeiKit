//! Block scanner — walks `<kit-root>/_blocks/*.md`.
//!
//! Constructor Pattern: this cube knows the flat `_blocks/` directory
//! convention. Body bytes = raw markdown; name = filename stem or H1;
//! maps to BlockType::Atom (atomic prompt fragment); caps = empty.

use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

use super::{Found, Scanner};
use crate::block::BlockType;

/// `<kit-root>/_blocks/<name>.md` adapter.
pub struct BlockMdScanner;

impl Scanner for BlockMdScanner {
    fn scan(&self, root: &Path) -> Result<Vec<Found>> {
        let blocks_root = root.join("_blocks");
        if !blocks_root.is_dir() {
            return Ok(Vec::new());
        }
        let mut found = Vec::new();
        for entry in fs::read_dir(&blocks_root)? {
            let entry = entry?;
            let path = entry.path();
            if !is_md_file(&path) {
                continue;
            }
            if let Some(f) = scan_one_block(&path)? {
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

fn scan_one_block(file: &Path) -> Result<Option<Found>> {
    let body = match fs::read(file) {
        Ok(b) => b,
        Err(_) => return Ok(None),
    };
    let stem = file
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();
    let name = extract_h1_title(&body).unwrap_or(stem);
    let path = canonical_str(file);
    Ok(Some(Found {
        block_type: BlockType::Atom,
        name,
        path,
        body,
        caps: String::new(),
    }))
}

fn extract_h1_title(body: &[u8]) -> Option<String> {
    let txt = std::str::from_utf8(body).ok()?;
    for line in txt.lines().take(50) {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("# ") {
            let title = rest.trim().trim_end_matches('#').trim();
            if !title.is_empty() {
                return Some(title.to_string());
            }
        }
    }
    None
}

fn canonical_str(p: &Path) -> String {
    p.canonicalize()
        .unwrap_or_else(|_| PathBuf::from(p))
        .to_string_lossy()
        .to_string()
}
