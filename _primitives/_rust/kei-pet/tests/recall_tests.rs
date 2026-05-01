//! Integration tests for `kei_pet::recall`.
//!
//! Hermetic: each test owns an in-memory SQLite Connection populated with
//! a minimal `agents` table that mirrors the subset of the real ledger
//! schema that `kei_dna_index::precedent` reads (id, dna, started_ts,
//! status).

use kei_pet::recall::{body_sha8, recall_similar};
use rusqlite::{params, Connection};

fn setup_agents_table(conn: &Connection) {
    conn.execute(
        "CREATE TABLE agents (
             id          TEXT PRIMARY KEY,
             dna         TEXT,
             started_ts  INTEGER NOT NULL,
             status      TEXT NOT NULL
         )",
        [],
    )
    .expect("create agents table");
}

fn insert_agent(
    conn: &Connection,
    id: &str,
    dna: &str,
    started_ts: i64,
    status: &str,
) {
    conn.execute(
        "INSERT INTO agents (id, dna, started_ts, status) VALUES (?1, ?2, ?3, ?4)",
        params![id, dna, started_ts, status],
    )
    .expect("insert agent");
}

fn dna_with_body_sha(role: &str, body_sha: &str, nonce: &str) -> String {
    // Format matches kei_shared::dna SSoT: `<role>::<caps>::<sha8>::<sha8>-<sha8>`
    format!("{role}::NG-FW-FD-CP::5435f821::{body_sha}-{nonce}")
}

#[test]
fn recall_returns_empty_on_fresh_db() {
    let conn = Connection::open_in_memory().unwrap();
    setup_agents_table(&conn);

    let hits = recall_similar(&conn, "any task body", 10).expect("recall ok");
    assert!(
        hits.is_empty(),
        "expected empty recall on fresh DB, got {} hits",
        hits.len()
    );
}

#[test]
fn recall_finds_same_body_sha() {
    let conn = Connection::open_in_memory().unwrap();
    setup_agents_table(&conn);

    let task_body = "refactor: extract recall primitive";
    let sha = body_sha8(task_body);
    let dna = dna_with_body_sha("code-implementer", &sha, "deadbeef");
    insert_agent(&conn, "agent-001", &dna, 1_700_000_000, "done");

    // Second agent, unrelated body → should NOT match.
    let other_sha = body_sha8("some completely different task");
    let other_dna = dna_with_body_sha("code-implementer", &other_sha, "cafebabe");
    insert_agent(&conn, "agent-002", &other_dna, 1_700_000_100, "done");

    let hits = recall_similar(&conn, task_body, 10).expect("recall ok");
    assert_eq!(hits.len(), 1, "expected exactly one recall hit");
    let hit = &hits[0];
    assert_eq!(hit.past_agent_id, "agent-001");
    assert_eq!(hit.status, "done");
    assert_eq!(hit.timestamp, 1_700_000_000);
    assert_eq!(hit.body_preview, task_body);
}

#[test]
fn recall_different_body_returns_none() {
    let conn = Connection::open_in_memory().unwrap();
    setup_agents_table(&conn);

    let other_sha = body_sha8("task A");
    let dna = dna_with_body_sha("research", &other_sha, "00112233");
    insert_agent(&conn, "agent-042", &dna, 1_700_000_050, "running");

    let hits = recall_similar(&conn, "task B — nothing in common", 10)
        .expect("recall ok");
    assert!(
        hits.is_empty(),
        "expected no hits for unrelated body, got {}",
        hits.len()
    );
}

#[test]
fn recall_sorts_newest_first_and_respects_limit() {
    let conn = Connection::open_in_memory().unwrap();
    setup_agents_table(&conn);

    let task_body = "shared task body";
    let sha = body_sha8(task_body);
    for (i, ts) in [1_000, 3_000, 2_000, 4_000].iter().enumerate() {
        let id = format!("agent-{:03}", i);
        let nonce = format!("{:08x}", i + 1);
        let dna = dna_with_body_sha("code-implementer", &sha, &nonce);
        insert_agent(&conn, &id, &dna, *ts, "done");
    }

    let hits = recall_similar(&conn, task_body, 2).expect("recall ok");
    assert_eq!(hits.len(), 2, "limit=2 should truncate");
    assert_eq!(hits[0].timestamp, 4_000, "newest first");
    assert_eq!(hits[1].timestamp, 3_000, "second newest next");
}
