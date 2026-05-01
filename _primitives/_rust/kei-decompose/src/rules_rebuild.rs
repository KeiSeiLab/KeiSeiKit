//! `decompose-rules --rebuild-fragments` one-time migration helper.
//!
//! Re-extracts fragment bodies for all active Rule-type registry rows that
//! were registered with the old `"<file>::<section>"` logical-key format.
//! Writes each body to `<frags_dir>/<rule>__<section>.md` and updates the
//! `path` column in the registry to point at the real filesystem path.
//!
//! Constructor Pattern: this cube owns the rebuild loop only.
//! Registry open + list lives in kei_registry. Fragment writing is reused
//! from `rules_cmd::write_fragment_file` and `rules_cmd::fragment_path`.

use anyhow::{Context, Result};
use std::path::Path;

use kei_registry::{open_db, Block, BlockType};

use crate::parsers::parse_rule_file;
use crate::rules_cmd::{ensure_dir, fragment_path, write_fragment_file};

/// Re-extract all active Rule-type rows to canonical fragment files and update
/// their `path` column. Returns count of updated rows.
pub fn run(db_path: &Path, frags_dir: &Path, dry_run: bool) -> anyhow::Result<usize> {
    let conn = open_db(db_path).with_context(|| format!("open registry: {}", db_path.display()))?;
    let blocks = kei_registry::list_by_type(&conn, BlockType::Rule)
        .context("list rule blocks")?;
    if !dry_run {
        ensure_dir(frags_dir)?;
    }
    let mut updated = 0usize;
    for block in &blocks {
        match rebuild_one(&conn, block, frags_dir, dry_run) {
            Ok(true) => updated += 1,
            Ok(false) => {}
            Err(e) => eprintln!("warn: rebuild {} — {e}", block.name),
        }
    }
    Ok(updated)
}

/// Rebuild one block from its legacy `"<file>::<section>"` path.
fn rebuild_one(
    conn: &rusqlite::Connection,
    block: &Block,
    frags_dir: &Path,
    dry_run: bool,
) -> Result<bool> {
    let (source_path_str, section_slug) = split_legacy_path(&block.path)?;
    let rule_slug = extract_rule_slug(source_path_str);
    let frags = parse_rule_file(Path::new(source_path_str))
        .with_context(|| format!("re-parse {source_path_str}"))?;
    let frag = frags
        .iter()
        .find(|f| f.section_slug == section_slug)
        .with_context(|| format!("section '{section_slug}' not in {source_path_str}"))?;
    let real_path = fragment_path(frags_dir, &rule_slug, &frag.section_slug);
    if dry_run {
        println!("[dry-run] would write {}", real_path.display());
        println!("[dry-run] would update path for {}", block.name);
        return Ok(true);
    }
    write_fragment_file(&real_path, &frag.body)?;
    conn.execute(
        "UPDATE blocks SET path = ?1 WHERE id = ?2",
        rusqlite::params![real_path.to_str().unwrap_or_default(), block.id],
    )
    .with_context(|| format!("update path for {}", block.name))?;
    Ok(true)
}

/// Split `"<file_path>::<section>"` into its two components.
/// Returns `Err` when no `::` separator is found (path is already canonical).
fn split_legacy_path(path: &str) -> Result<(&str, &str)> {
    if let Some(idx) = path.rfind("::") {
        let file = &path[..idx];
        let section = &path[idx + 2..];
        Ok((file, section))
    } else {
        anyhow::bail!("path has no '::' separator, already canonical")
    }
}

fn extract_rule_slug(source_path: &str) -> String {
    Path::new(source_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string()
}
