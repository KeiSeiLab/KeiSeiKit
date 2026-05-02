// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! kei-runtime-core — Hosted Sleep runtime substrate.
//!
//! 12 capability traits + DNA + plugin registry. No backend implementations
//! live here; each `kei-{compute,llm,git,memory,notify,scheduler,service,
//! network,backup,cost,auth,observability}-*` sibling crate provides one.
//!
//! Every trait extends [`HasDna`]: there are no anonymous impls. Every
//! registered impl carries a serial that traces parent → child via
//! [`HasGenealogy`].
//!
//! See `~/Projects/keisei-marketplace/spec/DNA-CONVENTION.md` for the
//! universal serial format and `spec/CONFIG-REFERENCE.md` for the
//! per-trait configuration surface.

pub mod dna;
pub mod error;
pub mod genealogy;
pub mod registry;
pub mod secrets;
pub mod traits;

pub use dna::{Dna, DnaBuilder, HasDna};
pub use error::{Error, Result};
pub use genealogy::HasGenealogy;
pub use registry::{Registry, RegistryEntry};
pub use secrets::SecretString;
pub use traits::*;

// Re-export the wire-format SSoT from kei-shared so consumers don't need
// to depend on it directly.
pub use kei_shared::dna::{compose_dna, is_hex8, parse_dna, DnaError, ParsedDna};
