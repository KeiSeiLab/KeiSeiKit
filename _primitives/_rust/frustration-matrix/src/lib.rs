//! Library surface for `frustration-matrix`.

//! Exposes the `firmware` byte-level n-gram language model for reuse by
//! sibling crates (e.g. `dna-store::axis_semantic`). Kept deliberately
//! narrow: only the four modules that form the firmware-training /
//! likelihood-scoring closure are public.

//! The binary (`main.rs`) continues to compile independently with its own
//! `mod firmware;` declarations — library and binary share source files
//! via Cargo's dual-target rule, not via re-use from one to the other.

//! Constructor Pattern: this cube is pure re-export. Any behaviour change
//! happens inside the individual `firmware*.rs` / `jsonl.rs` cubes, not
//! here.

pub mod firmware;
pub mod firmware_corpus;
pub mod firmware_ngram;
pub mod jsonl;
