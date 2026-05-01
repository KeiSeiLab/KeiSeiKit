//! kei-sage integration tests.

use kei_sage::bfs::bfs;
use kei_sage::edges::{add_edge, list_outgoing};
use kei_sage::import::import_vault;
use kei_sage::pagerank::pagerank;
use kei_sage::search::fts_search;
use kei_sage::{Store, Unit};
use std::fs;
use tempfile::tempdir;

fn mkstore() -> Store { Store::open_memory().unwrap() }

fn mkunit(title: &str, body: &str, vault: &str) -> Unit {
    Unit {
        unit_type: "note".into(), title: title.into(), content: body.into(),
        evidence_grade: "E2".into(), vault_path: vault.into(),
        ..Default::default()
    }
}

#[test]
fn crud_roundtrip() {
    let s = mkstore();
    let id = s.add_unit(&mkunit("hello", "world", "a.md")).unwrap();
    assert!(id > 0);
    let u = s.get_unit(id).unwrap().unwrap();
    assert_eq!(u.title, "hello");
    s.delete_unit(id).unwrap();
    assert!(s.get_unit(id).unwrap().is_none());
}

#[test]
fn fts_search_matches() {
    let s = mkstore();
    s.add_unit(&mkunit("rust async", "tokio runtime details", "a.md")).unwrap();
    s.add_unit(&mkunit("python sync", "flask wsgi server", "b.md")).unwrap();
    let hits = fts_search(&s, "tokio", 10).unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].title, "rust async");
}

#[test]
fn bfs_depth_limit() {
    let s = mkstore();
    add_edge(&s, "a", "b", "rel", 1.0).unwrap();
    add_edge(&s, "b", "c", "rel", 1.0).unwrap();
    add_edge(&s, "c", "d", "rel", 1.0).unwrap();
    let out = bfs(&s, "a", 2).unwrap();
    let paths: Vec<&str> = out.iter().map(|r| r.path.as_str()).collect();
    assert!(paths.contains(&"b"));
    assert!(paths.contains(&"c"));
    assert!(!paths.contains(&"d"));
}

#[test]
fn pagerank_orders_by_popularity() {
    let s = mkstore();
    add_edge(&s, "a", "hub", "rel", 1.0).unwrap();
    add_edge(&s, "b", "hub", "rel", 1.0).unwrap();
    add_edge(&s, "c", "hub", "rel", 1.0).unwrap();
    add_edge(&s, "d", "hub", "rel", 1.0).unwrap();
    add_edge(&s, "e", "hub", "rel", 1.0).unwrap();
    let ranks = pagerank(&s).unwrap();
    assert_eq!(ranks[0].0, "hub");
}

#[test]
fn edges_crud() {
    let s = mkstore();
    let id = add_edge(&s, "x", "y", "cites", 0.8).unwrap();
    assert!(id > 0);
    let out = list_outgoing(&s, "x").unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].dst_path, "y");
}

#[test]
fn import_idempotency() {
    let tmp = tempdir().unwrap();
    let p = tmp.path().join("one.md");
    fs::write(&p, "# title one\nhello").unwrap();
    let s = mkstore();
    let first = import_vault(&s, tmp.path()).unwrap();
    let second = import_vault(&s, tmp.path()).unwrap();
    assert_eq!(first.imported, 1);
    assert_eq!(second.imported, 1);
    assert_eq!(s.count_units().unwrap(), 1);
}

#[test]
fn edges_cross_reference_validates() {
    let s = mkstore();
    s.add_unit(&mkunit("note a", "", "a.md")).unwrap();
    s.add_unit(&mkunit("note b", "", "b.md")).unwrap();
    add_edge(&s, "a.md", "b.md", "refs", 1.0).unwrap();
    let out = list_outgoing(&s, "a.md").unwrap();
    assert_eq!(out.len(), 1);
}

#[test]
fn fts5_respects_limit() {
    let s = mkstore();
    for i in 0..25 {
        let t = format!("rust note {i}");
        s.add_unit(&mkunit(&t, "rust rust rust", &format!("n{i}.md"))).unwrap();
    }
    let hits = fts_search(&s, "rust", 5).unwrap();
    assert_eq!(hits.len(), 5);
}
