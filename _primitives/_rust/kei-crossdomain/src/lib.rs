//! kei-crossdomain — SQLite store for domain-to-domain typed edges + BFS.

pub mod auto_link;
pub mod bfs;
pub mod edges;
pub mod schema;
pub mod store;
pub mod types;

pub use store::Store;
pub use types::CrossEdge;
