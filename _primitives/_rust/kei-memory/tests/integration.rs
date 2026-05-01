//! Integration tests for kei-memory.
//!
//! Constructor Pattern: each test = one scenario, one assertion target.
//! Uses tempfile for per-test isolated sqlite file. Loads source modules
//! via `#[path]` so we don't need to expose a library crate surface.

#[path = "../src/schema.rs"]
mod schema;
#[path = "../src/similarity.rs"]
mod similarity;
#[path = "../src/coaccess.rs"]
mod coaccess;
#[path = "../src/tfidf.rs"]
mod tfidf;
#[path = "../src/injection_patterns.rs"]
mod injection_patterns;
#[path = "../src/injection_guard.rs"]
mod injection_guard;
#[path = "../src/ingest.rs"]
mod ingest;
#[path = "../src/analyze.rs"]
mod analyze;
#[path = "../src/patterns.rs"]
mod patterns;

use rusqlite::Connection;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tempfile::TempDir;

fn open_tmp() -> (TempDir, Connection) {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("kei-memory.sqlite");
    let conn = Connection::open(&db_path).unwrap();
    schema::migrate(&conn).unwrap();
    (dir, conn)
}

fn write_jsonl(dir: &TempDir, name: &str, lines: &[&str]) -> PathBuf {
    let p = dir.path().join(name);
    let mut f = fs::File::create(&p).unwrap();
    for l in lines {
        writeln!(f, "{l}").unwrap();
    }
    p
}

#[test]
fn ingest_then_analyze_roundtrip() {
    let (d, conn) = open_tmp();
    let trace = write_jsonl(&d, "s1.jsonl", &[
        r#"{"ts":1700000000,"kind":"tool_use","tool":"Bash","message":"ok"}"#,
        r#"{"ts":1700000010,"kind":"tool_use","tool":"Edit","file_path":"/a.rs"}"#,
        r#"{"ts":1700000020,"kind":"tool_use","tool":"Bash","is_error":true,"message":"permission denied"}"#,
    ]);
    let n = ingest::ingest_jsonl(&conn, "s1", &trace).unwrap();
    assert_eq!(n, 3);
    let hdr = analyze::session_header(&conn, "s1").unwrap().unwrap();
    assert_eq!(hdr.tool_call_count, 3);
    assert_eq!(hdr.error_count, 1);
    let report = analyze::render_report(&conn, "s1", false).unwrap();
    assert!(report.contains("Tool calls:  3"));
    assert!(report.contains("Errors:      1"));
}

#[test]
fn coaccess_counts_pair_within_window() {
    let (d, conn) = open_tmp();
    let trace = write_jsonl(&d, "s2.jsonl", &[
        r#"{"ts":1700000000,"kind":"tool_use","tool":"Edit","file_path":"/a.rs"}"#,
        r#"{"ts":1700000060,"kind":"tool_use","tool":"Edit","file_path":"/b.rs"}"#,
        r#"{"ts":1700000120,"kind":"tool_use","tool":"Edit","file_path":"/a.rs"}"#,
    ]);
    ingest::ingest_jsonl(&conn, "s2", &trace).unwrap();
    let pairs = coaccess::top_pairs(&conn, 10).unwrap();
    assert!(!pairs.is_empty());
    let hit = pairs.iter().find(|(a, b, _)| {
        (a == "/a.rs" && b == "/b.rs") || (a == "/b.rs" && b == "/a.rs")
    });
    assert!(hit.is_some(), "expected pair (/a.rs,/b.rs), got {pairs:?}");
    assert!(hit.unwrap().2 >= 1);
}

#[test]
fn tfidf_similarity_between_known_docs() {
    let (_d, conn) = open_tmp();
    tfidf::index_document(&conn, "sA", "rust cargo workspace conflict build error").unwrap();
    tfidf::index_document(&conn, "sB", "rust cargo workspace conflict ci").unwrap();
    tfidf::index_document(&conn, "sC", "swift xcode simulator audio").unwrap();
    let top = tfidf::top_similar(&conn, "rust cargo workspace", 3).unwrap();
    assert!(!top.is_empty());
    let best = &top[0].0;
    assert!(best == "sA" || best == "sB", "expected sA or sB first, got {best}");
    let worst = top.iter().find(|(id, _)| id == "sC");
    if let Some((_, s)) = worst {
        assert!(*s <= top[0].1, "unrelated doc should not outrank target");
    }
}

