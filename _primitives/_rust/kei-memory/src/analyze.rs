//! Session retrospective — duration, tool counts, files, errors, time-wasters.
//!
//! Constructor Pattern: one cube, one read-only responsibility.
//! Output is plain-text (stdout). Callers can `--summary` for a one-liner
//! suitable for appending to audit-backlog.md, or full report for review.

use rusqlite::{params, Connection, OptionalExtension, Result};

/// Minimal session-header info returned as tuple for downstream formatters.
pub struct SessionHeader {
    pub id: String,
    pub started_ts: i64,
    pub ended_ts: Option<i64>,
    pub tool_call_count: i64,
    pub error_count: i64,
}

/// Load the `sessions` row for an id.
pub fn session_header(conn: &Connection, id: &str) -> Result<Option<SessionHeader>> {
    conn.query_row(
        "SELECT id, started_ts, ended_ts, tool_call_count, error_count
         FROM sessions WHERE id = ?1",
        params![id],
        |r| {
            Ok(SessionHeader {
                id: r.get(0)?,
                started_ts: r.get(1)?,
                ended_ts: r.get(2)?,
                tool_call_count: r.get(3)?,
                error_count: r.get(4)?,
            })
        },
    )
    .optional()
}

/// Return the last `n` session ids (most recent first).
pub fn recent_session_ids(conn: &Connection, n: usize) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT id FROM sessions ORDER BY COALESCE(ended_ts, started_ts) DESC LIMIT ?1",
    )?;
    let rows = stmt
        .query_map(params![n as i64], |r| r.get::<_, String>(0))?
        .filter_map(|r| r.ok())
        .collect();
    Ok(rows)
}

/// Return (tool, count) pairs ordered by invocation count DESC.
pub fn top_tools(conn: &Connection, session_id: &str, limit: usize) -> Result<Vec<(String, i64)>> {
    let mut stmt = conn.prepare(
        "SELECT tool, COUNT(*) FROM events
         WHERE session_id = ?1 AND tool IS NOT NULL
         GROUP BY tool ORDER BY COUNT(*) DESC LIMIT ?2",
    )?;
    let rows = stmt
        .query_map(params![session_id, limit as i64], |r| {
            Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?))
        })?
        .collect::<Result<Vec<_>>>()?;
    Ok(rows)
}

/// Return (file_path, count) for the most-touched files in a session.
pub fn top_files(conn: &Connection, session_id: &str, limit: usize) -> Result<Vec<(String, i64)>> {
    let mut stmt = conn.prepare(
        "SELECT file_path, COUNT(*) FROM events
         WHERE session_id = ?1 AND file_path IS NOT NULL
         GROUP BY file_path ORDER BY COUNT(*) DESC LIMIT ?2",
    )?;
    let rows = stmt
        .query_map(params![session_id, limit as i64], |r| {
            Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?))
        })?
        .collect::<Result<Vec<_>>>()?;
    Ok(rows)
}

/// Render a full retrospective for one session to stdout.
pub fn render_report(conn: &Connection, session_id: &str, summary_only: bool) -> Result<String> {
    let hdr = match session_header(conn, session_id)? {
        Some(h) => h,
        None => return Ok(format!("(no session with id {session_id})\n")),
    };
    let duration = hdr.ended_ts.unwrap_or(hdr.started_ts) - hdr.started_ts;
    if summary_only {
        return Ok(format!(
            "session={} dur={}s tools={} errors={}\n",
            hdr.id, duration, hdr.tool_call_count, hdr.error_count
        ));
    }
    let mut out = String::new();
    out.push_str(&format!("=== SESSION {} ===\n", hdr.id));
    out.push_str(&format!("Duration:    {}s\n", duration));
    out.push_str(&format!("Tool calls:  {}\n", hdr.tool_call_count));
    out.push_str(&format!("Errors:      {}\n", hdr.error_count));
    out.push_str("\nTop tools:\n");
    for (t, c) in top_tools(conn, session_id, 5)? {
        out.push_str(&format!("  {c:>4}  {t}\n"));
    }
    out.push_str("\nTop files:\n");
    for (f, c) in top_files(conn, session_id, 10)? {
        out.push_str(&format!("  {c:>4}  {f}\n"));
    }
    Ok(out)
}

/// Aggregate analyze across recent N sessions — concat render_report each.
pub fn render_recent(conn: &Connection, n: usize, summary_only: bool) -> Result<String> {
    let ids = recent_session_ids(conn, n)?;
    if ids.is_empty() {
        return Ok("(no sessions ingested yet)\n".into());
    }
    let mut out = String::new();
    for id in ids {
        out.push_str(&render_report(conn, &id, summary_only)?);
        if !summary_only {
            out.push('\n');
        }
    }
    Ok(out)
}
