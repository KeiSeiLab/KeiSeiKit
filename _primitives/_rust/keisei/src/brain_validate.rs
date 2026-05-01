//! Validation primitives for `Brain::load`.
//!
//! Constructor Pattern: single responsibility — own the five mechanical
//! checks (symlink reject / root canonicalize / manifest read / name
//! regex / in-root path guard). `brain.rs` composes them into the load
//! pipeline. No cross-module state; every fn is pure w.r.t. filesystem.

use crate::brain::{BrainManifest, MANIFEST_FILENAME, MAX_SCHEMA, MIN_SCHEMA};
use crate::error::{Error, Result};
use regex::Regex;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

fn name_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^[a-z][a-z0-9_-]{0,63}$").expect("valid regex"))
}

pub fn reject_symlink_root(input: &Path) -> Result<()> {
    match std::fs::symlink_metadata(input) {
        Ok(md) if md.file_type().is_symlink() => {
            let target = std::fs::read_link(input).unwrap_or_else(|_| PathBuf::from("?"));
            Err(Error::BrainIsSymlink {
                input: input.to_path_buf(),
                target,
            })
        }
        // Missing-path → let canonicalize produce the final error message.
        _ => Ok(()),
    }
}

pub fn canonicalize_root(input: &Path) -> Result<PathBuf> {
    input.canonicalize().map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            Error::BrainNotFound(input.to_path_buf())
        } else {
            Error::BrainLoad {
                path: input.to_path_buf(),
                source: e,
            }
        }
    })
}

/// L12 (v0.19.2 audit): cap manifest.toml at 64 KiB. A well-formed brain
/// manifest is ~1 KB; anything larger is either corruption or an attempt
/// to exhaust memory by feeding an enormous file through the toml parser.
pub const MAX_MANIFEST_BYTES: u64 = 64 * 1024;

pub fn read_manifest(root: &Path) -> Result<BrainManifest> {
    let mpath = root.join(MANIFEST_FILENAME);
    if !mpath.is_file() {
        return Err(Error::BrainNotFound(mpath));
    }
    let meta = std::fs::metadata(&mpath)?;
    if meta.len() > MAX_MANIFEST_BYTES {
        return Err(Error::ManifestTooLarge {
            size: meta.len(),
            max: MAX_MANIFEST_BYTES,
        });
    }
    let raw = std::fs::read_to_string(&mpath)?;
    let manifest: BrainManifest = toml::from_str(&raw)?;
    Ok(manifest)
}

pub fn validate_schema(manifest: &BrainManifest) -> Result<()> {
    let v = manifest.brain.schema_version;
    if !(MIN_SCHEMA..=MAX_SCHEMA).contains(&v) {
        return Err(Error::UnsupportedSchema { found: v });
    }
    Ok(())
}

pub fn validate_name(name: &str) -> Result<()> {
    if name_regex().is_match(name) {
        Ok(())
    } else {
        Err(Error::InvalidName(name.to_string()))
    }
}

/// Syntactic check before touching disk: absolute path or `..` component
/// → `PathEscape`. Filters obvious attacks without requiring the target
/// to exist.
pub fn check_relative_in_root(rel: &str) -> Result<()> {
    let p = Path::new(rel);
    if p.is_absolute() {
        return Err(Error::PathEscape(p.to_path_buf()));
    }
    for comp in p.components() {
        if matches!(comp, std::path::Component::ParentDir) {
            return Err(Error::PathEscape(p.to_path_buf()));
        }
    }
    Ok(())
}

/// Resolve + canonicalize a manifest-declared relative path and assert it
/// lives under `root`. Called only on paths that already passed
/// `check_relative_in_root`.
pub fn canonicalize_in_root(root: &Path, rel: &str) -> Result<PathBuf> {
    let joined = root.join(rel);
    let canonical = joined.canonicalize().map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            Error::BrainNotFound(joined.clone())
        } else {
            Error::BrainLoad {
                path: joined.clone(),
                source: e,
            }
        }
    })?;
    if !canonical.starts_with(root) {
        return Err(Error::PathEscape(canonical));
    }
    Ok(canonical)
}
