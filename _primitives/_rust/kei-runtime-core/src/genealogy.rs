// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! [`HasGenealogy`] — trait for entities that can walk their parent
//! chain. Combined with [`HasDna`], gives every entity a queryable
//! genealogy from itself back to the root user signup.

use crate::dna::{Dna, HasDna};

/// An entity whose ancestors can be looked up. Implementations typically
/// query a backing store (D1, kei-ledger SQLite, kei-dna-index) to walk
/// the `parent_dna` chain.
#[async_trait::async_trait]
pub trait HasGenealogy: HasDna {
    /// Walk parents from immediate up to root. Empty when self is root.
    async fn ancestors(&self) -> crate::Result<Vec<Dna>>;

    /// Convenience: ultimate ancestor. Returns self's DNA when self is
    /// already root.
    async fn root_dna(&self) -> crate::Result<Dna> {
        let chain = self.ancestors().await?;
        Ok(chain.last().cloned().unwrap_or_else(|| self.dna().clone()))
    }

    /// Return depth from root. 0 = self IS root.
    async fn depth(&self) -> crate::Result<usize> {
        Ok(self.ancestors().await?.len())
    }
}
