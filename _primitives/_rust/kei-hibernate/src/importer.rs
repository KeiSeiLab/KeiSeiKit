//! Import side — verify manifest + extract (or dry-run preview).
//!
//! Two-pass design:
//!   Pass 1 — read manifest entry only, decide version + list conflicts.
//!   Pass 2 — if `dry_run=false`, re-stream archive and extract each
//!            file, then re-hash to verify sha256 against the manifest.
//!
//! Safe extraction: each entry path is checked for `..` traversal.

use crate::error::Error;
use crate::manifest::{HibernateManifest, MANIFEST_FILENAME, MANIFEST_VERSION};
use crate::sha;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Component, Path, PathBuf};
use tar::Archive;

#[derive(Debug, Clone)]
pub struct ImportReport {
    pub file_count: usize,
    pub conflicts: Vec<String>,
    pub extracted: usize,
    pub dry_run: bool,
    pub bundle_timestamp: i64,
    pub bundle_machine_id: String,
}

/// Entry point for the `import` CLI + library users.
pub fn import(bundle: &Path, kit_root: &Path, dry_run: bool) -> Result<ImportReport, Error> {
    let manifest = read_manifest(bundle)?;
    enforce_version(&manifest)?;
    let conflicts = list_conflicts(&manifest, kit_root);
    let extracted = if dry_run { 0 } else { extract_and_verify(bundle, kit_root, &manifest)? };
    Ok(ImportReport {
        file_count: manifest.entries.len(),
        conflicts,
        extracted,
        dry_run,
        bundle_timestamp: manifest.timestamp,
        bundle_machine_id: manifest.machine_id,
    })
}

/// Open bundle, locate manifest entry, decode TOML.
pub(crate) fn read_manifest(bundle: &Path) -> Result<HibernateManifest, Error> {
    let mut archive = open_archive(bundle)?;
    for entry in archive.entries().map_err(Error::Plain)? {
        let mut entry = entry.map_err(Error::Plain)?;
        let path = entry.path().map_err(Error::Plain)?.to_path_buf();
        if path.as_os_str() == MANIFEST_FILENAME {
            let mut buf = String::new();
            entry.read_to_string(&mut buf).map_err(Error::Plain)?;
            return Ok(HibernateManifest::from_toml(&buf)?);
        }
    }
    Err(Error::ManifestMissing(MANIFEST_FILENAME))
}

/// Open a fresh zstd-wrapped tar archive stream over `bundle`.
fn open_archive(bundle: &Path) -> Result<Archive<zstd::Decoder<'static, std::io::BufReader<File>>>, Error> {
    let file = File::open(bundle).map_err(|source| Error::Io {
        path: bundle.to_path_buf(),
        source,
    })?;
    let dec = zstd::Decoder::new(file).map_err(Error::Plain)?;
    Ok(Archive::new(dec))
}

fn enforce_version(m: &HibernateManifest) -> Result<(), Error> {
    if m.version != MANIFEST_VERSION {
        return Err(Error::VersionMismatch {
            bundle: m.version.clone(),
            primitive: MANIFEST_VERSION.to_string(),
        });
    }
    Ok(())
}

/// Files that would be overwritten on a real import (reporting only).
fn list_conflicts(m: &HibernateManifest, kit_root: &Path) -> Vec<String> {
    m.entries
        .iter()
        .filter(|e| kit_root.join(&e.path).exists())
        .map(|e| e.path.clone())
        .collect()
}

/// Re-open archive, extract every non-manifest entry, verify sha256.
fn extract_and_verify(bundle: &Path, kit_root: &Path, m: &HibernateManifest) -> Result<usize, Error> {
    std::fs::create_dir_all(kit_root).map_err(|source| Error::Io {
        path: kit_root.to_path_buf(),
        source,
    })?;
    let mut archive = open_archive(bundle)?;
    let mut count = 0usize;
    for entry in archive.entries().map_err(Error::Plain)? {
        let mut entry = entry.map_err(Error::Plain)?;
        let rel = entry_rel_path(&entry)?;
        if rel == MANIFEST_FILENAME {
            continue;
        }
        let target = safe_join(kit_root, &rel)?;
        ensure_parent(&target)?;
        entry.unpack(&target).map_err(Error::Plain)?;
        verify_entry(&target, &rel, m)?;
        count += 1;
    }
    let _ = seek_reset(bundle);
    Ok(count)
}

/// Extract the forward-slash relative path from a tar entry header.
fn entry_rel_path<R: Read>(entry: &tar::Entry<'_, R>) -> Result<String, Error> {
    let path = entry.path().map_err(Error::Plain)?.to_path_buf();
    Ok(path.to_string_lossy().replace('\\', "/"))
}

/// Reject any entry whose path contains `..` or is absolute.
fn safe_join(root: &Path, rel: &str) -> Result<PathBuf, Error> {
    let rel_p = Path::new(rel);
    for comp in rel_p.components() {
        match comp {
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(Error::UnsafeEntryPath(rel.to_string()));
            }
            _ => {}
        }
    }
    Ok(root.join(rel_p))
}

fn ensure_parent(target: &Path) -> Result<(), Error> {
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent).map_err(|source| Error::Io {
            path: parent.to_path_buf(),
            source,
        })?;
    }
    Ok(())
}

/// Hash extracted file, compare to manifest entry sha256.
fn verify_entry(target: &Path, rel: &str, m: &HibernateManifest) -> Result<(), Error> {
    let expected = match m.lookup(rel) {
        Some(e) => &e.sha256,
        None => return Ok(()),
    };
    let actual = sha::hash_file(target).map_err(|source| Error::Io {
        path: target.to_path_buf(),
        source,
    })?;
    if &actual != expected {
        return Err(Error::ShaMismatch {
            path: rel.to_string(),
            expected: expected.clone(),
            actual,
        });
    }
    Ok(())
}

/// Reset-to-start helper used for debugging / future streaming passes.
/// Best-effort; not load-bearing — errors ignored by caller.
fn seek_reset(bundle: &Path) -> std::io::Result<()> {
    let mut f = File::open(bundle)?;
    f.seek(SeekFrom::Start(0))?;
    Ok(())
}
