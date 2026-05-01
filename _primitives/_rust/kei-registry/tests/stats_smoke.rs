//! `compute_stats` counts active vs superseded per type and globally.

use kei_registry::{compute_stats, open_db, register, BlockType};
use tempfile::tempdir;

#[test]
fn stats_match_after_population() {
    let tmp = tempdir().unwrap();
    let conn = open_db(tmp.path().join("r.sqlite")).unwrap();
    register(&conn, BlockType::Primitive, "p1", "/p1", b"a", "fs").unwrap();
    register(&conn, BlockType::Primitive, "p2", "/p2", b"b", "fs").unwrap();
    register(&conn, BlockType::Rule, "r1", "/r1", b"x", "md").unwrap();
    register(&conn, BlockType::Skill, "s1", "/s1", b"y", "md").unwrap();
    register(&conn, BlockType::Hook, "h1", "/h1", b"z", "shell").unwrap();
    register(&conn, BlockType::Atom, "a1", "/a1", b"q", "md").unwrap();

    let s = compute_stats(&conn).unwrap();
    assert_eq!(s.total_active, 6);
    assert_eq!(s.total_superseded, 0);
    assert_eq!(s.by_type["primitive"].active, 2);
    assert_eq!(s.by_type["rule"].active, 1);
    assert_eq!(s.by_type["skill"].active, 1);
    assert_eq!(s.by_type["hook"].active, 1);
    assert_eq!(s.by_type["atom"].active, 1);
    assert_eq!(s.schema_version, kei_registry::SCHEMA_VERSION);
}

#[test]
fn stats_reflect_supersede() {
    let tmp = tempdir().unwrap();
    let conn = open_db(tmp.path().join("r.sqlite")).unwrap();
    register(&conn, BlockType::Atom, "a", "/p", b"v1", "md").unwrap();
    register(&conn, BlockType::Atom, "a", "/p", b"v2", "md").unwrap();

    let s = compute_stats(&conn).unwrap();
    assert_eq!(s.total_active, 1);
    assert_eq!(s.total_superseded, 1);
    assert_eq!(s.by_type["atom"].active, 1);
    assert_eq!(s.by_type["atom"].superseded, 1);
}
