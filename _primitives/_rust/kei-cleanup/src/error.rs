//! Error type for kei-cleanup runtime — see CLEANUP-RUNTIME-SPEC.md §3.3.
//!
//! Scanner failures are non-fatal at the runtime level: a missing
//! external tool returns `ToolNotFound`, which the dispatcher records
//! as `scanners_skipped` rather than aborting the whole run.

use std::path::PathBuf;
use thiserror::Error;

/// Runtime error type — see CLEANUP-RUNTIME-SPEC.md §3.3.
#[derive(Debug, Error)]
pub enum CleanupError {
    /// External CLI (cargo-machete / cargo-udeps) not on PATH.
    #[error("required tool not available: {0}")]
    ToolNotFound(String),

    /// Failed to read or parse a Cargo.toml.
    #[error("manifest read/parse error at {path:?}: {detail}")]
    Manifest {
        /// Path that failed.
        path: PathBuf,
        /// Underlying io / toml error as string.
        detail: String,
    },

    /// Failed to walk workspace tree.
    #[error("walkdir error: {0}")]
    Walk(String),

    /// Generic IO error.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialisation failure (report writer).
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    /// External tool returned non-zero exit but produced no parseable output.
    #[error("external tool '{tool}' failed: {detail}")]
    ToolFailure {
        /// Tool name (e.g. "cargo-machete").
        tool: String,
        /// Captured stderr or status detail.
        detail: String,
    },
}
