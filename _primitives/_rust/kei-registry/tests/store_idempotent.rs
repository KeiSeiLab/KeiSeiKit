//! Re-registering the same (path, body) returns the existing DNA. Single
//! row in the table; the original `created` timestamp is preserved.

use kei_registry::{open_db, register, BlockType};
use tempfile::tempdir;

#[test]
fn re_register_same_body_returns_same_dna() {
    let tmp = tempdir().unwrap();
    let db_path = tmp.path().join("registry.sqlite");
    let conn = open_db(&db_path).unwrap();

    let body = b"the body bytes never change";
    let path = "/tmp/fixture/foo";
    let first = register(&conn, BlockType::Atom, "foo", path, body, "md").unwrap();
    let second = register(&conn, BlockType::Atom, "foo", path, body, "md").unwrap();

    assert_eq!(first.dna, second.dna, "DNA must be identical on re-register");
    assert_eq!(first.id, second.id, "row id must be preserved");
    assert_eq!(first.nonce, second.nonce, "nonce must be stable");

    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM blocks", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 1, "exactly one row after re-register");
}

#[test]
fn re_register_different_caps_same_body_still_idempotent() {
    // The idempotency rule keys on (path, body_sha) only — caps drift on a
    // re-register call should NOT spawn a new row when the body is byte
    // identical. This protects against scanner-config churn.
    let tmp = tempdir().unwrap();
    let conn = open_db(tmp.path().join("r.sqlite")).unwrap();
    let body = b"x";
    let path = "/some/path";
    let first = register(&conn, BlockType::Atom, "x", path, body, "md").unwrap();
    let second = register(&conn, BlockType::Atom, "x", path, body, "md,extra").unwrap();
    assert_eq!(first.dna, second.dna);
}
