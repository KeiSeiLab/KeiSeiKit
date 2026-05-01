//! Error type for brain-view ops.
//!
//! Constructor Pattern: one cube = one error type + its trait impls.
//! Thiserror-based so `Display` + `From<rusqlite::Error>` are derived.

use thiserror::Error;

/// Hard cap on BFS descent to guard against cyclic or runaway data.
/// Mirrors `kei-ledger::MAX_TREE_DEPTH` — the visualizer must fail fast
/// rather than hang rendering a malformed graph.
pub const MAX_TREE_DEPTH: usize = 1024;

#[derive(Debug, Error)]
pub enum BrainViewError {
    #[error("sql: {0}")]
    Sql(#[from] rusqlite::Error),

    /// BFS traversal exceeded `MAX_TREE_DEPTH` iterations — the underlying
    /// ledger likely contains a cycle or an unexpectedly deep chain.
    #[error("tree walk exceeded {0} iterations (cycle or runaway ledger)")]
    MaxDepthExceeded(usize),

    /// Requested DNA prefix matched zero rows in the ledger.
    #[error("dna not found: {0}")]
    DnaNotFound(String),

    /// Requested DNA prefix matched multiple rows; caller must disambiguate.
    #[error("dna prefix {prefix} ambiguous ({count} matches)")]
    DnaAmbiguous { prefix: String, count: usize },

    #[error(transparent)]
    DnaIndex(#[from] kei_dna_index::Error),
}

pub type Result<T> = std::result::Result<T, BrainViewError>;
