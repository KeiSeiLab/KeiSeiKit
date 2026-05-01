//! Verb error type. Distinguishes user-input / validation failures
//! (map to CLI exit 2 in callers) from storage / IO failures (exit 1).

use crate::ddl_error::DdlError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VerbError {
    #[error("InvalidInput: {0}")]
    InvalidInput(String),

    /// Typed-validation failure for a declared schema field.
    /// Distinct variant from free-form `InvalidInput` so callers can
    /// match on `{field, expected, got}` programmatically.
    #[error("InvalidInput: field `{field}` expected {expected}, got {got}")]
    InvalidType {
        field: String,
        expected: String,
        got: String,
    },

    #[error("VerbDisabled: {verb} not enabled on schema {schema}")]
    VerbDisabled { verb: String, schema: String },

    /// Generic not-found. `id` is rendered as text so the same variant
    /// works for integer-PK and text-PK (UUID) schemas.
    #[error("NotFound: {entity} id {id}")]
    NotFound { entity: String, id: String },

    #[error("Sqlite: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("Serde: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Storage: {0}")]
    Storage(String),
}

impl VerbError {
    /// Exit code contract — 2 for validation / unknown verb / not found;
    /// 1 for storage / IO.
    pub fn exit_code(&self) -> u8 {
        match self {
            Self::InvalidInput(_)
            | Self::InvalidType { .. }
            | Self::VerbDisabled { .. }
            | Self::NotFound { .. } => 2,
            Self::Sqlite(_) | Self::Serde(_) | Self::Storage(_) => 1,
        }
    }

    /// Construct a `NotFound` from an i64 id. Kept as a shim so existing
    /// call-sites passing integer PKs keep compiling.
    pub fn not_found_i64(entity: impl Into<String>, id: i64) -> Self {
        Self::NotFound { entity: entity.into(), id: id.to_string() }
    }

    /// Construct a `NotFound` from a String id (TextPk schemas).
    pub fn not_found_text(entity: impl Into<String>, id: impl Into<String>) -> Self {
        Self::NotFound { entity: entity.into(), id: id.into() }
    }
}

/// Map DDL-generation failures into verb errors. An unsupported
/// `extra_columns` FieldKind is caller-configuration input, so it maps
/// to `InvalidInput` (exit code 2) rather than the storage path.
impl From<DdlError> for VerbError {
    fn from(e: DdlError) -> Self {
        VerbError::InvalidInput(e.to_string())
    }
}
