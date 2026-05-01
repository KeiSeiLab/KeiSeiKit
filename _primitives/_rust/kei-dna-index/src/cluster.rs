//! Clustering over DNAs by scope / body / role+caps.
//!
//! Constructor Pattern: one file = one responsibility (cluster grouping).

use crate::db::{load_rows, Row};
use crate::error::Result;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClusterBy {
    Scope,
    Body,
    RoleCaps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cluster {
    pub key: String,
    pub members: Vec<String>,
}

pub fn cluster_by(conn: &Connection, by: ClusterBy) -> Result<Vec<Cluster>> {
    let rows = load_rows(conn)?;
    Ok(group(&rows, by))
}

/// Group rows by the selected key, dropping singleton groups.
/// Output is sorted by key for determinism.
pub(crate) fn group(rows: &[Row], by: ClusterBy) -> Vec<Cluster> {
    let mut buckets: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for r in rows {
        let key = key_for(r, by);
        buckets.entry(key).or_default().push(r.dna.clone());
    }
    buckets
        .into_iter()
        .filter(|(_, v)| v.len() > 1)
        .map(|(key, members)| Cluster { key, members })
        .collect()
}

fn key_for(r: &Row, by: ClusterBy) -> String {
    match by {
        ClusterBy::Scope => r.parsed.scope_sha.clone(),
        ClusterBy::Body => r.parsed.body_sha.clone(),
        ClusterBy::RoleCaps => format!("{}::{}", r.parsed.role, r.parsed.caps),
    }
}
