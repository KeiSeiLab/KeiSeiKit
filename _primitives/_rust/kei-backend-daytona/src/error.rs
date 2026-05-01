//! Error types for the Daytona backend.
//!
//! All public APIs return `Result<_, DaytonaError>`. No panics outside tests.

use std::fmt;

/// Top-level error variant.
#[derive(Debug)]
pub enum DaytonaError {
    /// 401/403 from the API — bad/missing API key.
    Auth(String),
    /// 404 — sandbox does not exist.
    NotFound(String),
    /// 429/503 — caller should retry; we surface after exhausting retries.
    RateLimited(String),
    /// reqwest transport failure (DNS, TLS, timeout).
    Network(String),
    /// JSON serialization/deserialization failed.
    Serde(String),
    /// Any non-retriable HTTP error not covered above.
    Unknown(String),
}

impl fmt::Display for DaytonaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auth(m) => write!(f, "daytona auth error: {m}"),
            Self::NotFound(m) => write!(f, "daytona not found: {m}"),
            Self::RateLimited(m) => write!(f, "daytona rate-limited: {m}"),
            Self::Network(m) => write!(f, "daytona network error: {m}"),
            Self::Serde(m) => write!(f, "daytona serde error: {m}"),
            Self::Unknown(m) => write!(f, "daytona unknown error: {m}"),
        }
    }
}

impl std::error::Error for DaytonaError {}

impl From<reqwest::Error> for DaytonaError {
    fn from(e: reqwest::Error) -> Self {
        Self::Network(e.to_string())
    }
}

impl From<serde_json::Error> for DaytonaError {
    fn from(e: serde_json::Error) -> Self {
        Self::Serde(e.to_string())
    }
}

/// Result alias used throughout the crate.
pub type Result<T> = std::result::Result<T, DaytonaError>;
