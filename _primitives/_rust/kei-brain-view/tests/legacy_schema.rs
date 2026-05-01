//! Legacy-schema compat: brain-view must handle a pre-v2 ledger that
//! lacks the `dna` / `creator_id` / `fork_parent_id` columns. Separate
//! test file so the main integration suite stays focused.

use kei_brain_view::build_graph;
use rusqlite::Connection;

#[test]
fn older_schema_without_dna_column_still_builds() {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("legacy.sqlite");
    let conn = Connection::open(&db).unwrap();
    conn.execute_batch(
        "CREATE TABLE agents (
            id TEXT PRIMARY KEY,
            branch TEXT NOT NULL,
            parent_branch TEXT,
            spec_sha TEXT NOT NULL,
            status TEXT NOT NULL,
            started_ts INTEGER NOT NULL,
            finished_ts INTEGER,
            summary TEXT,
            worktree_path TEXT
        );",
    )
    .unwrap();
    conn.execute(
        "INSERT INTO agents (id, branch, parent_branch, spec_sha, status, started_ts)
         VALUES ('legacy1', 'agent/legacy', NULL, 'x', 'done', 100)",
        [],
    )
    .unwrap();
    let g = build_graph(&conn).unwrap();
    assert_eq!(g.nodes.len(), 1);
    assert_eq!(g.node(0).id, "legacy1");
    assert!(g.node(0).dna.is_none());
    assert!(g.node(0).creator_id.is_none());
}
