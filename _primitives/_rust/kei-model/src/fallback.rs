//! `chain` — walk `fallback` field until None or cycle.
//!
//! Detects cycles via a visited-set. Unknown ids halt the walk before adding
//! the unknown id to the chain. Returns a `Vec<Model>` in walk order with the
//! primary at index 0.

use anyhow::{anyhow, Result};
use std::collections::HashSet;

use crate::model::Model;
use crate::registry::Registry;

/// Walk the fallback chain starting at `primary`.
///
/// Errors:
///   * Unknown `primary` id (caller maps to exit-2).
///   * Cycle detected (caller maps to exit-3).
pub fn chain(primary: &str, registry: &Registry) -> Result<Vec<Model>> {
    let first = registry
        .get(primary)
        .ok_or_else(|| anyhow!("unknown primary model_id: {primary}"))?;

    let mut visited: HashSet<String> = HashSet::new();
    let mut acc: Vec<Model> = Vec::new();
    push_step(&mut visited, &mut acc, first.clone())?;
    walk_remaining(&mut visited, &mut acc, registry)?;
    Ok(acc)
}

fn walk_remaining(
    visited: &mut HashSet<String>,
    acc: &mut Vec<Model>,
    registry: &Registry,
) -> Result<()> {
    while let Some(next_id) = next_id_from(acc) {
        match registry.get(&next_id) {
            None => return Ok(()),
            Some(m) => push_step(visited, acc, m.clone())?,
        }
    }
    Ok(())
}

fn next_id_from(acc: &[Model]) -> Option<String> {
    let last = acc.last()?;
    last.fallback_target().map(|s| s.to_string())
}

fn push_step(
    visited: &mut HashSet<String>,
    acc: &mut Vec<Model>,
    m: Model,
) -> Result<()> {
    if !visited.insert(m.id.clone()) {
        return Err(anyhow!("cycle in fallback chain at {}", m.id));
    }
    acc.push(m);
    Ok(())
}
