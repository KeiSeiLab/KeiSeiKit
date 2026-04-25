use kei_auth::schema::open_memory;
use kei_auth::scopes::Scope;
use kei_auth::tokens::{issue, revoke, verify};

const KEY: &[u8] = b"test-key-must-not-be-used-in-production";

#[test]
fn issue_and_verify() {
    let conn = open_memory().unwrap();
    let tok = issue(&conn, "alice", "demo", Scope::Write, 3600, KEY).unwrap();
    let out = verify(&conn, &tok, KEY).unwrap();
    assert_eq!(out.user_id, "alice");
    assert_eq!(out.project, "demo");
    assert_eq!(out.scope, Scope::Write);
}

#[test]
fn revoke_blocks_verify() {
    let conn = open_memory().unwrap();
    let tok = issue(&conn, "bob", "x", Scope::Read, 3600, KEY).unwrap();
    assert_eq!(revoke(&conn, &tok).unwrap(), 1);
    assert!(verify(&conn, &tok, KEY).is_err());
}

#[test]
fn expired_token_rejected() {
    let conn = open_memory().unwrap();
    let tok = issue(&conn, "carol", "x", Scope::Read, -10, KEY).unwrap();
    let err = verify(&conn, &tok, KEY);
    assert!(err.is_err(), "expired must fail");
}

#[test]
fn scope_check_admin_implies_write() {
    assert!(Scope::Admin.allows(Scope::Write));
    assert!(Scope::Admin.allows(Scope::Read));
    assert!(Scope::Write.allows(Scope::Read));
    assert!(!Scope::Read.allows(Scope::Write));
    assert!(!Scope::Write.allows(Scope::Admin));
}

#[test]
fn tampered_token_rejected() {
    let conn = open_memory().unwrap();
    let tok = issue(&conn, "dave", "x", Scope::Read, 3600, KEY).unwrap();
    let mut chars: Vec<char> = tok.chars().collect();
    // flip one char in the signature
    let last = chars.len() - 1;
    chars[last] = if chars[last] == 'A' { 'B' } else { 'A' };
    let tampered: String = chars.into_iter().collect();
    assert!(verify(&conn, &tampered, KEY).is_err());
}
