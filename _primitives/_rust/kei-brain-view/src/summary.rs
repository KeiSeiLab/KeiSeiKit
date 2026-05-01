//! Summary rendering over kei-dna-index stats.
//!
//! Constructor Pattern: one file = one responsibility (render summary).
//! Thin formatter — the aggregation itself lives in `kei-dna-index::stats`.

use crate::error::BrainViewError;
use kei_dna_index::stats;
use rusqlite::Connection;

/// Format the DNA-index summary block as a single text blob.
///
/// All fields come verbatim from `kei_dna_index::stats`; this function is
/// pure presentation so the numbers can be re-used elsewhere unchanged.
pub fn render_summary(conn: &Connection) -> Result<String, BrainViewError> {
    let s = stats(conn)?;
    Ok(format!(
        "=== KeiSei Brain Summary ===\n\
         Total DNAs: {}\n\
         Unique scopes: {}\n\
         Unique bodies: {}\n\
         Clusters (scope ≥2): {}\n\
         Clusters (body ≥2): {}\n\
         Avg cluster size: {:.1}\n",
        s.total_dnas,
        s.unique_scopes,
        s.unique_bodies,
        s.clusters_scope,
        s.clusters_body,
        s.avg_cluster_size
    ))
}
