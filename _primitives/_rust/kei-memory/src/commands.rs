//! Command handlers — one function per CLI subcommand.
//!
//! Constructor Pattern: each handler <30 LOC, single responsibility.
//! Pulled out of main.rs to keep the dispatcher under the 200 LOC limit.

use crate::{analyze, dump, ingest, patterns, stats, tfidf};
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
            // Single IDF recompute after any prompt(s) — was per-document.
            let _ = tfidf::recompute_idf_if_stale(conn);
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
    let _ = tfidf::recompute_idf_if_stale(conn);
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
    let _ = tfidf::recompute_idf_if_stale(conn);
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
    match dump::render_events(conn, session_id) {
        Ok(s) => {
            print!("{s}");
            ExitCode::SUCCESS
        }
        Err(e) => err(&format!("dump failed: {e}")),
    }
}

pub fn cmd_stats(conn: &Connection) -> ExitCode {
    match stats::render_stats(conn) {
        Ok(s) => {
            print!("{s}");
            ExitCode::SUCCESS
        }
        Err(e) => err(&format!("stats failed: {e}")),
    }
}

