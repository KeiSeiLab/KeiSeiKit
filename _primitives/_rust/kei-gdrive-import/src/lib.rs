//! kei-gdrive-import — project-folder classification primitive.
//!
//! Pure-compute scorer that turns a folder path into a verdict
//! (PROJECT / AMBIGUOUS / NOT-A-PROJECT / ALREADY-REPO) based on a
//! table-driven set of build-manifest markers. Used by the
//! drive-import wizard to pick which Google Drive folders deserve a
//! Forgejo repo. No network, no git, no async.

pub mod classify;
pub mod cli;
pub mod scan;
pub mod scoring;

pub use classify::{classify, classify_remote, Classification, MarkerHit, Verdict};
pub use scan::scan_tree;
pub use scoring::{marker_for, MARKERS, MarkerKind};
