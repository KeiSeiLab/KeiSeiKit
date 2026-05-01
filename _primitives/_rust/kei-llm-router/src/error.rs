//! Error enum for kei-llm-router.
//!
//! Constructor Pattern: ONE responsibility — name failure modes and map
//! each to a stable exit code. The CLI surfaces the code; the lib surface
//! returns `Error` so callers can handle programmatically.
//!
//! Exit code map (per task spec):
//!   0 success
//!   1 IO / probe error           → ProbeFailed | IoError
//!   2 no backend available       → NoBackendAvailable | NoCompatibleBackend
//!   3 model not in registry      → ModelNotInRegistry

use std::fmt;

/// All failure modes the router surfaces.
#[derive(Debug)]
pub enum Error {
    /// `kei_machine_probe::probe()` failed before we could decide.
    ProbeFailed { reason: String },

    /// No viable backend reached an "available" state for `model_id`.
    NoBackendAvailable { model_id: String, tried: Vec<String> },

    /// Machine reports `Capability::NoLocalInferenceViable` — no point
    /// querying any backend.
    NoCompatibleBackend { reason: String },

    /// `kei_model::Registry::get(model_id)` returned None and the
    /// caller required a registry lookup (e.g. for fallback).
    ModelNotInRegistry { model_id: String },

    /// Generic IO / file / serde failure.
    IoError { reason: String },
}

impl Error {
    /// Stable exit code for the CLI to surface.
    pub fn exit_code(&self) -> i32 {
        match self {
            Error::ProbeFailed { .. } | Error::IoError { .. } => 1,
            Error::NoBackendAvailable { .. } | Error::NoCompatibleBackend { .. } => 2,
            Error::ModelNotInRegistry { .. } => 3,
        }
    }

    /// Human-readable kind tag for JSON serialisation.
    pub fn kind(&self) -> &'static str {
        match self {
            Error::ProbeFailed { .. } => "probe-failed",
            Error::NoBackendAvailable { .. } => "no-backend-available",
            Error::NoCompatibleBackend { .. } => "no-compatible-backend",
            Error::ModelNotInRegistry { .. } => "model-not-in-registry",
            Error::IoError { .. } => "io-error",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ProbeFailed { reason } => write!(f, "probe failed: {reason}"),
            Error::NoBackendAvailable { model_id, tried } => {
                write!(
                    f,
                    "no backend available for `{model_id}` (tried: {})",
                    tried.join(", ")
                )
            }
            Error::NoCompatibleBackend { reason } => {
                write!(f, "machine not compatible: {reason}")
            }
            Error::ModelNotInRegistry { model_id } => {
                write!(f, "model `{model_id}` is not in the registry")
            }
            Error::IoError { reason } => write!(f, "io error: {reason}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError { reason: e.to_string() }
    }
}

/// Convenient `Result` alias used throughout the crate.
pub type Result<T> = std::result::Result<T, Error>;
