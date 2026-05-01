//! kei-social-store — people + organizations + interactions.

pub mod graph;
pub mod interactions;
pub mod people;
pub mod schema;
pub mod search;
pub mod store;

pub use people::{Organization, Person};
pub use store::Store;
