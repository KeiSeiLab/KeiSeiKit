//! Integration tests for `render_summary`.

use kei_brain_view::render_summary;
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

fn insert(conn: &Connection, id: &str, status: &str, ts: i64, dna_str: Option<&str>) {
    conn.execute(
        "INSERT INTO agents
         (id, branch, parent_branch, spec_sha, status, started_ts, dna)
         VALUES (?1, ?2, NULL, 'deadbeef', ?3, ?4, ?5)",
        params![id, format!("agent/{id}"), status, ts, dna_str],
    )
    .unwrap();
}

#[test]
fn render_summary_shows_all_fields() {
    let (_d, conn) = seed_db();
    let d1 = dna("edit", "NG-FW", "AAAAAAAA", "B1B1B1B1", "11111111");
    let d2 = dna("edit", "NG-FW", "AAAAAAAA", "B2B2B2B2", "22222222");
    let d3 = dna("edit", "NG-FW", "CCCCCCCC", "B3B3B3B3", "33333333");
    insert(&conn, "a1", "running", 1, Some(&d1));
    insert(&conn, "a2", "done", 2, Some(&d2));
    insert(&conn, "a3", "failed", 3, Some(&d3));

    let out = render_summary(&conn).unwrap();
    assert!(out.contains("=== KeiSei Brain Summary ==="));
    assert!(out.contains("Total DNAs: 3"), "{out}");
    assert!(out.contains("Unique scopes: 2"), "{out}");
    assert!(out.contains("Unique bodies: 3"), "{out}");
    assert!(out.contains("Clusters (scope ≥2):"), "{out}");
    assert!(out.contains("Clusters (body ≥2):"), "{out}");
    assert!(out.contains("Avg cluster size:"), "{out}");
}

#[test]
fn render_summary_empty_ledger_shows_zeros() {
    let (_d, conn) = seed_db();
    let out = render_summary(&conn).unwrap();
    assert!(out.contains("Total DNAs: 0"), "{out}");
    assert!(out.contains("Unique scopes: 0"), "{out}");
    assert!(out.contains("Unique bodies: 0"), "{out}");
    assert!(out.contains("Clusters (scope ≥2): 0"), "{out}");
    assert!(out.contains("Clusters (body ≥2): 0"), "{out}");
    assert!(out.contains("Avg cluster size: 0.0"), "{out}");
}

#[test]
fn render_summary_ignores_malformed_dna() {
    let (_d, conn) = seed_db();
    // 3 valid rows + 1 malformed (missing body separator) + 1 NULL.
    let d1 = dna("edit", "NG-FW", "AAAAAAAA", "B1B1B1B1", "11111111");
    let d2 = dna("edit", "NG-FW", "AAAAAAAA", "B2B2B2B2", "22222222");
    let d3 = dna("edit", "NG-FW", "CCCCCCCC", "B3B3B3B3", "33333333");
    insert(&conn, "a1", "running", 1, Some(&d1));
    insert(&conn, "a2", "done", 2, Some(&d2));
    insert(&conn, "a3", "failed", 3, Some(&d3));
    insert(&conn, "bad", "done", 4, Some("not-a-dna"));
    insert(&conn, "nil", "done", 5, None);

    let out = render_summary(&conn).unwrap();
    // load_rows drops malformed + NULL silently → only 3 valid counted.
    assert!(out.contains("Total DNAs: 3"), "{out}");
    assert!(out.contains("Unique scopes: 2"), "{out}");
    assert!(out.contains("Unique bodies: 3"), "{out}");
}
