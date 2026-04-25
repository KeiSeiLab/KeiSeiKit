//! Error type for the `keisei` CLI.
//!
//! Constructor Pattern: single responsibility — own all failure modes of the
//! attach / status / mount / detach flow as one thiserror enum. Every other
//! module returns `Result<T, Error>` using the `#[from]` conversions here.

use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("brain manifest not found at {0}")]
    BrainNotFound(PathBuf),

    #[error("manifest too large: {size} bytes (limit {max})")]
    ManifestTooLarge { size: u64, max: u64 },

    #[error("brain schema version {found} not supported (need 1 or 2)")]
    UnsupportedSchema { found: u32 },

    #[error(
        "no mcp_server binary for {os}-{arch}; available: {}",
        available.join(", ")
    )]
    NoPlatformBinary {
        os: String,
        arch: String,
        available: Vec<String>,
    },

    #[error("no supported client detected in this directory")]
    NoClientDetected,

    #[error("config parse error at {path}: {reason}")]
    ConfigParseError { path: PathBuf, reason: String },

    #[error(
        "brain manifest path '{0}' escapes the brain root \
         (absolute path, parent traversal, or canonical mismatch)"
    )]
    PathEscape(PathBuf),

    #[error(
        "invalid brain name '{0}' — must match ^[a-z][a-z0-9_-]{{0,63}}$ \
         (lowercase, letter-start, 1-64 chars, word chars + hyphen)"
    )]
    InvalidName(String),

    #[error(
        "MCP entry '{name}' already exists in {existing_client} config with \
         different content; resolve manually (keisei will not clobber)"
    )]
    NameConflict {
        name: String,
        existing_client: String,
    },

    #[error(
        "brain path '{input}' is a symlink to '{target}'; \
         pass the canonical path explicitly to avoid USB/host pivot ambiguity"
    )]
    BrainIsSymlink { input: PathBuf, target: PathBuf },

    #[error(
        "adapter '{client}' does not support scope '{scope}' \
         (supported: {})",
        supported.join(", ")
    )]
    ScopeUnsupported {
        client: String,
        scope: String,
        supported: Vec<String>,
    },

    #[error("failed to load brain at {path}: {source}")]
    BrainLoad {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),

    #[error("toml parse: {0}")]
    TomlDe(#[from] toml::de::Error),

    #[error("toml serialize: {0}")]
    TomlSer(#[from] toml::ser::Error),

    #[error("json: {0}")]
    Json(#[from] serde_json::Error),

    #[error("yaml: {0}")]
    Yaml(#[from] serde_yaml::Error),
}
