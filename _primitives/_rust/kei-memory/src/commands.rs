//! Command handlers — one function per CLI subcommand.
//!
//! Constructor Pattern: each handler <30 LOC, single responsibility.
//! Pulled out of main.rs to keep the dispatcher under the 200 LOC limit.

use crate::{analyze, ingest, patterns, tfidf};
use rusqlite::Connection;
use std::path::PathBuf;
use std::process::ExitCode;

fn err(msg: &str) -> ExitCode {
    eprintln!("kei-memory: {msg}");
    ExitCode::from(1)
}

pub fn cmd_ingest(
    conn: &Connection,
    session_id: &str,
    transcript: &PathBuf,
    prompt: Option<String>,
) -> ExitCode {
    match ingest::ingest_jsonl(conn, session_id, transcript) {
        Ok(n) => {
            if let Some(p) = prompt {
                let _ = tfidf::index_document(conn, session_id, &p);
            }
            let _ = patterns::detect_in_session(conn, session_id);
            println!("ingested {n} events into session {session_id}");
            ExitCode::SUCCESS
        }
        Err(e) => err(&format!("ingest failed: {e}")),
    }
}

pub fn cmd_analyze(
    conn: &Connection,
    session: Option<String>,
    last: usize,
    summary: bool,
) -> ExitCode {
    let out = match session {
        Some(id) => analyze::render_report(conn, &id, summary),
        None => analyze::render_recent(conn, last, summary),
    };
    match out {
        Ok(s) => {
            print!("{s}");
            ExitCode::SUCCESS
        }
        Err(e) => err(&format!("analyze failed: {e}")),
    }
}

pub fn cmd_patterns(
    conn: &Connection,
    cross_session: bool,
    session: Option<String>,
) -> ExitCode {
    let rows = if cross_session {
        patterns::detect_cross_session(conn)
    } else if let Some(id) = session {
        patterns::detect_in_session(conn, &id)
    } else {
        patterns::list_all(conn, 50)
    };
    match rows {
        Ok(list) => {
            if list.is_empty() {
                println!("(no patterns)");
            }
            for p in list {
                println!(
                    "{:>4}  {}  session={}",
                    p.count,
                    p.event_class,
                    p.session_id.as_deref().unwrap_or("-")
                );
            }
            ExitCode::SUCCESS
        }
        Err(e) => err(&format!("patterns failed: {e}")),
    }
}

pub fn cmd_similar(conn: &Connection, prompt: &str, limit: usize) -> ExitCode {
    match tfidf::top_similar(conn, prompt, limit) {
        Ok(rows) => {
            if rows.is_empty() {
                println!("(no matches)");
            }
            for (sid, score) in rows {
                println!("{:.4}  {}", score, sid);
            }
            ExitCode::SUCCESS
        }
        Err(e) => err(&format!("similar failed: {e}")),
    }
}

pub fn cmd_dump(conn: &Connection, session_id: &str) -> ExitCode {
    match dump_events(conn, session_id) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => err(&format!("dump failed: {e}")),
    }
}

fn dump_events(conn: &Connection, session_id: &str) -> rusqlite::Result<()> {
    let mut stmt = conn.prepare(
        "SELECT ts, kind, tool, file_path, is_error, message
         FROM events WHERE session_id = ?1 ORDER BY ts ASC",
    )?;
    println!("# session {session_id}\n");
    let rows = stmt.query_map(rusqlite::params![session_id], |r| {
        Ok((
            r.get::<_, i64>(0)?,
            r.get::<_, String>(1)?,
            r.get::<_, Option<String>>(2)?,
            r.get::<_, Option<String>>(3)?,
            r.get::<_, i64>(4)?,
            r.get::<_, Option<String>>(5)?,
        ))
    })?;
    for row in rows {
        let (ts, kind, tool, file, is_err, msg) = row?;
        println!(
            "- `t={ts}` **{kind}** {} {} err={} {}",
            tool.unwrap_or_default(),
            file.unwrap_or_default(),
            is_err,
            msg.unwrap_or_default()
        );
    }
    Ok(())
}

pub fn cmd_stats(conn: &Connection) -> ExitCode {
    match print_stats(conn) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => err(&format!("stats failed: {e}")),
    }
}

fn print_stats(conn: &Connection) -> rusqlite::Result<()> {
    let n_sess: i64 = conn.query_row("SELECT COUNT(*) FROM sessions", [], |r| r.get(0))?;
    let n_evt: i64 = conn.query_row("SELECT COUNT(*) FROM events", [], |r| r.get(0))?;
    let n_pat: i64 = conn.query_row("SELECT COUNT(*) FROM patterns", [], |r| r.get(0))?;
    println!("sessions: {n_sess}\nevents:   {n_evt}\npatterns: {n_pat}");
    let mut stmt = conn.prepare(
        "SELECT tool, COUNT(*) FROM events WHERE tool IS NOT NULL
         GROUP BY tool ORDER BY COUNT(*) DESC LIMIT 10",
    )?;
    println!("\nTop tools:");
    let rows = stmt.query_map([], |r| {
        Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?))
    })?;
    for row in rows {
        let (t, c) = row?;
        println!("  {c:>4}  {t}");
    }
    Ok(())
}

