//! Filtering `list_by_type` returns only rows of the requested block_type.

use kei_registry::{list_by_type, open_db, register, BlockType};
use tempfile::tempdir;

#[test]
fn list_by_type_counts_match() {
    let tmp = tempdir().unwrap();
    let conn = open_db(tmp.path().join("r.sqlite")).unwrap();

    register(&conn, BlockType::Primitive, "p1", "/tmp/p1", b"prim body", "fs").unwrap();
    register(&conn, BlockType::Primitive, "p2", "/tmp/p2", b"prim body 2", "fs").unwrap();
    register(&conn, BlockType::Skill, "s1", "/tmp/s1", b"skill body", "md").unwrap();
    register(&conn, BlockType::Rule, "r1", "/tmp/r1", b"rule body", "md").unwrap();
    register(&conn, BlockType::Rule, "r2", "/tmp/r2", b"rule body 2", "md").unwrap();
    register(&conn, BlockType::Rule, "r3", "/tmp/r3", b"rule body 3", "md").unwrap();

    let prims = list_by_type(&conn, BlockType::Primitive).unwrap();
    let skills = list_by_type(&conn, BlockType::Skill).unwrap();
    let rules = list_by_type(&conn, BlockType::Rule).unwrap();
    let hooks = list_by_type(&conn, BlockType::Hook).unwrap();

    assert_eq!(prims.len(), 2);
    assert_eq!(skills.len(), 1);
    assert_eq!(rules.len(), 3);
    assert_eq!(hooks.len(), 0, "no hooks registered");

    for r in rules {
        assert_eq!(r.block_type, BlockType::Rule);
    }
}

#[test]
fn list_by_type_returns_only_matching_type() {
    let tmp = tempdir().unwrap();
    let conn = open_db(tmp.path().join("r.sqlite")).unwrap();
    register(&conn, BlockType::Skill, "x", "/tmp/x", b"x", "md").unwrap();
    register(&conn, BlockType::Rule, "y", "/tmp/y", b"y", "md").unwrap();

    let skills = list_by_type(&conn, BlockType::Skill).unwrap();
    assert_eq!(skills.len(), 1);
    assert_eq!(skills[0].block_type, BlockType::Skill);
}
