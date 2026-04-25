//! `ApplyError` — structured failure reasons for `apply()`.
//!
//! Kept in its own module so `apply.rs` stays focused on the algorithm
//! and each file stays within Constructor Pattern limits.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ApplyError {
    InvalidPointer(String),
    MissingParent(String),
    MissingTarget(String),
    IndexOutOfBounds { path: String, index: usize, len: usize },
    TypeMismatch { path: String, expected: &'static str },
    CannotAddToRoot,
    CannotRemoveRoot,
}

impl fmt::Display for ApplyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPointer(p) => write!(f, "invalid JSON pointer: {p:?}"),
            Self::MissingParent(p) => write!(f, "missing parent at {p:?}"),
            Self::MissingTarget(p) => write!(f, "missing target at {p:?}"),
            Self::IndexOutOfBounds { path, index, len } => {
                write!(f, "index {index} out of bounds (len {len}) at {path:?}")
            }
            Self::TypeMismatch { path, expected } => {
                write!(f, "type mismatch at {path:?}: expected {expected}")
            }
            Self::CannotAddToRoot => write!(f, "cannot 'add' to root (use 'replace')"),
            Self::CannotRemoveRoot => write!(f, "cannot 'remove' root"),
        }
    }
}

impl std::error::Error for ApplyError {}
