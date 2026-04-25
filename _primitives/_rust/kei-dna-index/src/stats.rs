//! Aggregate stats over parsed ledger DNAs.
//!
//! Constructor Pattern: one file = one responsibility.

use crate::cluster::{group, ClusterBy};
use crate::db::load_rows;
use crate::error::Result;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub total_dnas: usize,
    pub unique_scopes: usize,
    pub unique_bodies: usize,
    pub clusters_scope: usize,
    pub clusters_body: usize,
    pub avg_cluster_size: f64,
}

pub fn stats(conn: &Connection) -> Result<Stats> {
    let rows = load_rows(conn)?;
    let total_dnas = rows.len();
    let unique_scopes = rows
        .iter()
        .map(|r| r.parsed.scope_sha.as_str())
        .collect::<HashSet<_>>()
        .len();
    let unique_bodies = rows
        .iter()
        .map(|r| r.parsed.body_sha.as_str())
        .collect::<HashSet<_>>()
        .len();

    let scope_clusters = group(&rows, ClusterBy::Scope);
    let body_clusters = group(&rows, ClusterBy::Body);
    let clusters_scope = scope_clusters.len();
    let clusters_body = body_clusters.len();
    let avg_cluster_size = avg_size(&scope_clusters, &body_clusters);

    Ok(Stats {
        total_dnas,
        unique_scopes,
        unique_bodies,
        clusters_scope,
        clusters_body,
        avg_cluster_size,
    })
}

fn avg_size(
    scope_clusters: &[crate::cluster::Cluster],
    body_clusters: &[crate::cluster::Cluster],
) -> f64 {
    let total: usize = scope_clusters.iter().map(|c| c.members.len()).sum::<usize>()
        + body_clusters.iter().map(|c| c.members.len()).sum::<usize>();
    let n = scope_clusters.len() + body_clusters.len();
    if n == 0 {
        0.0
    } else {
        total as f64 / n as f64
    }
}
