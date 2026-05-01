//! Event dump renderer — print events for a session as markdown.
//!
//! Constructor Pattern: extracted from commands.rs (was `dump_events`).
//! Pure formatter: takes a Connection + session_id, returns a String.
//! The CLI wrapper in commands.rs prints it; library callers can capture.

use rusqlite::{params, Connection, Result};

/// Render a session's events as a markdown bullet list.
///
/// Output starts with a `# session <id>` header followed by one bullet
/// per event ordered by timestamp ASC. Errors propagate from the
/// underlying SQLite query / row decoding.
pub fn render_events(conn: &Connection, session_id: &str) -> Result<String> {
    let mut stmt = conn.prepare(
        "SELECT ts, kind, tool, file_path, is_error, message
         FROM events WHERE session_id = ?1 ORDER BY ts ASC",
    )?;
    let rows = stmt
        .query_map(params![session_id], |r| {
            Ok((
                r.get::<_, i64>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, Option<String>>(2)?,
                r.get::<_, Option<String>>(3)?,
                r.get::<_, i64>(4)?,
                r.get::<_, Option<String>>(5)?,
            ))
        })?
        .collect::<Result<Vec<_>>>()?;
    let mut out = String::new();
    out.push_str(&format!("# session {session_id}\n\n"));
    for (ts, kind, tool, file, is_err, msg) in rows {
        out.push_str(&format!(
            "- `t={ts}` **{kind}** {} {} err={} {}\n",
            tool.unwrap_or_default(),
            file.unwrap_or_default(),
            is_err,
            msg.unwrap_or_default()
        ));
    }
    Ok(out)
}
