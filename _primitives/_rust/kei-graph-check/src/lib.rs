//! kei-graph-check — post-refactor reference-integrity gate.
//!
//! Inputs: a directory root + an optional patch file (advisory only — we
//! detect file deletions/renames declared in the patch header and warn).
//! Output: list of broken references with file:line.

pub mod graph;
pub mod patch_advisory;

pub use graph::{BrokenRef, Graph};
