//! Ledger ops — fork / done / fail / list / tree / validate.
//! Constructor Pattern: each public fn <30 LOC. rusqlite-backed, one file per caller.

use crate::error::MAX_TREE_DEPTH;
pub use crate::error::LedgerError;
pub use crate::row::AgentRow;
use crate::row::{row_to_agent, SELECT_COLS};
use crate::schema::{migrate, MAX_BRANCH_LEN, REQUIRED_ARTEFACTS};
use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension, Result as SqlResult};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Open or create the ledger file and run migrations.
///
/// Returns `LedgerError` rather than raw `rusqlite::Error` because v5 can
/// surface `DnaMigrationBlocked` (pre-existing duplicate DNAs). Legacy
/// rusqlite errors are wrapped in `LedgerError::Sql` via `From`.
pub fn open(path: &Path) -> Result<Connection, LedgerError> {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let conn = Connection::open(path)?;
    migrate(&conn)?;
    Ok(conn)
}

/// Cap branch / parent_branch length (audit L1). Schema triggers mirror this.
fn check_branch_lens(branch: &str, parent: Option<&str>) -> Result<(), LedgerError> {
    if branch.len() > MAX_BRANCH_LEN {
        return Err(LedgerError::BranchTooLong { field: "branch", len: branch.len() });
    }
    if let Some(p) = parent {
        if p.len() > MAX_BRANCH_LEN {
            return Err(LedgerError::BranchTooLong { field: "parent_branch", len: p.len() });
        }
    }
    Ok(())
}

/// Insert running-agent row. Errors on duplicate id or branch > MAX_BRANCH_LEN.
///
/// Surfaces `LedgerError::DnaCollision { dna }` when a non-NULL DNA already
/// exists in the ledger (v5 UNIQUE index). Caller regenerates DNA with a
/// fresh nonce and retries — the ledger never silently dedupes.
#[allow(clippy::too_many_arguments)]
pub fn fork(
    conn: &Connection,
    id: &str,
    branch: &str,
    parent: Option<&str>,
    spec_sha: &str,
    worktree: Option<&str>,
    dna: Option<&str>,
    creator_id: Option<&str>,
    fork_parent_id: Option<&str>,
) -> Result<(), LedgerError> {
    check_branch_lens(branch, parent)?;
    let now = Utc::now().timestamp();
    let res = conn.execute(
        "INSERT INTO agents
         (id, branch, parent_branch, spec_sha, status, started_ts,
          worktree_path, dna, creator_id, fork_parent_id)
         VALUES (?1, ?2, ?3, ?4, 'running', ?5, ?6, ?7, ?8, ?9)",
        params![id, branch, parent, spec_sha, now, worktree, dna, creator_id, fork_parent_id],
    );
    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(classify_insert_error(e, dna)),
    }
}

/// Map rusqlite unique-constraint failures to typed variants. `agents.dna`
/// violations become `DnaCollision`; everything else (id collision, trigger
/// abort, etc.) flows through as `LedgerError::Sql`.
fn classify_insert_error(e: rusqlite::Error, dna: Option<&str>) -> LedgerError {
    let msg = e.to_string();
    if msg.contains("agents.dna") || msg.contains("idx_agents_dna_unique") {
        LedgerError::DnaCollision {
            dna: dna.unwrap_or_default().to_string(),
        }
    } else {
        LedgerError::Sql(e)
    }
}

/// Mark a running agent as done. No-op if already in terminal state.
pub fn done(conn: &Connection, id: &str, summary: &str) -> SqlResult<usize> {
    let now = Utc::now().timestamp();
    conn.execute(
        "UPDATE agents SET status='done', finished_ts=?1, summary=?2
         WHERE id=?3 AND status='running'",
        params![now, summary, id],
    )
}

/// Mark a running agent as failed with reason.
pub fn fail(conn: &Connection, id: &str, reason: &str) -> SqlResult<usize> {
    let now = Utc::now().timestamp();
    conn.execute(
        "UPDATE agents SET status='failed', finished_ts=?1, summary=?2
         WHERE id=?3 AND status='running'",
        params![now, reason, id],
    )
}

/// Mark an agent as merged (post-ceremony bookkeeping).
pub fn merged(conn: &Connection, id: &str) -> SqlResult<usize> {
    let now = Utc::now().timestamp();
    conn.execute(
        "UPDATE agents SET status='merged', finished_ts=COALESCE(finished_ts, ?1)
         WHERE id=?2 AND status IN ('done','failed')",
        params![now, id],
    )
}

/// List all agents, optionally filtered by status.
pub fn list(conn: &Connection, status: Option<&str>) -> SqlResult<Vec<AgentRow>> {
    let (sql, bound): (String, Vec<String>) = match status {
        Some(s) => (
            format!("SELECT {SELECT_COLS} FROM agents WHERE status = ?1 ORDER BY started_ts DESC"),
            vec![s.to_string()],
        ),
        None => (
            format!("SELECT {SELECT_COLS} FROM agents ORDER BY started_ts DESC"),
            vec![],
        ),
    };
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt
        .query_map(rusqlite::params_from_iter(bound.iter()), row_to_agent)?
        .collect::<SqlResult<Vec<_>>>()?;
    Ok(rows)
}

fn by_id(conn: &Connection, id: &str) -> SqlResult<Option<AgentRow>> {
    let sql = format!("SELECT {SELECT_COLS} FROM agents WHERE id = ?1");
    conn.query_row(&sql, params![id], row_to_agent).optional()
}

/// Fetch immediate children of a given parent_branch.
fn children_of(conn: &Connection, parent_branch: &str) -> SqlResult<Vec<AgentRow>> {
    let sql = format!(
        "SELECT {SELECT_COLS} FROM agents WHERE parent_branch = ?1 ORDER BY started_ts ASC"
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt
        .query_map(params![parent_branch], row_to_agent)?
        .collect::<SqlResult<Vec<_>>>()?;
    Ok(rows)
}

/// BFS from `root_id` to all descendants, root-first. Cycle-safe via `visited`;
/// aborts after `MAX_TREE_DEPTH` iterations (audit S2 runaway-data guard).
pub fn tree(conn: &Connection, root_id: &str) -> Result<Vec<AgentRow>, LedgerError> {
    let root = match by_id(conn, root_id)? {
        Some(r) => r,
        None => return Ok(vec![]),
    };
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(root.branch.clone());
    let mut out = vec![root.clone()];
    let mut frontier: Vec<String> = vec![root.branch];
    let mut steps = 0usize;
    while let Some(parent_branch) = frontier.pop() {
        steps += 1;
        if steps > MAX_TREE_DEPTH {
            return Err(LedgerError::MaxDepthExceeded);
        }
        for k in children_of(conn, &parent_branch)? {
            if visited.insert(k.branch.clone()) {
                frontier.push(k.branch.clone());
                out.push(k);
            }
        }
    }
    Ok(out)
}

/// Verify all 6 required artefacts exist under `.claude/agents/<id>/`.
/// Returns list of missing filenames (empty = OK).
pub fn validate(repo_root: &Path, agent_id: &str) -> Vec<String> {
    let mut base: PathBuf = repo_root.to_path_buf();
    base.push(".claude");
    base.push("agents");
    base.push(agent_id);
    REQUIRED_ARTEFACTS
        .iter()
        .filter(|a| !base.join(a).is_file())
        .map(|a| a.to_string())
        .collect()
}
