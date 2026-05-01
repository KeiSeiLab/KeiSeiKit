//! hibernate-manifest.toml schema.
//!
//! One manifest entry per file in the bundle. `machine_id` captures
//! the source host so operators can detect cross-machine restores.
//! Version gate blocks imports from future/incompatible primitives.

use serde::{Deserialize, Serialize};

pub const MANIFEST_FILENAME: &str = "hibernate-manifest.toml";
pub const MANIFEST_VERSION: &str = "1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ManifestEntry {
    /// Path inside the bundle (forward-slash normalised, relative
    /// to `kit_root`). Matches the tar archive entry name.
    pub path: String,
    /// Hex-encoded SHA-256 digest of file bytes at export time.
    pub sha256: String,
    /// File size in bytes (pre-compression). Sanity check; not
    /// load-bearing for integrity (sha256 is the real guarantee).
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HibernateManifest {
    pub version: String,
    pub timestamp: i64,
    pub machine_id: String,
    pub entries: Vec<ManifestEntry>,
}

impl HibernateManifest {
    pub fn new(timestamp: i64, machine_id: String, entries: Vec<ManifestEntry>) -> Self {
        Self {
            version: MANIFEST_VERSION.to_string(),
            timestamp,
            machine_id,
            entries,
        }
    }

    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }

    pub fn from_toml(raw: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(raw)
    }

    /// Locate an entry by bundle-relative path.
    pub fn lookup(&self, path: &str) -> Option<&ManifestEntry> {
        self.entries.iter().find(|e| e.path == path)
    }
}
