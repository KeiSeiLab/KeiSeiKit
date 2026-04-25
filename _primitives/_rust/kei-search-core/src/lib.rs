//! kei-search-core — 3-wave deep research engine, budget-capped.
//!
//! Waves:
//!  0 — claim extraction from prompt
//!  1 — per-claim source hunt (WebFetch stubbed behind [`SourceFetcher`] trait)
//!  2 — cross-validation + consensus scoring
//!
//! Port of LBM internal/search. The actual fetch is a trait the caller
//! supplies. Default implementation returns empty (frozen interface, todo!()
//! reflects unimplemented runtime).

pub mod budget;
pub mod export;
pub mod fetch;
pub mod pipeline;
pub mod schema;
pub mod store;
pub mod types;

pub use pipeline::run_research;
pub use store::ResearchStore;
pub use types::{Claim, Research, Source};
