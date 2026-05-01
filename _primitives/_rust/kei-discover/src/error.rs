//! `DiscoverError` — typed error for the kei-discover public API.
//!
//! The CLI maps validation-style errors (DuplicateSlug, NotFound,
//! InvalidInput) to exit 2 and storage / IO failures to exit 1,
//! matching the kei-entity-store convention.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DiscoverError {
    #[error("DuplicateSlug: `{0}` is already registered")]
    DuplicateSlug(String),

    #[error("NotFound: discover entry id {0}")]
    NotFound(i64),

    #[error("InvalidInput: {0}")]
    InvalidInput(String),

    #[error("Storage: {0}")]
    Storage(String),
}

impl DiscoverError {
    /// Exit code contract — 2 for user-facing input errors, 1 for
    /// storage / IO failures.
    pub fn exit_code(&self) -> u8 {
        match self {
            Self::DuplicateSlug(_) | Self::NotFound(_) | Self::InvalidInput(_) => 2,
            Self::Storage(_) => 1,
        }
    }
}

/// Map engine errors to kei-discover errors. SQLite unique-constraint
/// failures on the `slug` column are re-classified as `DuplicateSlug`
/// so callers get a typed signal instead of a raw SQL string.
impl From<kei_entity_store::VerbError> for DiscoverError {
    fn from(e: kei_entity_store::VerbError) -> Self {
        let msg = format!("{e}");
        if msg.contains("UNIQUE constraint failed") && msg.contains("slug") {
            // slug is carried by the caller — fill in at the call site
            // via map_err when more context is available. Here we emit
            // an empty marker that the caller overrides.
            return Self::DuplicateSlug(String::new());
        }
        match e.exit_code() {
            2 => Self::InvalidInput(msg),
            _ => Self::Storage(msg),
        }
    }
}

impl From<rusqlite::Error> for DiscoverError {
    fn from(e: rusqlite::Error) -> Self {
        Self::Storage(e.to_string())
    }
}
