//! kei-curator — exponential edge decay + orphan node prune.
//!
//! Operates on a `cross_edges` table compatible with kei-crossdomain.
//! Also usable standalone against any SQLite DB with the expected schema.

pub mod config;
pub mod decay;
pub mod orphans;

pub use config::Config;
pub use decay::{decay_edges, DecayReport};
pub use orphans::prune_orphans;
