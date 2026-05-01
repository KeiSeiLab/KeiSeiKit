//! Shared fixture builders for golden tests.
//!
//! Two synthetic agents:
//!  - agent-a: 'done', 3 successful tool events (Read x2, Bash x1)
//!  - agent-b: 'failed', 1 failed tool event (Bash)
//!
//! Their started_ts are 1700000100 and 1700000300 respectively, so any
//! cutoff at or below 1700000100 includes both.

use rusqlite::{params, Connection};
use std::fs;
use std::path::Path;

/// Create a minimal `agents`-schema sqlite at `path` with two synthetic
/// rows. We deliberately do NOT call `kei_ledger::migrate` — this keeps
/// the test independent of unrelated v3..v7 columns we don't query.
pub fn build_ledger(path: &Path) {
    let conn = Connection::open(path).unwrap();
    conn.execute_batch(
        "CREATE TABLE agents (
            id TEXT PRIMARY KEY,
            branch TEXT NOT NULL,
            parent_branch TEXT,
            spec_sha TEXT NOT NULL,
            status TEXT NOT NULL,
            started_ts INTEGER NOT NULL,
            finished_ts INTEGER,
            summary TEXT,
            worktree_path TEXT,
            dna TEXT
        );",
    )
    .unwrap();
    insert_agent(&conn, "agent-a", "feat/a", "sha-a", "done",
                 1700000100, 1700000200, "completed task A", "dna-a");
    insert_agent(&conn, "agent-b", "feat/b", "sha-b", "failed",
                 1700000300, 1700000400, "crashed task B", "dna-b");
}

#[allow(clippy::too_many_arguments)]
fn insert_agent(
    conn: &Connection,
    id: &str,
    branch: &str,
    sha: &str,
    status: &str,
    started: i64,
    finished: i64,
    summary: &str,
    dna: &str,
) {
    conn.execute(
        "INSERT INTO agents
         (id, branch, spec_sha, status, started_ts, finished_ts, summary, dna)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![id, branch, sha, status, started, finished, summary, dna],
    )
    .unwrap();
}

/// Create a minimal `events`-schema sqlite at `path` with 4 events
/// pinned to the two test agents by `session_id`.
pub fn build_memory(path: &Path) {
    let conn = Connection::open(path).unwrap();
    conn.execute_batch(
        "CREATE TABLE events (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            session_id TEXT NOT NULL,
            ts INTEGER NOT NULL,
            kind TEXT NOT NULL,
            tool TEXT,
            file_path TEXT,
            is_error INTEGER NOT NULL DEFAULT 0
        );",
    )
    .unwrap();
    let rows = [
        ("agent-a", 1700000110_i64, "Read", 0_i64),
        ("agent-a", 1700000120, "Read", 0),
        ("agent-a", 1700000130, "Bash", 0),
        ("agent-b", 1700000310, "Bash", 1),
    ];
    for (sid, ts, tool, err) in rows {
        conn.execute(
            "INSERT INTO events (session_id, ts, kind, tool, is_error)
             VALUES (?1, ?2, 'tool', ?3, ?4)",
            params![sid, ts, tool, err],
        )
        .unwrap();
    }
}

/// Create `.claude/agents/{agent-a,agent-b}/{spec.md,chatlog.md}` under
/// `repo_root` with deterministic content.
pub fn build_artefacts(repo_root: &Path) {
    for (id, spec, chat) in [
        ("agent-a", "do the A task", "wrote A files"),
        ("agent-b", "do the B task", "B crashed"),
    ] {
        let dir = repo_root.join(".claude").join("agents").join(id);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("spec.md"), spec).unwrap();
        fs::write(dir.join("chatlog.md"), chat).unwrap();
    }
}
