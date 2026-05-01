//! kei-decompose — UNIVERSAL decomposition layer.
//!
//! Supersedes the format-specific kei-decision (Wave 51) by treating it as
//! one adapter among many. Closes Wave 50 META-finding: kit has 6+ MD-output
//! formats (research / wave-audit / sleep / architecture / new-project /
//! compose-solution), but only `research` had a path to action via
//! kei-decision. kei-decompose unifies the decomposition layer.
//!
//! Pipeline:
//!   ANY MD output  →  detect  →  parser_registry  →  Action[]
//!                                                       ↓
//!                                                     emit
//!                                                       ↓
//!                                             task.toml[] for kei-spawn
//!                                                       ↓
//!                                                    dispatch
//!                                                       ↓
//!                                              kei-spawn (fork + ledger)
//!
//! Constructor Pattern: each module owns one responsibility, ≤ 200 LOC,
//! ≤ 30 LOC per fn. No async, no network, no md crate (regex-only).

pub mod cli;
pub mod dispatcher;
pub mod emitter;
pub mod normalizer;
pub mod parsers;
pub mod rules_cmd;
pub mod rules_paths;
pub mod rules_rebuild;
pub mod rules_walker;

pub use normalizer::{Action, Severity};
pub use parsers::{registry, FormatParser};
