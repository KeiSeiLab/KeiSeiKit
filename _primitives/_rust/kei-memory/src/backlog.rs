//! Backlog — silent-first audit item CRUD.
//!
//! Constructor Pattern: one cube, one CLI subcommand's worth of logic.
//! Wire-point #3 of the injection guard (RULE 0.14 audit-CRUD path):
//! malicious content in audit items would be rendered into self-audit
//! reports verbatim, so we scan before persistence the same as
//! `ingest::insert_event` does for transcript messages.

use crate::injection_guard;
use chrono::Utc;
use rusqlite::{params, Connection};
use std::process::ExitCode;

fn err(msg: &str) -> ExitCode {
    eprintln!("kei-memory: {msg}");
    ExitCode::from(1)
}

pub fn cmd_backlog(
    conn: &Connection,
    add: Option<String>,
    list: bool,
    clear: bool,
) -> ExitCode {
    let now = Utc::now().timestamp();
    if let Some(item) = add {
        if let Err(finding) = injection_guard::scan(&item) {
            return err(&format!("backlog add blocked by injection guard: {finding}"));
        }
        if let Err(e) = conn.execute(
            "INSERT INTO backlog (ts, item) VALUES (?1, ?2)",
            params![now, item],
        ) {
            return err(&format!("backlog add failed: {e}"));
        }
        println!("added");
        return ExitCode::SUCCESS;
    }
    if clear {
        let _ = conn.execute("UPDATE backlog SET processed = 1", []);
        println!("cleared");
        return ExitCode::SUCCESS;
    }
    if list {
        return list_open(conn);
    }
    err("backlog: pass --add=<item> | --list | --clear")
}

fn list_open(conn: &Connection) -> ExitCode {
    let mut stmt = match conn
        .prepare("SELECT ts, item FROM backlog WHERE processed = 0 ORDER BY ts ASC")
    {
        Ok(s) => s,
        Err(e) => return err(&format!("backlog list prep: {e}")),
    };
    let rows = match stmt.query_map([], |r| {
        Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?))
    }) {
        Ok(r) => r,
        Err(e) => return err(&format!("backlog list query: {e}")),
    };
    for row in rows {
        if let Ok((ts, it)) = row {
            println!("{ts}\t{it}");
        }
    }
    ExitCode::SUCCESS
}
