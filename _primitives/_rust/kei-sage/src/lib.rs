//! kei-sage — SQLite knowledge-vault with FTS5 + typed edges + BFS + PageRank.
//!
//! Port of LBM internal/sage. Constructor Pattern: one concept per file.

pub mod atom_cli;
pub mod atom_index;
pub mod atom_parse;
pub mod atoms;
pub mod bfs;
pub mod edges;
pub mod facet_query;
pub mod import;
pub mod lineage;
pub mod pagerank;
pub mod rule_index;
pub mod schema;
pub mod search;
pub mod store;
pub mod types;

pub use store::Store;
pub use types::{Edge, Related, Unit};
