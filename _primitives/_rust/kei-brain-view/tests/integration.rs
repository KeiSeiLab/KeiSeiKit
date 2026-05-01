//! Integration tests for kei-brain-view.
//!
//! Constructor Pattern: each test = one scenario. A helper seeds a
//! minimal kei-ledger-compatible schema into a tempfile sqlite, then
//! the library walks it read-only.

use kei_brain_view::{
    build_graph, compute_stats, lineage, render_ascii_with_color, BrainViewError,
};
use rusqlite::{params, Connection};
use tempfile::TempDir;

/// Seed a v4-compatible `agents` table and return (tempdir, conn).
fn seed_db() -> (TempDir, Connection) {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("ledger.sqlite");
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
            worktree_path TEXT,
            dna TEXT,
            creator_id TEXT,
            fork_parent_id TEXT
        );",
    )
    .unwrap();
    (dir, conn)
}

#[allow(clippy::too_many_arguments)]
fn insert(
    conn: &Connection,
    id: &str,
    branch: &str,
    parent: Option<&str>,
    status: &str,
    ts: i64,
    dna: Option<&str>,
) {
    conn.execute(
        "INSERT INTO agents
         (id, branch, parent_branch, spec_sha, status, started_ts, dna)
         VALUES (?1, ?2, ?3, 'deadbeef', ?4, ?5, ?6)",
        params![id, branch, parent, status, ts, dna],
    )
    .unwrap();
}

#[test]
fn build_graph_empty_db() {
    let (_d, conn) = seed_db();
    let g = build_graph(&conn).unwrap();
    assert_eq!(g.nodes.len(), 0);
    assert_eq!(g.roots.len(), 0);
    assert!(g.children_of.is_empty());
}

#[test]
fn build_graph_single_root() {
    let (_d, conn) = seed_db();
    insert(&conn, "a1", "agent/a1", None, "running", 1, Some("DNA1"));
    let g = build_graph(&conn).unwrap();
    assert_eq!(g.nodes.len(), 1);
    assert_eq!(g.roots, vec![0]);
    assert_eq!(g.node(0).id, "a1");
    assert_eq!(g.node(0).dna.as_deref(), Some("DNA1"));
}

#[test]
fn build_graph_chain_3_deep() {
    let (_d, conn) = seed_db();
    insert(&conn, "a1", "agent/a1", None, "done", 1, Some("DNAA"));
    insert(&conn, "a2", "agent/a2", Some("agent/a1"), "done", 2, Some("DNAB"));
    insert(&conn, "a3", "agent/a3", Some("agent/a2"), "running", 3, Some("DNAC"));
    let g = build_graph(&conn).unwrap();
    assert_eq!(g.nodes.len(), 3);
    assert_eq!(g.roots.len(), 1);
    assert_eq!(g.roots[0], 0);
    assert_eq!(g.children_of.get("agent/a1").unwrap().len(), 1);
    assert_eq!(g.children_of.get("agent/a2").unwrap().len(), 1);
    assert!(!g.children_of.contains_key("agent/a3"));
}

#[test]
fn render_ascii_preserves_order() {
    let (_d, conn) = seed_db();
    insert(&conn, "a1", "agent/a1", None, "done", 1, Some("DNA_ROOT"));
    insert(&conn, "a2", "agent/a2", Some("agent/a1"), "done", 2, Some("DNA_MID"));
    insert(&conn, "a3", "agent/a3", Some("agent/a1"), "running", 3, Some("DNA_SIB"));
    insert(&conn, "a4", "agent/a4", Some("agent/a2"), "failed", 4, Some("DNA_LEAF"));
    let g = build_graph(&conn).unwrap();
    let s = render_ascii_with_color(&g, false);
    let p_root = s.find("a1").unwrap();
    let p_mid = s.find("a2").unwrap();
    let p_leaf = s.find("a4").unwrap();
    let p_sib = s.find("a3").unwrap();
    assert!(p_root < p_mid, "root before middle child");
    assert!(p_mid < p_leaf, "middle before leaf");
    assert!(p_leaf < p_sib, "oldest-child subtree before sibling");
    assert!(!s.contains("\x1b["), "no-color output must be plain ASCII");
}

#[test]
fn lineage_returns_ancestors_and_descendants() {
    let (_d, conn) = seed_db();
    insert(&conn, "root", "agent/root", None, "done", 1, Some("DNA_ROOT"));
    insert(&conn, "mid", "agent/mid", Some("agent/root"), "done", 2, Some("DNA_MID"));
    insert(&conn, "focus", "agent/focus", Some("agent/mid"), "running", 3, Some("DNA_FOCUS"));
    insert(&conn, "leaf", "agent/leaf", Some("agent/focus"), "running", 4, Some("DNA_LEAF"));
    insert(&conn, "other", "agent/other", None, "done", 5, Some("DNA_OTHER"));
    let g = build_graph(&conn).unwrap();
    let l = lineage(&g, "DNA_FOCUS").unwrap();
    assert!(l.focus.is_some());
    assert_eq!(l.focus.as_ref().unwrap().id, "focus");
    let anc_ids: Vec<&str> = l.ancestors.iter().map(|n| n.id.as_str()).collect();
    assert_eq!(anc_ids, vec!["root", "mid"]);
    let desc_ids: Vec<&str> = l.descendants.iter().map(|n| n.id.as_str()).collect();
    assert_eq!(desc_ids, vec!["leaf"]);
    assert!(!anc_ids.contains(&"other"));
    assert!(!desc_ids.contains(&"other"));
}

#[test]
fn stats_buckets_by_status() {
    let (_d, conn) = seed_db();
    insert(&conn, "a", "agent/a", None, "running", 1, Some("DNA1"));
    insert(&conn, "b", "agent/b", Some("agent/a"), "done", 2, Some("DNA2"));
    insert(&conn, "c", "agent/c", Some("agent/a"), "done", 3, None);
    insert(&conn, "d", "agent/d", None, "failed", 4, Some("DNA4"));
    let g = build_graph(&conn).unwrap();
    let s = compute_stats(&g);
    assert_eq!(s.total, 4);
    assert_eq!(s.roots, 2);
    assert_eq!(s.forks, 2);
    assert_eq!(s.with_dna, 3);
    assert_eq!(s.by_status.get("running"), Some(&1));
    assert_eq!(s.by_status.get("done"), Some(&2));
    assert_eq!(s.by_status.get("failed"), Some(&1));
    assert_eq!(s.by_status.get("merged"), None);
}

#[test]
fn dna_prefix_ambiguous_surface_error() {
    let (_d, conn) = seed_db();
    insert(&conn, "a", "agent/a", None, "done", 1, Some("DNA_SHARED_AAA"));
    insert(&conn, "b", "agent/b", None, "done", 2, Some("DNA_SHARED_BBB"));
    let g = build_graph(&conn).unwrap();
    match lineage(&g, "DNA_SHARED") {
        Err(BrainViewError::DnaAmbiguous { count, .. }) => assert_eq!(count, 2),
        other => panic!("expected DnaAmbiguous, got {other:?}"),
    }
    match lineage(&g, "DNA_DOES_NOT_EXIST") {
        Err(BrainViewError::DnaNotFound(_)) => {}
        other => panic!("expected DnaNotFound, got {other:?}"),
    }
}
