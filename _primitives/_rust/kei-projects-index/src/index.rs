//! Index orchestrator: walk → git_state → docs → sqlite_scan → upsert.
//!
//! Constructor Pattern: one cube = the "rebuild" pipeline. Pure glue —
//! all data extraction lives in sibling modules. Idempotent: rebuilding
//! against the same filesystem yields the same DB state (rows are
//! upserted by primary key `path`).

use crate::docs::{detect_docs, DocsState};
use crate::git_state::{detect_git_state, GitState};
use crate::schema::init;
use crate::sqlite_scan::count_sqlite_files;
use crate::walk::{walk_projects_root, ProjectEntry};
use anyhow::{Context, Result};
use chrono::Utc;
use rusqlite::{params, Connection};
use std::path::Path;

// Re-export `ProjectRow` at the orchestrator path so existing callers
// (`kei_projects_index::index::ProjectRow`) keep working alongside the
// canonical `kei_projects_index::row::ProjectRow` location.
pub use crate::row::ProjectRow;

/// Build a `ProjectRow` from the four data sources for one project.
fn build_row(entry: &ProjectEntry, now_ts: i64) -> ProjectRow {
    let git: Option<GitState> = if entry.has_git {
        detect_git_state(&entry.path)
    } else {
        None
    };
    let docs: DocsState = detect_docs(&entry.path);
    let sqlite_count = count_sqlite_files(&entry.path) as i64;
    ProjectRow {
        path: entry.path.to_string_lossy().to_string(),
        name: entry.name.clone(),
        has_git: entry.has_git,
        branch: git.as_ref().and_then(|g| g.branch.clone()),
        dirty: git.as_ref().map(|g| g.dirty).unwrap_or(false),
        ahead: git.as_ref().map(|g| g.ahead as i64).unwrap_or(0),
        behind: git.as_ref().map(|g| g.behind as i64).unwrap_or(0),
        last_commit_sha: git.as_ref().and_then(|g| g.last_commit_sha.clone()),
        last_commit_msg: git.as_ref().and_then(|g| g.last_commit_msg.clone()),
        last_commit_ts: git.as_ref().and_then(|g| g.last_commit_ts),
        has_claude_md: docs.has_claude_md,
        has_decisions_md: docs.has_decisions_md,
        has_runbook_md: docs.has_runbook_md,
        has_readme: docs.has_readme,
        sqlite_count,
        last_indexed_ts: now_ts,
    }
}

const UPSERT_SQL: &str = "INSERT OR REPLACE INTO projects
    (path, name, has_git, branch, dirty, ahead, behind,
     last_commit_sha, last_commit_msg, last_commit_ts,
     has_claude_md, has_decisions_md, has_runbook_md, has_readme,
     sqlite_count, last_indexed_ts)
    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,
            ?11, ?12, ?13, ?14, ?15, ?16)";

/// Upsert one row keyed on PRIMARY KEY (`path`).
fn upsert_row(conn: &Connection, row: &ProjectRow) -> Result<()> {
    conn.execute(
        UPSERT_SQL,
        params![
            row.path, row.name, row.has_git as i64, row.branch,
            row.dirty as i64, row.ahead, row.behind,
            row.last_commit_sha, row.last_commit_msg, row.last_commit_ts,
            row.has_claude_md as i64, row.has_decisions_md as i64,
            row.has_runbook_md as i64, row.has_readme as i64,
            row.sqlite_count, row.last_indexed_ts,
        ],
    )
    .context("upsert into projects")?;
    Ok(())
}

/// Open `db_path` (creating parent dir) and apply the schema.
fn open_db(db_path: &Path) -> Result<Connection> {
    if let Some(parent) = db_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let conn = Connection::open(db_path).context("open projects-index sqlite")?;
    init(&conn).context("apply schema")?;
    Ok(conn)
}

/// Rebuild the index from `projects_root` into `db_path`.
///
/// 1. Open / create DB and apply schema.
/// 2. Walk top-level dirs of `projects_root`.
/// 3. Extract git_state + docs + sqlite_count for each.
/// 4. Upsert as one `projects` row.
///
/// Returns the number of rows touched. Idempotent — running twice
/// against the same filesystem yields the same DB state.
pub fn rebuild_index(db_path: &Path, projects_root: &Path) -> Result<usize> {
    let conn = open_db(db_path)?;
    rebuild_index_with_conn(&conn, projects_root)
}

/// Same as `rebuild_index` but uses a caller-supplied connection. Used by
/// `kei-projects-watcher` to reuse one Connection across many reindex
/// calls without reopening the DB on each event.
pub fn rebuild_index_with_conn(conn: &Connection, projects_root: &Path) -> Result<usize> {
    let entries = walk_projects_root(projects_root).context("walk projects root")?;
    let now_ts = Utc::now().timestamp();
    let mut count = 0usize;
    for entry in &entries {
        let row = build_row(entry, now_ts);
        upsert_row(conn, &row)?;
        count += 1;
    }
    Ok(count)
}

/// Re-index a single project (one row). Used by the fsevents watcher
/// after a debounced file change in `<projects_root>/<project>/`.
/// `project_path` must be the immediate child of the projects root, i.e.
/// the project's own top-level directory.
pub fn reindex_one(conn: &Connection, project_path: &Path) -> Result<()> {
    let name = project_path
        .file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
        .unwrap_or_default();
    let entry = ProjectEntry {
        path: project_path.to_path_buf(),
        name,
        has_git: project_path.join(".git").exists(),
    };
    let now_ts = Utc::now().timestamp();
    let row = build_row(&entry, now_ts);
    upsert_row(conn, &row)
}
