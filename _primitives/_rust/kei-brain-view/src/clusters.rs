//! Cluster rendering over kei-dna-index groupings.
//!
//! Constructor Pattern: one file = one responsibility (render clusters).
//! Pulls cluster groupings from `kei-dna-index` and decorates each member
//! with its current ledger status. Read-only — no schema mutation.

use crate::error::BrainViewError;
use kei_dna_index::{cluster_by, ClusterBy};
use rusqlite::Connection;

/// Render the cluster tree for `by` as an ASCII text block.
///
/// Each non-singleton cluster becomes a `CLUSTER <key> (N members)` header
/// followed by `  └─ <dna> [status]` rows. Empty clusters produce `""`.
pub fn render_clusters(
    conn: &Connection,
    by: ClusterBy,
) -> Result<String, BrainViewError> {
    let clusters = cluster_by(conn, by)?;
    if clusters.is_empty() {
        return Ok(String::new());
    }
    let mut out = String::new();
    for c in clusters {
        out.push_str(&format!(
            "CLUSTER {} ({} members)\n",
            c.key,
            c.members.len()
        ));
        for dna in &c.members {
            let status = lookup_status(conn, dna);
            out.push_str(&format!("  └─ {}  [{}]\n", dna, status));
        }
    }
    Ok(out)
}

/// Return the `agents.status` column for the row whose DNA matches.
/// Falls back to `"unknown"` if the row is missing or the query fails —
/// rendering must always succeed even against a sparsely-populated ledger.
fn lookup_status(conn: &Connection, dna: &str) -> String {
    conn.query_row(
        "SELECT COALESCE(status,'unknown') FROM agents WHERE dna = ?1",
        [dna],
        |r| r.get::<_, String>(0),
    )
    .unwrap_or_else(|_| "unknown".to_string())
}
