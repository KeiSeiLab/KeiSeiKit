//! Typed errors for kei-hibernate. One variant per failure class.
//! Constructor Pattern: no inheritance, no wrappers beyond thiserror.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error at {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("plain io: {0}")]
    Plain(#[from] std::io::Error),

    #[error("manifest serialise: {0}")]
    ManifestEncode(#[from] toml::ser::Error),

    #[error("manifest parse: {0}")]
    ManifestDecode(#[from] toml::de::Error),

    #[error("manifest missing from bundle ({0})")]
    ManifestMissing(&'static str),

    #[error("manifest version mismatch: bundle={bundle} primitive={primitive}")]
    VersionMismatch { bundle: String, primitive: String },

    #[error("sha256 mismatch for {path}: manifest={expected} bundle={actual}")]
    ShaMismatch {
        path: String,
        expected: String,
        actual: String,
    },

    #[error("bundle entry escapes target dir: {0}")]
    UnsafeEntryPath(String),

    #[error("kit root not a directory: {0}")]
    KitRootInvalid(PathBuf),
}
