//! Obsidian-style vault import: walk a directory, ingest .md files.
//!
//! Minimal subset of LBM internal/sage/import_obsidian.go — we do NOT parse
//! frontmatter here (the upstream parser used multiple helper files). Port
//! of frontmatter/wikilinks parsing is a later milestone; this cube honours
//! the public interface.

use crate::store::Store;
use crate::types::Unit;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub struct ImportStats {
    pub imported: usize,
    pub skipped: usize,
}

pub fn import_vault(store: &Store, root: &Path) -> Result<ImportStats> {
    let mut stats = ImportStats { imported: 0, skipped: 0 };
    let files = walk_md(root)?;
    for path in files {
        match ingest_one(store, root, &path) {
            Ok(_) => stats.imported += 1,
            Err(_) => stats.skipped += 1,
        }
    }
    Ok(stats)
}

fn walk_md(root: &Path) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    walk_recursive(root, &mut out)?;
    Ok(out)
}

fn walk_recursive(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in fs::read_dir(dir).with_context(|| format!("read_dir {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            walk_recursive(&path, out)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("md") {
            out.push(path);
        }
    }
    Ok(())
}

fn ingest_one(store: &Store, root: &Path, path: &Path) -> Result<()> {
    let content = fs::read_to_string(path)?;
    let title = path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("untitled")
        .to_string();
    let vault_path = path.strip_prefix(root)
        .ok()
        .and_then(|p| p.to_str())
        .unwrap_or(&title)
        .to_string();
    let unit = Unit {
        unit_type: "note".into(),
        title,
        content,
        evidence_grade: "E4".into(),
        source_path: path.to_string_lossy().into(),
        vault_path,
        category: String::new(),
        ..Default::default()
    };
    store.add_unit(&unit)?;
    Ok(())
}
