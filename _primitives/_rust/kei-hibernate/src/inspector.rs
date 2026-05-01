//! Inspector — read-only preview of bundle contents.
//!
//! Streams the archive, counts non-manifest entries, and returns
//! paths from the manifest (pre-computed, order-preserving). No
//! extraction, no side effects.

use crate::error::Error;
use crate::importer::read_manifest;
use crate::manifest::MANIFEST_FILENAME;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct InspectReport {
    pub version: String,
    pub timestamp: i64,
    pub machine_id: String,
    pub file_count: usize,
    pub total_bytes: u64,
    pub paths: Vec<String>,
}

/// List bundle contents without extracting. Rejects missing manifest
/// (same invariant as `import`). `MANIFEST_FILENAME` itself is not
/// included in the reported list.
pub fn inspect(bundle: &Path) -> Result<InspectReport, Error> {
    let manifest = read_manifest(bundle)?;
    let paths: Vec<String> = manifest
        .entries
        .iter()
        .map(|e| e.path.clone())
        .filter(|p| p != MANIFEST_FILENAME)
        .collect();
    let total_bytes = manifest.entries.iter().map(|e| e.size).sum();
    Ok(InspectReport {
        version: manifest.version,
        timestamp: manifest.timestamp,
        machine_id: manifest.machine_id,
        file_count: paths.len(),
        total_bytes,
        paths,
    })
}
