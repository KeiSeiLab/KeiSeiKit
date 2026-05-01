//! kei-shared — shared substrate types.
//!
//! Single source of truth for the agent DNA wire format. Consumers
//! (kei-agent-runtime, kei-dna-index) import from here so a format
//! change is a one-file edit, not a two-crate refactor.
//!
//! Constructor Pattern: one file = one responsibility. `dna.rs` owns the
//! parse/compose/validate primitives, nothing else.

pub mod dna;

pub use dna::{compose_dna, is_hex8, parse_dna, DnaError, ParsedDna};
