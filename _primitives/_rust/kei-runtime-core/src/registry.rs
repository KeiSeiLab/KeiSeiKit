// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! [`Registry`] — in-memory DNA-keyed registry for trait impls.
//!
//! Each `kei-{compute,llm,git,...}-*` impl registers itself at startup
//! by handing the orchestrator a [`RegistryEntry`]. The registry refuses
//! anonymous impls (impl must satisfy [`HasDna`] via its `RegistryEntry`).

use crate::dna::Dna;
use crate::error::{Error, Result};
use std::collections::HashMap;
use std::sync::Arc;

/// What every registered impl carries.
pub struct RegistryEntry {
    pub dna: Dna,
    pub kind: TraitKind,
    pub display_name: String,
    pub version: String,
}

/// The 12 trait surfaces. Each registry slot is keyed by `(kind, dna)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TraitKind {
    Compute,
    Llm,
    Git,
    Memory,
    Notify,
    Scheduler,
    Service,
    Network,
    Backup,
    Cost,
    Auth,
    Observability,
}

impl TraitKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            TraitKind::Compute => "compute",
            TraitKind::Llm => "llm",
            TraitKind::Git => "git",
            TraitKind::Memory => "memory",
            TraitKind::Notify => "notify",
            TraitKind::Scheduler => "scheduler",
            TraitKind::Service => "service",
            TraitKind::Network => "network",
            TraitKind::Backup => "backup",
            TraitKind::Cost => "cost",
            TraitKind::Auth => "auth",
            TraitKind::Observability => "observability",
        }
    }
}

/// Registry of all loaded impls. Cheap to clone (Arc-internal).
#[derive(Default, Clone)]
pub struct Registry {
    entries: Arc<std::sync::RwLock<HashMap<(TraitKind, Dna), RegistryEntry>>>,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&self, entry: RegistryEntry) -> Result<()> {
        let key = (entry.kind, entry.dna.clone());
        let mut map = self
            .entries
            .write()
            .map_err(|e| Error::Registry(format!("poisoned: {e}")))?;
        if map.contains_key(&key) {
            return Err(Error::Registry(format!(
                "duplicate registration {:?}::{}",
                entry.kind, entry.dna
            )));
        }
        map.insert(key, entry);
        Ok(())
    }

    pub fn list(&self, kind: TraitKind) -> Result<Vec<Dna>> {
        let map = self
            .entries
            .read()
            .map_err(|e| Error::Registry(format!("poisoned: {e}")))?;
        Ok(map
            .keys()
            .filter(|(k, _)| *k == kind)
            .map(|(_, d)| d.clone())
            .collect())
    }

    pub fn count(&self, kind: TraitKind) -> Result<usize> {
        Ok(self.list(kind)?.len())
    }

    pub fn count_all(&self) -> Result<usize> {
        let map = self
            .entries
            .read()
            .map_err(|e| Error::Registry(format!("poisoned: {e}")))?;
        Ok(map.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dna::DnaBuilder;

    fn dummy_dna(role: &str) -> Dna {
        DnaBuilder::new(role).cap("MN").scope("test").body(b"x").build().unwrap()
    }

    #[test]
    fn register_and_list() {
        let r = Registry::new();
        let dna = dummy_dna("primitive");
        r.register(RegistryEntry {
            dna: dna.clone(),
            kind: TraitKind::Compute,
            display_name: "kei-compute-hetzner".into(),
            version: "0.1.0".into(),
        })
        .unwrap();
        assert_eq!(r.count(TraitKind::Compute).unwrap(), 1);
        assert_eq!(r.count(TraitKind::Llm).unwrap(), 0);
        assert_eq!(r.list(TraitKind::Compute).unwrap()[0], dna);
    }

    #[test]
    fn duplicate_rejected() {
        let r = Registry::new();
        let dna = dummy_dna("primitive");
        r.register(RegistryEntry {
            dna: dna.clone(),
            kind: TraitKind::Compute,
            display_name: "k".into(),
            version: "0.1.0".into(),
        })
        .unwrap();
        let dup = r.register(RegistryEntry {
            dna,
            kind: TraitKind::Compute,
            display_name: "k".into(),
            version: "0.1.0".into(),
        });
        assert!(dup.is_err());
    }
}
