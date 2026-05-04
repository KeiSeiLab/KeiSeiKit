//! Integration tests for `kei_comments::CommentStore`.
//! Covers: roundtrip, threading, delete authorisation, reactions, body cap.

use kei_comments::{CommentStore, MAX_BODY_BYTES};

fn open() -> CommentStore {
    let store = CommentStore::open_memory().expect("open in-memory store");
    store.migrate().expect("migrate schema");
    store
}

#[test]
fn post_and_list_roundtrip() {
    let s = open();
    let id1 = s.post("page-a", "alice", "hello world", None).unwrap();
    let id2 = s.post("page-a", "bob", "second comment", None).unwrap();
    assert_ne!(id1, id2, "ids must differ across distinct posts");

    let listed = s.list("page-a").unwrap();
    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].author, "alice");
    assert_eq!(listed[0].body, "hello world");
    assert!(!listed[0].deleted);
    assert_eq!(listed[1].author, "bob");

    // Other page is empty.
    assert_eq!(s.list("page-b").unwrap().len(), 0);
}

#[test]
fn threading_via_parent_id() {
    let s = open();
    let root = s.post("p", "alice", "root", None).unwrap();
    let reply = s.post("p", "bob", "re: root", Some(&root)).unwrap();

    let fetched = s.get(&reply).unwrap().expect("reply exists");
    assert_eq!(fetched.parent_id.as_deref(), Some(root.as_str()));

    // Listing returns both, ordered by created_at.
    let all = s.list("p").unwrap();
    assert_eq!(all.len(), 2);
    assert_eq!(all[1].parent_id.as_deref(), Some(root.as_str()));
}

#[test]
fn delete_only_by_author() {
    let s = open();
    let id = s.post("p", "alice", "secret", None).unwrap();

    // Bob can't delete Alice's comment.
    assert!(!s.delete(&id, "bob").unwrap());
    let still_there = s.get(&id).unwrap().unwrap();
    assert!(!still_there.deleted);
    assert_eq!(still_there.body, "secret");

    // Alice can.
    assert!(s.delete(&id, "alice").unwrap());
    let gone = s.get(&id).unwrap().unwrap();
    assert!(gone.deleted);
    assert_eq!(gone.body, "");
}

#[test]
fn reactions_toggle() {
    let s = open();
    let id = s.post("p", "alice", "good post", None).unwrap();

    s.react(&id, "bob", "👍").unwrap();
    s.react(&id, "carol", "👍").unwrap();
    s.react(&id, "bob", "🎉").unwrap();
    // Idempotent — second add is no-op.
    s.react(&id, "bob", "👍").unwrap();

    let map = s.reactions(&id).unwrap();
    let thumbs = map.get("👍").unwrap();
    assert_eq!(thumbs.len(), 2);
    assert!(thumbs.contains(&"bob".to_string()));
    assert!(thumbs.contains(&"carol".to_string()));
    assert_eq!(map.get("🎉").unwrap(), &vec!["bob".to_string()]);

    // Unreact removes.
    s.unreact(&id, "bob", "👍").unwrap();
    let map2 = s.reactions(&id).unwrap();
    assert_eq!(map2.get("👍").unwrap(), &vec!["carol".to_string()]);
    // Unreact missing is no-op.
    s.unreact(&id, "nobody", "🌟").unwrap();
}

#[test]
fn body_length_cap_rejected() {
    let s = open();
    let oversized = "x".repeat(MAX_BODY_BYTES + 1);
    assert!(s.post("p", "alice", &oversized, None).is_err());
    // Exactly at cap is accepted.
    let exact = "y".repeat(MAX_BODY_BYTES);
    assert!(s.post("p", "alice", &exact, None).is_ok());
    // Empty is rejected.
    assert!(s.post("p", "alice", "   ", None).is_err());
}
