//! Integration tests for `render_clusters`.
//!
//! Each test seeds a minimal kei-ledger-compatible `agents` table with
//! canonical DNAs (`<role>::<caps>::<sha8-scope>::<sha8-body>-<hex8-nonce>`)
//! and asserts on the rendered ASCII block.

use kei_brain_view::render_clusters;
use kei_dna_index::ClusterBy;
use rusqlite::{params, Connection};
use tempfile::TempDir;

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

fn dna(role: &str, caps: &str, scope: &str, body: &str, nonce: &str) -> String {
    format!("{role}::{caps}::{scope}::{body}-{nonce}")
}

fn insert(conn: &Connection, id: &str, status: &str, ts: i64, dna_str: &str) {
    conn.execute(
        "INSERT INTO agents
         (id, branch, parent_branch, spec_sha, status, started_ts, dna)
         VALUES (?1, ?2, NULL, 'deadbeef', ?3, ?4, ?5)",
        params![id, format!("agent/{id}"), status, ts, dna_str],
    )
    .unwrap();
}

#[test]
fn render_clusters_scope_shows_tree() {
    let (_d, conn) = seed_db();
    // Three agents sharing scope AAAAAAAA; distinct bodies + nonces.
    let d1 = dna("edit", "NG-FW", "AAAAAAAA", "B1B1B1B1", "11111111");
    let d2 = dna("edit", "NG-FW", "AAAAAAAA", "B2B2B2B2", "22222222");
    let d3 = dna("edit", "NG-FW", "AAAAAAAA", "B3B3B3B3", "33333333");
    insert(&conn, "a1", "running", 1, &d1);
    insert(&conn, "a2", "done", 2, &d2);
    insert(&conn, "a3", "failed", 3, &d3);

    let out = render_clusters(&conn, ClusterBy::Scope).unwrap();
    assert!(
        out.contains("CLUSTER AAAAAAAA (3 members)"),
        "header missing: {out}"
    );
    assert_eq!(out.matches("└─").count(), 3, "expected 3 members: {out}");
    assert!(out.contains("[running]"));
    assert!(out.contains("[done]"));
    assert!(out.contains("[failed]"));
}

#[test]
fn render_clusters_empty_returns_empty_string() {
    let (_d, conn) = seed_db();
    // All singleton scopes → cluster_by drops all → empty output.
    let d1 = dna("edit", "NG-FW", "AAAAAAAA", "B1B1B1B1", "11111111");
    let d2 = dna("edit", "NG-FW", "CCCCCCCC", "B2B2B2B2", "22222222");
    insert(&conn, "a1", "running", 1, &d1);
    insert(&conn, "a2", "done", 2, &d2);

    let out = render_clusters(&conn, ClusterBy::Scope).unwrap();
    assert_eq!(out, "", "singletons must not render: {out:?}");
}

#[test]
fn render_clusters_body_groups_by_body_sha() {
    let (_d, conn) = seed_db();
    // All scope/body/nonce must be 8 hex chars per split_dna's validator.
    let d1 = dna("edit", "NG-FW", "10000001", "BEEFBEEF", "11111111");
    let d2 = dna("edit", "NG-FW", "20000002", "BEEFBEEF", "22222222");
    let d3 = dna("edit", "NG-FW", "30000003", "FEEDFEED", "33333333");
    insert(&conn, "a1", "running", 1, &d1);
    insert(&conn, "a2", "done", 2, &d2);
    insert(&conn, "a3", "failed", 3, &d3);

    let out = render_clusters(&conn, ClusterBy::Body).unwrap();
    assert!(
        out.contains("CLUSTER BEEFBEEF (2 members)"),
        "body cluster missing: {out}"
    );
    // FEEDFEED is singleton → must not appear as header.
    assert!(!out.contains("FEEDFEED"), "singleton leaked: {out}");
}

#[test]
fn render_clusters_role_caps_groups_by_role() {
    let (_d, conn) = seed_db();
    // Two agents share role+caps "edit::NG-FW"; one differs on caps.
    let d1 = dna("edit", "NG-FW", "AAAAAAAA", "B1B1B1B1", "11111111");
    let d2 = dna("edit", "NG-FW", "CCCCCCCC", "B2B2B2B2", "22222222");
    let d3 = dna("edit", "TG-ND", "DDDDDDDD", "B3B3B3B3", "33333333");
    insert(&conn, "a1", "running", 1, &d1);
    insert(&conn, "a2", "done", 2, &d2);
    insert(&conn, "a3", "failed", 3, &d3);

    let out = render_clusters(&conn, ClusterBy::RoleCaps).unwrap();
    assert!(
        out.contains("CLUSTER edit::NG-FW (2 members)"),
        "role cluster missing: {out}"
    );
    // edit::TG-ND is singleton → excluded.
    assert!(!out.contains("edit::TG-ND"), "singleton leaked: {out}");
}
