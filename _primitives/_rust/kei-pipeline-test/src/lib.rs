//! kei-pipeline-test — end-to-end integration test crate.
//!
//! This crate has no runtime API. All logic lives in `tests/e2e.rs`,
//! which exercises the `kei-cleanup` → `kei-arch-derive` → `kei-arch-map`
//! pipeline against a synthetic tempdir fixture.
//!
//! Tests gracefully skip when the prerequisite binaries are not built.
//! To run end-to-end:
//!
//! ```text
//! cargo build --release -p kei-cleanup -p kei-arch-derive -p kei-arch-map
//! cargo test  --release -p kei-pipeline-test
//! ```
