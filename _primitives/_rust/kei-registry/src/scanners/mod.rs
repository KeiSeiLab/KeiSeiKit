//! Scanner trait + adapter registry.
//!
//! Constructor Pattern: each cube under `scanners/` is one Scanner adapter
//! for one block type. The trait stays minimal — `scan(root) -> Vec<Found>`
//! with no I/O contract beyond walking the filesystem read-only. The
//! registry CLI dispatcher composes scanners; scanners do not know about
//! SQLite.

pub mod atom;
pub mod block_md;
pub mod capability;
pub mod hook;
pub mod primitive;
pub mod role;
pub mod rule;
pub mod skill;

use std::path::Path;

use crate::block::BlockType;

/// One detected artefact from a scanner. Caller (CLI) merges these into
/// `register()` calls to upsert the SQLite store.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Found {
    pub block_type: BlockType,
    pub name: String,
    pub path: String,
    pub body: Vec<u8>,
    pub caps: String,
}

/// Filesystem scanner adapter. One impl per block type. Each scanner walks
/// its own conventional root (primitives → workspace `_primitives/_rust/`,
/// skills → `<kit>/skills/`, etc.) and returns one `Found` per artefact.
pub trait Scanner {
    /// Scan `root` and return zero or more found artefacts. Errors return
    /// `Err`; missing directory returns `Ok(vec![])`.
    fn scan(&self, root: &Path) -> anyhow::Result<Vec<Found>>;
}
