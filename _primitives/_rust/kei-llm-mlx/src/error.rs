//! Error enum — exit-code SSoT.
//!
//! Each variant maps to a stable exit code in `cli.rs`:
//!   1 → IO / parse / serialize error
//!   2 → NotSupported (platform gate)
//!   3 → BinaryNotFound / ModelNotFound
//!   4 → NonZeroExit (subprocess returned non-zero)
//!   5 → Timeout
//!
//! Constructor Pattern: this cube holds the enum + impls only. Conversion
//! to `ExitCode` lives in `cli.rs` so the error surface stays
//! interpretation-agnostic (a library consumer can map differently).

use std::fmt;

#[derive(Debug)]
pub enum Error {
    /// Platform gate refused. Reason is the stable `is_supported()` string.
    NotSupported(String),
    /// `which mlx_lm.generate` / `mlx_lm.server` returned nothing.
    BinaryNotFound(String),
    /// Model id not present in the cache (or cache directory missing).
    ModelNotFound(String),
    /// `Command::output` failed (binary not executable, permission, etc.).
    SpawnFailed(String),
    /// Child exited with non-zero. `code = None` only on signal kill.
    NonZeroExit { code: Option<i32>, stderr: String },
    /// stdout / footer / NDJSON could not be parsed.
    ParseFailed(String),
    /// Reserved for future timeout-bounded calls. Currently unused; kept
    /// in the surface so the exit-code table is stable across the W57-W59
    /// trio.
    Timeout,
    /// Security gate refused (e.g. `--host` not localhost).
    SecurityRefused(String),
    /// IO / serialize error not covered by the above.
    Io(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotSupported(r) => write!(f, "not supported: {r}"),
            Error::BinaryNotFound(b) => write!(f, "binary not found: {b}"),
            Error::ModelNotFound(m) => write!(f, "model not found: {m}"),
            Error::SpawnFailed(e) => write!(f, "spawn failed: {e}"),
            Error::NonZeroExit { code, stderr } => {
                let c = code.map(|c| c.to_string()).unwrap_or_else(|| "signal".into());
                write!(f, "non-zero exit ({c}): {stderr}")
            }
            Error::ParseFailed(s) => write!(f, "parse failed: {s}"),
            Error::Timeout => write!(f, "timeout"),
            Error::SecurityRefused(s) => write!(f, "security refused: {s}"),
            Error::Io(s) => write!(f, "io: {s}"),
        }
    }
}

impl std::error::Error for Error {}

/// Exit-code table. SSoT for `cli.rs`.
pub fn exit_code_for(err: &Error) -> u8 {
    match err {
        Error::Io(_) | Error::ParseFailed(_) | Error::SpawnFailed(_) => 1,
        Error::NotSupported(_) => 2,
        Error::BinaryNotFound(_) | Error::ModelNotFound(_) => 3,
        Error::NonZeroExit { .. } => 4,
        Error::Timeout => 5,
        Error::SecurityRefused(_) => 1,
    }
}
