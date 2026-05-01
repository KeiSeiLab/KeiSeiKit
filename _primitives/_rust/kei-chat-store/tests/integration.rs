use kei_chat_store::search::search;
use kei_chat_store::sessions::{archive_session, get_session, save_message, start_session, ChatMessage};
use kei_chat_store::stats::stats;
use kei_chat_store::Store;

fn mk() -> Store { Store::open_memory().unwrap() }

#[test]
fn save_and_retrieve() {
    let s = mk();
    let sid = start_session(&s, "demo", "t", "claude-opus-4").unwrap();
    save_message(&s, &ChatMessage {
        session_id: sid.clone(), role: "user".into(),
        content: "hello world".into(), tokens_in: 3, tokens_out: 0, cost: 0.001,
        ..Default::default()
    }).unwrap();
    let sess = get_session(&s, &sid).unwrap().unwrap();
    assert_eq!(sess.message_count, 1);
    assert_eq!(sess.total_tokens, 3);
}

#[test]
fn fts_search_finds_message() {
    let s = mk();
    let sid = start_session(&s, "demo", "", "").unwrap();
    save_message(&s, &ChatMessage {
        session_id: sid, role: "user".into(),
        content: "rust async tokio bench".into(),
        ..Default::default()
    }).unwrap();
    let hits = search(&s, "tokio", 10).unwrap();
    assert_eq!(hits.len(), 1);
}

#[test]
fn archive_session_works() {
    let s = mk();
    let sid = start_session(&s, "p", "", "").unwrap();
    archive_session(&s, &sid).unwrap();
    let sess = get_session(&s, &sid).unwrap().unwrap();
    assert_eq!(sess.status, "archived");
}

#[test]
fn engine_migration_parity_smoke() {
    // Layer-A convergence parity: fresh session opens cleanly through
    // kei_entity_store::Store + CHAT_SCHEMA, start_session returns a
    // UUID that get_session can retrieve with id > 0 chars and the
    // engine-generated chat_messages table is writeable.
    let s = mk();
    let sid = start_session(&s, "conv-layer-a", "smoke", "claude-opus-4").unwrap();
    assert!(!sid.is_empty(), "session id should be a non-empty UUID");
    let mid = save_message(&s, &ChatMessage {
        session_id: sid.clone(), role: "user".into(),
        content: "migration-parity probe".into(),
        tokens_in: 1, tokens_out: 0, cost: 0.0,
        ..Default::default()
    }).unwrap();
    assert!(mid > 0, "engine-backed message id must be > 0");
    let sess = get_session(&s, &sid).unwrap().unwrap();
    assert_eq!(sess.id, sid);
    assert_eq!(sess.message_count, 1);
}

#[test]
fn cost_roundtrips_via_search() {
    // Wave-8 re-migration: cost is re-instated as engine-managed
    // RealDefault column. The value written via save_message must be
    // visible on the ChatMessage returned from search (no longer 0.0).
    let s = mk();
    let sid = start_session(&s, "demo", "", "").unwrap();
    save_message(&s, &ChatMessage {
        session_id: sid, role: "user".into(),
        content: "rust async tokio bench cost-marker".into(),
        tokens_in: 1, tokens_out: 1, cost: 0.00777,
        ..Default::default()
    }).unwrap();
    let hits = search(&s, "cost-marker", 10).unwrap();
    assert_eq!(hits.len(), 1);
    assert!((hits[0].cost - 0.00777).abs() < 1e-9,
        "cost should round-trip via engine RealDefault column, got {}",
        hits[0].cost);
}

#[test]
fn stats_aggregates() {
    let s = mk();
    let sid = start_session(&s, "p", "", "").unwrap();
    for _ in 0..3 {
        save_message(&s, &ChatMessage {
            session_id: sid.clone(), role: "user".into(),
            content: "x".into(), tokens_in: 5, tokens_out: 5, cost: 0.01,
            ..Default::default()
        }).unwrap();
    }
    let st = stats(&s).unwrap();
    assert_eq!(st.total_sessions, 1);
    assert_eq!(st.total_messages, 3);
    assert_eq!(st.total_tokens, 30);
}
