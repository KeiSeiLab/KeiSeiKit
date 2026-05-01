//! Prune orphan URIs — those that appear in `cross_edges` but have no in-edges.
//! Conservative: only removes edges where the tail URI has no other incoming edge.

use anyhow::Result;
use rusqlite::Connection;

pub fn prune_orphans(conn: &Connection) -> Result<usize> {
    // Find URIs that appear as to_uri but also as from_uri with no other incoming
    // => they are dead-ends. We remove edges where the outgoing side is orphan.
    let deleted = conn.execute(
        "DELETE FROM cross_edges
         WHERE to_uri IN (
             SELECT e1.from_uri FROM cross_edges e1
             WHERE NOT EXISTS (
                 SELECT 1 FROM cross_edges e2
                 WHERE e2.to_uri = e1.from_uri
             )
         )",
        [],
    )?;
    Ok(deleted)
}
