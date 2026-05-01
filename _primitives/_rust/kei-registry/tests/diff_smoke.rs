//! `diff_blocks` walks all four facets and reports differences.

use kei_registry::{diff_blocks, open_db, register, BlockType};
use tempfile::tempdir;

#[test]
fn identical_blocks_have_no_differs() {
    let tmp = tempdir().unwrap();
    let conn = open_db(tmp.path().join("r.sqlite")).unwrap();
    let a = register(&conn, BlockType::Atom, "x", "/p1", b"body", "md").unwrap();
    let b = register(&conn, BlockType::Atom, "x", "/p1", b"body", "md").unwrap();
    let d = diff_blocks(&a, &b);
    assert!(d.differs.is_empty(), "identical re-register → no differs");
    assert_eq!(d.identical.len(), 4, "all 4 facets identical");
}

#[test]
fn different_paths_diff_scope_sha() {
    let tmp = tempdir().unwrap();
    let conn = open_db(tmp.path().join("r.sqlite")).unwrap();
    let a = register(&conn, BlockType::Atom, "x", "/p1", b"body", "md").unwrap();
    let b = register(&conn, BlockType::Atom, "x", "/p2", b"body", "md").unwrap();
    let d = diff_blocks(&a, &b);
    assert!(
        d.differs.iter().any(|s| s == "scope_sha"),
        "scope_sha must be flagged as differing for different paths"
    );
    assert!(d.identical.iter().any(|s| s == "body_sha"));
    assert!(d.identical.iter().any(|s| s == "block_type"));
}

#[test]
fn different_block_types_diff_block_type_facet() {
    let tmp = tempdir().unwrap();
    let conn = open_db(tmp.path().join("r.sqlite")).unwrap();
    let a = register(&conn, BlockType::Atom, "x", "/p1", b"body", "md").unwrap();
    let b = register(&conn, BlockType::Rule, "y", "/p2", b"body2", "md").unwrap();
    let d = diff_blocks(&a, &b);
    assert!(d.differs.iter().any(|s| s == "block_type"));
}
