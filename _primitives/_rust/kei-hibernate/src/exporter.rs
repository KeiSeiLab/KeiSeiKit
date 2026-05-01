//! Export side — build a `tar.zst` bundle + manifest.
//!
//! Pipeline:
//!   collector::collect → sha::hash_file per entry → tar append →
//!   manifest append last → zstd compress to `out`.
//!
//! Manifest is written LAST so all file hashes are already known.
//! Single-pass compression: bundle stays small even on large stores.

use crate::collector::{self, Found};
use crate::error::Error;
use crate::manifest::{HibernateManifest, ManifestEntry, MANIFEST_FILENAME};
use crate::sha;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct ExportMeta {
    pub file_count: usize,
    pub total_bytes: u64,
    pub machine_id: String,
    pub timestamp: i64,
}

/// Export the KeiSei installation rooted at `kit_root` into `out` (tar.zst).
///
/// Orchestration only: delegates manifest build to `build_manifest`
/// and archive write to `write_archive`.
pub fn export(kit_root: &Path, out: &Path) -> Result<ExportMeta, Error> {
    if !kit_root.is_dir() {
        return Err(Error::KitRootInvalid(kit_root.to_path_buf()));
    }
    let found = collector::collect(kit_root).map_err(Error::Plain)?;
    let manifest = build_manifest(&found)?;
    let total_bytes = manifest.entries.iter().map(|e| e.size).sum();
    write_archive(out, &found, &manifest)?;
    Ok(ExportMeta {
        file_count: manifest.entries.len(),
        total_bytes,
        machine_id: manifest.machine_id.clone(),
        timestamp: manifest.timestamp,
    })
}

/// Hash every collected file and assemble the manifest. One loop,
/// one error-context join per missing path.
fn build_manifest(found: &[Found]) -> Result<HibernateManifest, Error> {
    let mut entries = Vec::with_capacity(found.len());
    for f in found {
        let sha256 = sha::hash_file(&f.abs).map_err(|source| Error::Io {
            path: f.abs.clone(),
            source,
        })?;
        let size = std::fs::metadata(&f.abs)
            .map_err(|source| Error::Io { path: f.abs.clone(), source })?
            .len();
        entries.push(ManifestEntry { path: f.rel.clone(), sha256, size });
    }
    Ok(HibernateManifest::new(now_epoch(), machine_id(), entries))
}

/// Open `out`, wrap in zstd encoder, tar-append each file, then the
/// manifest blob. `finish()` on both writers is mandatory.
fn write_archive(out: &Path, found: &[Found], m: &HibernateManifest) -> Result<(), Error> {
    let file = File::create(out).map_err(|source| Error::Io { path: out.to_path_buf(), source })?;
    let zstd_enc = zstd::Encoder::new(file, 3).map_err(Error::Plain)?.auto_finish();
    let mut tar_builder = tar::Builder::new(zstd_enc);
    for f in found {
        tar_builder
            .append_path_with_name(&f.abs, &f.rel)
            .map_err(|source| Error::Io { path: f.abs.clone(), source })?;
    }
    append_manifest(&mut tar_builder, m)?;
    tar_builder.finish().map_err(Error::Plain)?;
    Ok(())
}

/// Serialise manifest to TOML and append as a tar entry (no fs temp file).
fn append_manifest<W: Write>(
    builder: &mut tar::Builder<W>,
    m: &HibernateManifest,
) -> Result<(), Error> {
    let toml_str = m.to_toml()?;
    let bytes = toml_str.as_bytes();
    let mut header = tar::Header::new_gnu();
    header.set_size(bytes.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();
    builder
        .append_data(&mut header, MANIFEST_FILENAME, bytes)
        .map_err(Error::Plain)?;
    Ok(())
}

/// Best-effort machine identifier. `HOSTNAME` env first, hostname
/// crate avoided to keep dep footprint minimal.
fn machine_id() -> String {
    std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("HOST"))
        .unwrap_or_else(|_| "unknown-host".to_string())
}

fn now_epoch() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}
