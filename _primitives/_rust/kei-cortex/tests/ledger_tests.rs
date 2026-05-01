//! `GET /api/v1/cortex/ledger/recent` — integration test with a seeded DB.
//!
//! Uses the minimal v1 schema subset that the cortex query depends on.
//! Not linked against `kei-ledger` to keep the test hermetic.

mod common;

use common::{async_client, spawn};
use reqwest::header;
use rusqlite::{params, Connection};
use serde_json::Value;

/// Create the subset of the kei-ledger v1 `agents` table we need + seed rows.
fn seed_ledger(path: &std::path::Path, rows: &[(i64, &str)]) {
    let conn = Connection::open(path).unwrap();
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS agents (
            id TEXT PRIMARY KEY,
            branch TEXT NOT NULL,
            parent_branch TEXT,
            spec_sha TEXT NOT NULL,
            status TEXT NOT NULL CHECK (status IN ('running','done','failed','merged','rejected')),
            started_ts INTEGER NOT NULL,
            finished_ts INTEGER,
            summary TEXT,
            worktree_path TEXT
        )",
    )
    .unwrap();
    for (started_ts, id) in rows {
        conn.execute(
            "INSERT INTO agents (id, branch, spec_sha, status, started_ts)
             VALUES (?1, ?2, ?3, 'running', ?4)",
            params![id, format!("feat/{id}"), "sha-test", started_ts],
        )
        .unwrap();
    }
}

#[tokio::test]
async fn ledger_recent_respects_limit_param() {
    let srv = spawn().await;
    seed_ledger(
        &srv.config.ledger_path,
        &[
            (1_000, "agent-a"),
            (2_000, "agent-b"),
            (3_000, "agent-c"),
            (4_000, "agent-d"),
            (5_000, "agent-e"),
        ],
    );
    let resp = async_client()
        .get(format!(
            "{}/api/v1/cortex/ledger/recent?limit=2",
            srv.base_url
        ))
        .header(header::AUTHORIZATION, format!("Bearer {}", srv.token))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    let rows = body["rows"].as_array().unwrap();
    assert_eq!(rows.len(), 2);
    // Newest first, order by started_ts DESC.
    assert_eq!(rows[0]["id"], "agent-e");
    assert_eq!(rows[1]["id"], "agent-d");
}

#[tokio::test]
async fn ledger_recent_returns_empty_when_db_absent() {
    let srv = spawn().await;
    // Do NOT seed — the DB file does not exist.
    let resp = async_client()
        .get(format!("{}/api/v1/cortex/ledger/recent", srv.base_url))
        .header(header::AUTHORIZATION, format!("Bearer {}", srv.token))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert!(body["rows"].as_array().unwrap().is_empty());
}
