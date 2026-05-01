//! Brain — portable exobrain directory representation.
//!
//! A "brain" is a self-contained directory on any filesystem (USB, iCloud,
//! remote mount) attached to an AI client via the `keisei` CLI. It
//! declares its layout in a top-level `manifest.toml`.
//!
//! Two schemas are supported:
//!
//! * **v1** — single-string `mcp_server = "bin/kei-mcp-server-<os>-<arch>"`
//!   (one brain per platform).
//! * **v2** — `[paths.mcp_server]` table keyed by `<os>-<arch>` so a
//!   single brain on USB serves every host automatically.
//!
//! # Invariants (audit-hardened, v0.19 + v0.20)
//!
//! - **Path confinement** — every path under `[paths]` MUST be relative;
//!   absolute paths and `..` components are rejected syntactically, and
//!   the canonical form must remain inside the brain root
//!   (`Error::PathEscape`). In schema v2 every map value is checked
//!   independently.
//! - **Symlink reject** — the brain-root input itself cannot be a
//!   symlink; the user must pass the canonical path to close the
//!   USB → `$HOME` pivot (`Error::BrainIsSymlink`).
//! - **Name regex** — `^[a-z][a-z0-9_-]{0,63}$` on `brain.name`: lowercase
//!   letter start, up to 64 chars, word chars + hyphen only
//!   (`Error::InvalidName`).
//! - **Manifest size bound** — `manifest.toml` is capped at 64 KiB
//!   (`brain_validate::MAX_MANIFEST_BYTES`); anything larger returns
//!   `Error::ManifestTooLarge` before the toml parser sees a byte.
//! - **Schema range** — `schema_version ∈ {1, 2}` accepted (see
//!   `MIN_SCHEMA..=MAX_SCHEMA`). v1 = single-string `mcp_server`; v2 =
//!   `[paths.mcp_server]` map keyed by `<os>-<arch>`.
//!
//! Platform key format (v2): derived from `std::env::consts` with renames
//! `macos → darwin`, `x86_64 → x64`, `aarch64 → arm64`. See
//! [`Brain::current_platform_key`]. A brain may ship only a subset of
//! platforms — the missing ones surface as [`Error::NoPlatformBinary`]
//! at `mcp_server_path()` call time, NOT at load time, so
//! `keisei status` can still inspect a partial brain.
//!
//! Constructor Pattern: single responsibility — parse + compose the
//! validation primitives from `brain_validate.rs` into the load pipeline.

use crate::brain_validate as v;
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Lowest schema version understood by this binary.
pub const MIN_SCHEMA: u32 = 1;
/// Highest schema version understood by this binary.
pub const MAX_SCHEMA: u32 = 2;
pub const MANIFEST_FILENAME: &str = "manifest.toml";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BrainMeta {
    pub schema_version: u32,
    pub name: String,
    #[serde(default)]
    pub created: Option<String>,
}

/// v1 carries a single relative path; v2 carries a map keyed by
/// `<os>-<arch>`. Serde's `untagged` dispatch picks the right arm by TOML
/// shape — string vs table.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum McpServerPath {
    /// Schema v1 form: single relative path good for one platform only.
    Single(String),
    /// Schema v2 form: `{ "darwin-arm64": "bin/...", "linux-x64": "bin/..." }`.
    PerPlatform(BTreeMap<String, String>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BrainPaths {
    /// Required. Path(s) to the MCP server binary, relative to the brain root.
    pub mcp_server: McpServerPath,
    /// Optional. If present, must be relative + in-root.
    #[serde(default)]
    pub memory: Option<String>,
    /// Optional. If present, must be relative + in-root.
    #[serde(default)]
    pub artifacts: Option<String>,
    /// Optional. If present, must be relative + in-root.
    #[serde(default)]
    pub manifests: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BrainManifest {
    pub brain: BrainMeta,
    pub paths: BrainPaths,
}

#[derive(Debug, Clone)]
pub struct Brain {
    pub root: PathBuf,
    pub manifest: BrainManifest,
}

impl Brain {
    /// Load a brain from `<root>/manifest.toml`.
    ///
    /// Order matters (security-critical):
    ///   1. Reject symlink-rooted inputs (SEC-H3 — USB/host pivot).
    ///   2. Canonicalize `root`.
    ///   3. Parse manifest, validate schema_version ∈ {1, 2}.
    ///   4. Validate `brain.name` against regex.
    ///   5. Syntactic path-escape check on every declared path (all v2
    ///      platform entries included). Canonicalization is deferred to
    ///      `mcp_server_path()` so an incomplete brain (missing current
    ///      platform's binary) still loads and shows up in `status`.
    pub fn load(input: &Path) -> Result<Self> {
        v::reject_symlink_root(input)?;
        let root = v::canonicalize_root(input)?;
        crate::fs_type::warn_on_unsafe_fs(&root);
        let manifest = v::read_manifest(&root)?;
        v::validate_schema(&manifest)?;
        v::validate_name(&manifest.brain.name)?;
        check_all_paths(&manifest)?;
        Ok(Self { root, manifest })
    }

    /// Return the `<os>-<arch>` key used to look up v2 platform entries.
    ///
    /// Mapping (differs from raw `std::env::consts`):
    /// * `macos`  → `darwin`
    /// * `x86_64` → `x64`
    /// * `aarch64`→ `arm64`
    /// * everything else passes through unchanged.
    pub fn current_platform_key() -> String {
        let os = match std::env::consts::OS {
            "macos" => "darwin",
            other => other,
        };
        let arch = match std::env::consts::ARCH {
            "x86_64" => "x64",
            "aarch64" => "arm64",
            other => other,
        };
        format!("{os}-{arch}")
    }

    /// Resolve the mcp_server binary for the current host and canonicalize
    /// against the brain root. Errors:
    /// * [`Error::NoPlatformBinary`] — v2 brain without a map entry for
    ///   the current `(os, arch)`.
    /// * [`Error::PathEscape`] / [`Error::BrainLoad`] / [`Error::BrainNotFound`]
    ///   — propagated from the canonicalizer.
    pub fn mcp_server_path(&self) -> Result<PathBuf> {
        let rel = self.resolve_mcp_rel()?;
        v::canonicalize_in_root(&self.root, rel)
    }

    fn resolve_mcp_rel(&self) -> Result<&str> {
        match &self.manifest.paths.mcp_server {
            McpServerPath::Single(rel) => Ok(rel.as_str()),
            McpServerPath::PerPlatform(map) => {
                let key = Self::current_platform_key();
                match map.get(&key) {
                    Some(rel) => Ok(rel.as_str()),
                    None => Err(Error::NoPlatformBinary {
                        os: std::env::consts::OS.into(),
                        arch: std::env::consts::ARCH.into(),
                        available: map.keys().cloned().collect(),
                    }),
                }
            }
        }
    }

    pub fn name(&self) -> &str {
        &self.manifest.brain.name
    }
}

fn check_all_paths(manifest: &BrainManifest) -> Result<()> {
    match &manifest.paths.mcp_server {
        McpServerPath::Single(rel) => v::check_relative_in_root(rel)?,
        McpServerPath::PerPlatform(map) => {
            for rel in map.values() {
                v::check_relative_in_root(rel)?;
            }
        }
    }
    if let Some(p) = manifest.paths.memory.as_deref() {
        v::check_relative_in_root(p)?;
    }
    if let Some(p) = manifest.paths.artifacts.as_deref() {
        v::check_relative_in_root(p)?;
    }
    if let Some(p) = manifest.paths.manifests.as_deref() {
        v::check_relative_in_root(p)?;
    }
    Ok(())
}
