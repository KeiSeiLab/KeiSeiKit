//! kei-replay — reconstruct an agent spawn from its DNA string.
//!
//! Given a DNA `role::caps::scope::body-nonce`, look up the ledger row,
//! locate the archived `task.toml` for that agent, re-run the compose
//! pipeline, and compare the resulting body hash to the DNA's body segment.
//! A mismatch is schema drift since the original spawn.
//!
//! Constructor Pattern: one responsibility per cube. No I/O beyond SQLite
//! read + `std::fs` on task files + stdout.
//!
//! Modules:
//!   - `replay`        — reconstruct composed prompt from DNA
//!   - `diff`          — compare two DNAs (facets + bodies)
//!   - `ledger_lookup` — SQLite direct read of ledger rows by DNA

pub mod diff;
pub mod ledger_lookup;
pub mod replay;