#[test]
fn pattern_detection_finds_recurring_class() {
    let (d, conn) = open_tmp();
    let trace = write_jsonl(&d, "s3.jsonl", &[
        r#"{"ts":1700000000,"kind":"tool_use","tool":"Bash","event_class":"worktree_denied","is_error":true}"#,
        r#"{"ts":1700000010,"kind":"tool_use","tool":"Bash","event_class":"worktree_denied","is_error":true}"#,
        r#"{"ts":1700000020,"kind":"tool_use","tool":"Bash","event_class":"worktree_denied","is_error":true}"#,
        r#"{"ts":1700000030,"kind":"tool_use","tool":"Read","event_class":"read_ok"}"#,
    ]);
    ingest::ingest_jsonl(&conn, "s3", &trace).unwrap();
    let hits = patterns::detect_in_session(&conn, "s3").unwrap();
    let wd = hits.iter().find(|h| h.event_class == "worktree_denied");
    assert!(wd.is_some(), "expected worktree_denied pattern");
    assert_eq!(wd.unwrap().count, 3);
}

#[test]
fn stats_counts_sessions_and_events() {
    let (d, conn) = open_tmp();
    let t1 = write_jsonl(&d, "a.jsonl", &[
        r#"{"ts":1,"kind":"tool_use","tool":"Bash"}"#,
        r#"{"ts":2,"kind":"tool_use","tool":"Edit"}"#,
    ]);
    let t2 = write_jsonl(&d, "b.jsonl", &[
        r#"{"ts":3,"kind":"tool_use","tool":"Grep"}"#,
    ]);
    ingest::ingest_jsonl(&conn, "a", &t1).unwrap();
    ingest::ingest_jsonl(&conn, "b", &t2).unwrap();
    let n_sess: i64 = conn.query_row("SELECT COUNT(*) FROM sessions", [], |r| r.get(0)).unwrap();
    let n_evt: i64 = conn.query_row("SELECT COUNT(*) FROM events", [], |r| r.get(0)).unwrap();
    assert_eq!(n_sess, 2);
    assert_eq!(n_evt, 3);
}

#[test]
fn backlog_crud_add_list_clear() {
    let (_d, conn) = open_tmp();
    let now = 1700000000i64;
    conn.execute(
        "INSERT INTO backlog (ts, item) VALUES (?1, ?2)",
        rusqlite::params![now, "item-one"],
    ).unwrap();
    conn.execute(
        "INSERT INTO backlog (ts, item) VALUES (?1, ?2)",
        rusqlite::params![now + 1, "item-two"],
    ).unwrap();
    let open_ct: i64 = conn.query_row(
        "SELECT COUNT(*) FROM backlog WHERE processed = 0", [], |r| r.get(0),
    ).unwrap();
    assert_eq!(open_ct, 2);
    conn.execute("UPDATE backlog SET processed = 1", []).unwrap();
    let after: i64 = conn.query_row(
        "SELECT COUNT(*) FROM backlog WHERE processed = 0", [], |r| r.get(0),
    ).unwrap();
    assert_eq!(after, 0);
}

#[test]
fn cross_session_pattern_needs_two_sessions() {
    let (d, conn) = open_tmp();
    let a = write_jsonl(&d, "a.jsonl", &[
        r#"{"ts":1,"kind":"tool_use","event_class":"foo"}"#,
    ]);
    let b = write_jsonl(&d, "b.jsonl", &[
        r#"{"ts":2,"kind":"tool_use","event_class":"foo"}"#,
    ]);
    ingest::ingest_jsonl(&conn, "a", &a).unwrap();
    ingest::ingest_jsonl(&conn, "b", &b).unwrap();
    let cross = patterns::detect_cross_session(&conn).unwrap();
    let foo = cross.iter().find(|p| p.event_class == "foo");
    assert!(foo.is_some());
    assert_eq!(foo.unwrap().count, 2);
}

#[test]
fn cosine_similarity_sanity() {
    let mut a = std::collections::HashMap::new();
    a.insert("rust".to_string(), 1.0);
    a.insert("cargo".to_string(), 1.0);
    let mut b = std::collections::HashMap::new();
    b.insert("rust".to_string(), 1.0);
    b.insert("cargo".to_string(), 1.0);
    let s_ident = similarity::cosine_tfidf(&a, &b);
    assert!((s_ident - 1.0).abs() < 1e-9);
    let mut c = std::collections::HashMap::new();
    c.insert("swift".to_string(), 1.0);
    let s_ortho = similarity::cosine_tfidf(&a, &c);
    assert!(s_ortho.abs() < 1e-9);
}
