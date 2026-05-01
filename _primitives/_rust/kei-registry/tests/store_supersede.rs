//! Body change at the same path supersedes the prior row.

use kei_registry::{find_by_path, get, list, open_db, register, BlockType};
use tempfile::tempdir;

#[test]
fn body_change_creates_supersede_chain() {
    let tmp = tempdir().unwrap();
    let conn = open_db(tmp.path().join("registry.sqlite")).unwrap();

    let path = "/tmp/fixture/movable";
    let v1 = register(&conn, BlockType::Atom, "x", path, b"v1 body", "md").unwrap();
    let v2 = register(&conn, BlockType::Atom, "x", path, b"v2 body different", "md").unwrap();

    assert_ne!(v1.dna, v2.dna, "new body sha → new DNA");
    assert_ne!(v1.id, v2.id, "new row inserted");

    // Old row is now superseded; pointer matches new DNA.
    let v1_after = get(&conn, v1.id).unwrap().expect("v1 still exists");
    assert_eq!(v1_after.superseded_by.as_deref(), Some(v2.dna.as_str()));

    // Active query at the path returns the NEW row.
    let active = find_by_path(&conn, path).unwrap().expect("active row exists");
    assert_eq!(active.id, v2.id);

    // Two rows total; default list (active) returns one.
    let total: i64 = conn
        .query_row("SELECT COUNT(*) FROM blocks", [], |r| r.get(0))
        .unwrap();
    assert_eq!(total, 2);
    let active_only = list(&conn, false, 100).unwrap();
    assert_eq!(active_only.len(), 1);
    let with_superseded = list(&conn, true, 100).unwrap();
    assert_eq!(with_superseded.len(), 2);
}
