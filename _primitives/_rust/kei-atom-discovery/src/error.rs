//! Typed errors for atom discovery + frontmatter parsing.
//!
//! Every failure mode is a distinct variant — callers pattern-match by variant,
//! not by `to_string()` scraping.

use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("path escape: `{rel}` escapes base `{}`", base.display())]
    PathEscape { base: PathBuf, rel: String },

    #[error("path absolute not allowed: `{0}`")]
    PathAbsolute(String),

    #[error("path contains parent component (..): `{0}`")]
    PathParent(String),

    #[error("canonicalize `{}`: {source}", path.display())]
    Canonicalize {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("frontmatter missing leading --- delimiter")]
    FrontmatterMissingStart,

    #[error("frontmatter missing closing --- delimiter")]
    FrontmatterMissingEnd,

    #[error("frontmatter exceeds {limit} bytes (got {got})")]
    FrontmatterTooLarge { limit: usize, got: usize },

    #[error("yaml parse: {0}")]
    Yaml(#[from] serde_yaml_ng::Error),

    #[error("atom id must be `<crate>::<verb>`, got `{0}`")]
    BadAtomId(String),

    #[error("unknown atom kind: `{0}`")]
    UnknownKind(String),

    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}
