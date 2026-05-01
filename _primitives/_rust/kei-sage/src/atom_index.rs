//! Persist discovered atoms into the kei-sage Store as Units + typed edges.
//!
//! Unit-type = `"atom"`; `vault_path` = atom full_id (e.g. `kei-task::create`).
//! Edge-type = `"atom_related"` for wikilinks between atoms. Idempotent:
//! re-ingesting the same corpus replaces existing rows by vault_path.

use crate::atoms::{resolve_wikilinks, AtomRecord};
use crate::edges::add_edge;
use crate::store::Store;
use crate::types::Unit;
use anyhow::Result;

pub struct IndexStats {
    pub units_indexed: usize,
    pub edges_indexed: usize,
}

pub fn index_atoms(store: &Store, records: &[AtomRecord]) -> Result<IndexStats> {
    let units_indexed = index_units(store, records)?;
    let edges_indexed = index_edges(store, records)?;
    Ok(IndexStats { units_indexed, edges_indexed })
}

fn index_units(store: &Store, records: &[AtomRecord]) -> Result<usize> {
    let mut n = 0;
    for rec in records {
        store.add_unit(&record_to_unit(rec))?;
        n += 1;
    }
    Ok(n)
}

fn record_to_unit(rec: &AtomRecord) -> Unit {
    Unit {
        unit_type: "atom".into(),
        title: rec.full_id.clone(),
        content: build_content(rec),
        evidence_grade: rec.stability.clone(),
        source_path: rec.md_path.to_string_lossy().into(),
        vault_path: rec.full_id.clone(),
        category: rec.kind.as_str().into(),
        ..Default::default()
    }
}

fn build_content(rec: &AtomRecord) -> String {
    let kw = rec.keywords.join(", ");
    let mut s = String::with_capacity(rec.body.len() + kw.len() + 64);
    s.push_str("[keywords] ");
    s.push_str(&kw);
    s.push_str("\n\n");
    s.push_str(&rec.body);
    s
}

fn index_edges(store: &Store, records: &[AtomRecord]) -> Result<usize> {
    let mut n = 0;
    for (src, dst) in resolve_wikilinks(records) {
        add_edge(store, &src, &dst, "atom_related", 1.0)?;
        n += 1;
    }
    Ok(n)
}
