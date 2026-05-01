//! Library surface for `frustration-matrix`.
//!
//! Exposes the byte-level n-gram firmware (training + likelihood scoring),
//! the regex-category SSoT, and the JSONL chatlog parser for reuse by
//! sibling crates:
//!   * `dna-store::axis_semantic` — consumes `firmware`
//!   * `kei-frustration-loop` — consumes `firmware`, `firmware_corpus`,
//!     `categories`, `jsonl` to drive the per-user online learning loop
//!
//! Kept deliberately narrow: only modules the binary AND external sibling
//! crates need to consume are public. Internal helpers (scan, report,
//! classifier, eval) stay private to the binary.
//!
//! The binary (`main.rs`) continues to compile independently with its own
//! `mod firmware;` declarations — library and binary share source files
//! via Cargo's dual-target rule, not via re-use from one to the other.
//!
//! Constructor Pattern: this cube is pure re-export. Any behaviour change
//! happens inside the individual `firmware*.rs` / `jsonl.rs` /
//! `categories.rs` cubes, not here.

pub mod categories;
pub mod firmware;
pub mod firmware_corpus;
pub mod firmware_ngram;
pub mod jsonl;
