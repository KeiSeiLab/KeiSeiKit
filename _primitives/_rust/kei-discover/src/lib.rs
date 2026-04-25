//! kei-discover — Wave 14 federated marketplace discovery stub.
//!
//! Local index of primitives announced by other KeiSeiKit users.
//! Metadata-only: `register` records a primitive (slug, author, url,
//! description), `list_available` returns not-yet-installed entries,
//! `mark_installed` flips the flag (does NOT fetch — real federation is
//! a future wave), `search` runs FTS over slug + description, `stats`
//! reports totals.
//!
//! Storage is delegated to `kei-entity-store`: schema lives in
//! `schema.rs`, each API verb lives in its own module (Constructor
//! Pattern, 1 file = 1 responsibility). The crate is engine-native —
//! every write / read routes through kei_entity_store verbs so a future
//! backend swap (remote registry, IPFS, etc.) only touches one layer.

pub mod entry;
pub mod error;
pub mod install;
pub mod list;
pub mod register;
pub mod schema;
pub mod search;
pub mod stats;
pub mod store;

pub use entry::Entry;
pub use error::DiscoverError;
pub use install::mark_installed;
pub use list::list_available;
pub use register::register;
pub use search::search;
pub use stats::{stats, Stats};
pub use store::{open, open_memory, Store};
