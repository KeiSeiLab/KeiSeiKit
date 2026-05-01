//! Error enum — exit-code carrier for the kei-llm-llamacpp CLI.
//!
//! Exit-code mapping (locked by spec):
//!   0 success   — never reached via Error
//!   1 IO/parse  — SpawnFailed, ParseFailed
//!   2 not-found — BinaryNotFound, ModelNotFound
//!   3 process   — NonZeroExit
//!   4 timeout   — Timeout
//!   5 security  — InvalidHost (non-localhost rejected)

use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    BinaryNotFound { name: String },
    ModelNotFound { path: PathBuf },
    SpawnFailed { source: std::io::Error },
    NonZeroExit { code: i32, stderr: String },
    ParseFailed { reason: String },
    Timeout,
    InvalidHost { host: String },
}

impl Error {
    /// Map this error to the locked CLI exit code.
    pub fn exit_code(&self) -> u8 {
        match self {
            Error::SpawnFailed { .. } | Error::ParseFailed { .. } => 1,
            Error::BinaryNotFound { .. } | Error::ModelNotFound { .. } => 2,
            Error::NonZeroExit { .. } => 3,
            Error::Timeout => 4,
            Error::InvalidHost { .. } => 5,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::BinaryNotFound { name } => {
                write!(f, "binary not found on PATH: {name}")
            }
            Error::ModelNotFound { path } => {
                write!(f, "model not found: {}", path.display())
            }
            Error::SpawnFailed { source } => {
                write!(f, "process spawn failed: {source}")
            }
            Error::NonZeroExit { code, stderr } => {
                write!(f, "process exited with code {code}: {stderr}")
            }
            Error::ParseFailed { reason } => {
                write!(f, "stdout parse failed: {reason}")
            }
            Error::Timeout => write!(f, "process timed out"),
            Error::InvalidHost { host } => {
                write!(
                    f,
                    "rejected host '{host}': only 127.0.0.1 / localhost / ::1 allowed (security)"
                )
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::SpawnFailed { source } => Some(source),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        Error::SpawnFailed { source }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
