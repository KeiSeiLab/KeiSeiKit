//! Conversational recall — "have we discussed this before?"
//!
//! Thin adapter over `kei_dna_index::precedent`. Hashes a task body with
//! SHA-256, truncates to the first 4 bytes (8 hex chars — matches the DNA
//! `body_sha` width SSoT in `kei_shared::dna`), then asks the ledger for
//! past agents whose DNA carries the same body_sha.
//!
//! Scope: reads the `agents` table on the supplied `Connection`. No writes,
//! no schema mutation. Caller decides whether the connection points at the
//! real ledger or a test fixture.

use kei_dna_index::precedent;
use rusqlite::{params, Connection};
use sha2::{Digest, Sha256};

/// One past agent whose DNA body_sha matches the current task body.
#[derive(Debug, Clone)]
pub struct RecallHit {
    pub past_agent_id: String,
    pub body_preview: String,
    pub timestamp: i64,
    pub status: String,
}

/// Errors surfaced by recall.
#[derive(Debug, thiserror::Error)]
pub enum RecallError {
    #[error(transparent)]
    Sql(#[from] rusqlite::Error),
    #[error(transparent)]
    DnaIndex(#[from] kei_dna_index::Error),
}

/// SHA-256 of `task_body`, truncated to the first 4 bytes rendered as
/// lowercase hex (8 chars). Matches the `body_sha` width in the DNA wire
/// format — see `kei_shared::dna`.
pub fn body_sha8(task_body: &str) -> String {
    let mut h = Sha256::new();
    h.update(task_body.as_bytes());
    let digest = h.finalize();
    hex_lower(&digest[..4])
}

fn hex_lower(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

/// First 80 characters of `task_body`, respecting UTF-8 char boundaries.
fn preview(task_body: &str) -> String {
    task_body.chars().take(80).collect()
}

/// Fetch `started_ts` for a given agent_id. Returns 0 when the row is gone
/// (shouldn't happen inside a single transaction but we degrade gracefully).
fn fetch_started_ts(conn: &Connection, agent_id: &str) -> Result<i64, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT started_ts FROM agents WHERE id = ?1")?;
    let ts: Option<i64> = stmt
        .query_row(params![agent_id], |r| r.get::<_, i64>(0))
        .ok();
    Ok(ts.unwrap_or(0))
}

/// Find up to `limit` past agents whose DNA body_sha matches the hash of
/// `task_body`. Results are sorted newest-first by `started_ts`.
pub fn recall_similar(
    conn: &Connection,
    task_body: &str,
    limit: usize,
) -> Result<Vec<RecallHit>, RecallError> {
    let sha = body_sha8(task_body);
    let matches = precedent(conn, &sha, None)?;
    let prev = preview(task_body);
    let mut hits: Vec<RecallHit> = matches
        .into_iter()
        .map(|r| {
            let ts = fetch_started_ts(conn, &r.agent_id).unwrap_or(0);
            RecallHit {
                past_agent_id: r.agent_id,
                body_preview: prev.clone(),
                timestamp: ts,
                status: r.status,
            }
        })
        .collect();
    hits.sort_by_key(|h| std::cmp::Reverse(h.timestamp));
    if limit > 0 && hits.len() > limit {
        hits.truncate(limit);
    }
    Ok(hits)
}
