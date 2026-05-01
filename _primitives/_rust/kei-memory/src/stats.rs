//! DB-wide statistics renderer.
//!
//! Constructor Pattern: extracted from commands.rs (was `print_stats`).
//! Pure formatter: takes a Connection, returns a String. Sessions, events,
//! patterns counts plus the top-10 most-invoked tools.

use rusqlite::{Connection, Result};

/// Render DB-wide statistics as a multi-line string.
///
/// Lines: `sessions: N`, `events: N`, `patterns: N`, blank, `Top tools:`,
/// then up to 10 `count  tool` rows ordered by count DESC.
pub fn render_stats(conn: &Connection) -> Result<String> {
    let n_sess: i64 = conn.query_row("SELECT COUNT(*) FROM sessions", [], |r| r.get(0))?;
    let n_evt: i64 = conn.query_row("SELECT COUNT(*) FROM events", [], |r| r.get(0))?;
    let n_pat: i64 = conn.query_row("SELECT COUNT(*) FROM patterns", [], |r| r.get(0))?;
    let mut out = String::new();
    out.push_str(&format!(
        "sessions: {n_sess}\nevents:   {n_evt}\npatterns: {n_pat}\n"
    ));
    let mut stmt = conn.prepare(
        "SELECT tool, COUNT(*) FROM events WHERE tool IS NOT NULL
         GROUP BY tool ORDER BY COUNT(*) DESC LIMIT 10",
    )?;
    let rows = stmt
        .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?)))?
        .collect::<Result<Vec<_>>>()?;
    out.push_str("\nTop tools:\n");
    for (t, c) in rows {
        out.push_str(&format!("  {c:>4}  {t}\n"));
    }
    Ok(out)
}
