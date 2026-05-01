//! kei-dna-index — read-only adjacency / cluster / precedent primitive over
//! the kei-ledger `agents.dna` column.
//!
//! No schema mutation. No dependency on kei-ledger or kei-agent-runtime crates.

pub mod adjacency;
pub mod cluster;
pub mod db;
pub mod error;
pub mod parsed;
pub mod precedent;
pub mod stats;

pub use adjacency::{adjacent, AdjacencyKind, AdjacencyResult, Relationship};
pub use cluster::{cluster_by, Cluster, ClusterBy};
pub use db::open_read_only;
pub use error::{Error, Result};
pub use parsed::{split_dna, ParsedDna};
pub use precedent::precedent;
pub use stats::{stats, Stats};
