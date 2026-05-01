//! kei-brain-view — read-only visualizer of the kei-ledger taxonomy graph.
//!
//! Wave 14 concept: turns the SQLite `agents` table into an in-memory
//! `Graph` and renders it as ASCII tree, summary stats, or a DNA-centric
//! lineage view. NO writes to the ledger. NO new data sources.
//!
//! Constructor Pattern: each sub-module owns one primitive (error, graph,
//! render, stats, lineage). `lib.rs` is a pure re-export surface so the
//! binary and integration tests share the same types.

pub mod clusters;
pub mod error;
pub mod graph;
pub mod lineage;
pub mod render;
pub mod stats;
pub mod summary;

pub use clusters::render_clusters;
pub use error::{BrainViewError, Result};
pub use graph::{build_graph, resolve_dna, Graph, Node};
pub use lineage::{lineage, Lineage};
pub use render::{render_ascii, render_ascii_with_color, render_lineage};
pub use stats::{compute_stats, render_stats, Stats};
pub use summary::render_summary;
