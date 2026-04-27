//! Pattern detector — recurring event-classes.
//!
//! Constructor Pattern: one cube, one read/write responsibility.
//! A "pattern" is an event_class that occurred ≥2 times in ONE session
//! (in-session recurrence) or ≥2 times across DIFFERENT sessions
//! (cross-session recurrence). Results are persisted into `patterns` and
//! also returned to the caller for display.

use rusqlite::{params, Connection, Result};

#[derive(Debug)]
#[allow(dead_code)]
pub struct PatternHit {
    pub event_class: String,
    pub session_id: Option<String>,
    pub count: i64,
}

/// Detect in-session recurrences for `session_id`. Persists rows.
pub fn detect_in_session(conn: &Connection, session_id: &str) -> Result<Vec<PatternHit>> {
    let mut stmt = conn.prepare(
        "SELECT event_class, COUNT(*), MIN(ts), MAX(ts)
         FROM events
         WHERE session_id = ?1 AND event_class IS NOT NULL
         GROUP BY event_class HAVING COUNT(*) >= 2
         ORDER BY COUNT(*) DESC",
    )?;
    let rows = stmt
        .query_map(params![session_id], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, i64>(1)?,
                r.get::<_, i64>(2)?,
                r.get::<_, i64>(3)?,
            ))
        })?
        .collect::<Result<Vec<_>>>()?;
    let mut out = Vec::new();
    for (class, count, first, last) in rows {
        conn.execute(
            "INSERT INTO patterns (event_class, session_id, count, first_seen_ts, last_seen_ts)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![class, session_id, count, first, last],
        )?;
        out.push(PatternHit {
            event_class: class,
            session_id: Some(session_id.to_string()),
            count,
        });
    }
    Ok(out)
}

/// Detect cross-session recurrences. Does NOT persist (history aggregate).
pub fn detect_cross_session(conn: &Connection) -> Result<Vec<PatternHit>> {
    let mut stmt = conn.prepare(
        "SELECT event_class, COUNT(DISTINCT session_id)
         FROM events
         WHERE event_class IS NOT NULL
         GROUP BY event_class HAVING COUNT(DISTINCT session_id) >= 2
         ORDER BY COUNT(DISTINCT session_id) DESC",
    )?;
    let rows = stmt
        .query_map([], |r| {
            Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?))
        })?
        .collect::<Result<Vec<_>>>()?;
    Ok(rows
        .into_iter()
        .map(|(class, count)| PatternHit {
            event_class: class,
            session_id: None,
            count,
        })
        .collect())
}

/// List all patterns in the persistent table (newest first).
#[allow(dead_code)]
pub fn list_all(conn: &Connection, limit: usize) -> Result<Vec<PatternHit>> {
    let mut stmt = conn.prepare(
        "SELECT event_class, session_id, count FROM patterns
         ORDER BY last_seen_ts DESC LIMIT ?1",
    )?;
    let rows = stmt
        .query_map(params![limit as i64], |r| {
            Ok(PatternHit {
                event_class: r.get::<_, String>(0)?,
                session_id: Some(r.get::<_, String>(1)?),
                count: r.get::<_, i64>(2)?,
            })
        })?
        .collect::<Result<Vec<_>>>()?;
    Ok(rows)
}
