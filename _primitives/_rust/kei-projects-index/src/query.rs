//! Read-only query helpers used by the CLI to render the `list` and
//! `get` outputs. The library `index::ProjectRow` shape is the SSoT.

use crate::row::ProjectRow;
use rusqlite::{params, Connection, Result, Row};

const SELECT_COLS: &str = "
path, name, has_git, branch, dirty, ahead, behind,
last_commit_sha, last_commit_msg, last_commit_ts,
has_claude_md, has_decisions_md, has_runbook_md, has_readme,
sqlite_count, last_indexed_ts
";

fn row_to_project(row: &Row<'_>) -> Result<ProjectRow> {
    Ok(ProjectRow {
        path: row.get(0)?,
        name: row.get(1)?,
        has_git: row.get::<_, i64>(2)? != 0,
        branch: row.get(3)?,
        dirty: row.get::<_, i64>(4)? != 0,
        ahead: row.get(5)?,
        behind: row.get(6)?,
        last_commit_sha: row.get(7)?,
        last_commit_msg: row.get(8)?,
        last_commit_ts: row.get(9)?,
        has_claude_md: row.get::<_, i64>(10)? != 0,
        has_decisions_md: row.get::<_, i64>(11)? != 0,
        has_runbook_md: row.get::<_, i64>(12)? != 0,
        has_readme: row.get::<_, i64>(13)? != 0,
        sqlite_count: row.get(14)?,
        last_indexed_ts: row.get(15)?,
    })
}

/// All rows ordered most-recently-committed first (NULLS LAST). The
/// dashboard's first call paints the top of the table without further
/// sort logic on the consumer side.
pub fn list_all(conn: &Connection) -> Result<Vec<ProjectRow>> {
    let sql = format!(
        "SELECT {SELECT_COLS} FROM projects
         ORDER BY last_commit_ts DESC NULLS LAST, name ASC"
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt
        .query_map([], row_to_project)?
        .collect::<Result<Vec<_>>>()?;
    Ok(rows)
}

/// Fetch a single row by primary key (`path`). Returns `Ok(None)` if no
/// row matches — callers translate to a CLI 1-exit on the caller side.
pub fn get_one(conn: &Connection, path: &str) -> Result<Option<ProjectRow>> {
    let sql = format!("SELECT {SELECT_COLS} FROM projects WHERE path = ?1");
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query(params![path])?;
    match rows.next()? {
        Some(r) => Ok(Some(row_to_project(r)?)),
        None => Ok(None),
    }
}
