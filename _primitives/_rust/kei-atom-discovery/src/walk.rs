//! Filesystem walk for atom discovery.
//!
//! `discover_atoms` enumerates `<root>/*/atoms/*.md` with `follow_links(false)`.
//! Malformed files emit a stderr warning and are dropped (skip-on-invalid).

use crate::error::Error;
use crate::frontmatter::{
    parse_frontmatter, parse_side_effects, AtomKind, AtomMeta, Frontmatter,
};
use crate::path_safety::safe_join;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use walkdir::WalkDir;

/// Walk `<root>/*/atoms/*.md`. Skip-on-invalid: malformed files emit a
/// stderr warning and are dropped. Never follows symlinks.
pub fn discover_atoms(root: &Path) -> Vec<AtomMeta> {
    let mut out = Vec::new();
    for entry in WalkDir::new(root)
        .max_depth(3)
        .follow_links(false)
        .into_iter()
        .flatten()
    {
        if !is_atom_md(entry.path()) {
            continue;
        }
        match parse_one(entry.path()) {
            Ok(meta) => out.push(meta),
            Err(e) => eprintln!("warn: skip {}: {}", entry.path().display(), e),
        }
    }
    out
}

fn is_atom_md(path: &Path) -> bool {
    path.is_file()
        && path.extension().and_then(|s| s.to_str()) == Some("md")
        && path
            .parent()
            .and_then(|p| p.file_name())
            .is_some_and(|n| n == "atoms")
}

fn parse_one(md_path: &Path) -> Result<AtomMeta, Error> {
    let text = std::fs::read_to_string(md_path)?;
    let (fm_text, body) = parse_frontmatter(&text)?;
    let fm: Frontmatter = serde_yaml_ng::from_str(fm_text)?;
    build_meta(fm, body, md_path)
}

fn build_meta(fm: Frontmatter, body: &str, md_path: &Path) -> Result<AtomMeta, Error> {
    let kind = AtomKind::from_str(&fm.kind)?;
    let (crate_name, verb) = split_atom_id(&fm.atom)?;
    let md_dir = md_path.parent().unwrap_or(md_path);
    let input_schema = resolve_opt_schema(md_dir, fm.input.as_ref().and_then(|s| s.schema.as_deref()));
    let output_schema =
        resolve_opt_schema(md_dir, fm.output.as_ref().and_then(|s| s.schema.as_deref()));
    Ok(AtomMeta {
        full_id: fm.atom.clone(),
        crate_name,
        verb,
        kind,
        version: fm.version.unwrap_or_default(),
        md_path: md_path.to_path_buf(),
        input_schema,
        output_schema,
        side_effects: parse_side_effects(&fm.side_effects),
        idempotent: fm.idempotent.unwrap_or(false),
        stability: fm.stability.unwrap_or_else(|| "unknown".into()),
        keywords: fm.keywords,
        related: fm.related,
        body: body.to_string(),
        taxonomy: fm.taxonomy,
        lineage: fm.lineage,
    })
}

/// Resolve an optional schema path relative to the atom's directory.
/// Silently drops entries that fail `safe_join` — lint catches them separately.
fn resolve_opt_schema(md_dir: &Path, rel: Option<&str>) -> Option<PathBuf> {
    rel.and_then(|r| safe_join(md_dir, r).ok())
}

/// Split `<crate>::<verb>` atom id into components.
pub fn split_atom_id(id: &str) -> Result<(String, String), Error> {
    match id.split_once("::") {
        Some((c, v)) if !c.is_empty() && !v.is_empty() => Ok((c.into(), v.into())),
        _ => Err(Error::BadAtomId(id.to_string())),
    }
}
