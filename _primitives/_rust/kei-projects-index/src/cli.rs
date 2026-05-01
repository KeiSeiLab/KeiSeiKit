//! CLI command handlers — keeps `main.rs` ≤ 30 LOC by holding the four
//! sub-command implementations as one cube.

use kei_projects_index::{index::rebuild_index, query, schema::init};
use rusqlite::Connection;
use std::path::PathBuf;
use std::process::ExitCode;

fn home() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("."))
}

fn default_db() -> PathBuf {
    home().join(".claude/agents/projects-index.sqlite")
}

fn default_root() -> PathBuf {
    home().join("Projects")
}

fn open_db(db: &PathBuf) -> Result<Connection, String> {
    if let Some(p) = db.parent() {
        let _ = std::fs::create_dir_all(p);
    }
    let conn = Connection::open(db).map_err(|e| format!("open {}: {e}", db.display()))?;
    init(&conn).map_err(|e| format!("schema init: {e}"))?;
    Ok(conn)
}

fn err(msg: &str) -> ExitCode {
    eprintln!("kei-projects-index: {msg}");
    ExitCode::from(1)
}

fn print_json<T: serde::Serialize>(value: &T) -> ExitCode {
    match serde_json::to_string_pretty(value) {
        Ok(s) => {
            println!("{s}");
            ExitCode::SUCCESS
        }
        Err(e) => err(&format!("serialize: {e}")),
    }
}

/// `init` — open / create DB and apply schema.
pub fn cmd_init(db: Option<PathBuf>) -> ExitCode {
    let path = db.unwrap_or_else(default_db);
    match open_db(&path) {
        Ok(_) => {
            println!("initialised {}", path.display());
            ExitCode::SUCCESS
        }
        Err(e) => err(&e),
    }
}

/// `rebuild` — walk root and refresh all rows.
pub fn cmd_rebuild(db: Option<PathBuf>, root: Option<PathBuf>) -> ExitCode {
    let dbp = db.unwrap_or_else(default_db);
    let rp = root.unwrap_or_else(default_root);
    match rebuild_index(&dbp, &rp) {
        Ok(n) => {
            println!("indexed {n} project(s)");
            ExitCode::SUCCESS
        }
        Err(e) => err(&format!("rebuild: {e}")),
    }
}

/// `list` — print all rows as JSON.
pub fn cmd_list(db: Option<PathBuf>) -> ExitCode {
    let path = db.unwrap_or_else(default_db);
    let conn = match open_db(&path) {
        Ok(c) => c,
        Err(e) => return err(&e),
    };
    match query::list_all(&conn) {
        Ok(rows) => print_json(&rows),
        Err(e) => err(&format!("list: {e}")),
    }
}

/// `get` — print one row by path.
pub fn cmd_get(db: Option<PathBuf>, project_path: String) -> ExitCode {
    let path = db.unwrap_or_else(default_db);
    let conn = match open_db(&path) {
        Ok(c) => c,
        Err(e) => return err(&e),
    };
    match query::get_one(&conn, &project_path) {
        Ok(Some(row)) => print_json(&row),
        Ok(None) => err(&format!("no project at {project_path}")),
        Err(e) => err(&format!("get: {e}")),
    }
}
