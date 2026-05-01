//! kei-capability library surface — exposes `fork` for integration tests.
//!
//! The binary (`src/main.rs`) owns Check/Verify dispatch; the library
//! re-exports the pure copy+rewrite logic used by `kei-capability fork`.

pub mod fork;
