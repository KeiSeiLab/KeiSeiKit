//! TF-IDF over session documents.
//!
//! Constructor Pattern: one cube, one responsibility. Classical text
//! retrieval: tokens, TF, IDF, cosine similarity. Document = session_id.
//!
//! Design: `index_document` no longer rebuilds IDF on every call (was
//! O(N·V) per insert). It marks `tokens.idf_dirty = 1`; readers
//! (analyze, patterns, similar) invoke `recompute_idf_if_stale` once.

use crate::similarity::cosine_tfidf;
use regex::Regex;
use rusqlite::{params, Connection, Result};
use std::collections::HashMap;

/// Tokenise free text into lowercase alphanumeric word stems (≥3 chars).
pub fn tokenise(text: &str) -> Vec<String> {
    let re = Regex::new(r"[A-Za-z][A-Za-z0-9_]{2,}").unwrap();
    re.find_iter(text)
        .map(|m| m.as_str().to_lowercase())
        .collect()
}

/// Compute term-frequencies for a single document.
pub fn tf(tokens: &[String]) -> HashMap<String, i64> {
    let mut h = HashMap::<String, i64>::new();
    for t in tokens {
        *h.entry(t.clone()).or_insert(0) += 1;
    }
    h
}

/// Record a document's tokens under `session_id`. Overwrites prior entry
/// for the same session (idempotent ingest). Sets `idf_dirty = 1` to mark
/// the corpus as needing IDF recomputation; the caller flushes via
/// `recompute_idf_if_stale` at the next read-side entry point.
pub fn index_document(conn: &Connection, session_id: &str, text: &str) -> Result<()> {
    conn.execute("DELETE FROM tokens WHERE session_id = ?1", params![session_id])?;
    let toks = tokenise(text);
    let counts = tf(&toks);
    for (tok, c) in &counts {
        conn.execute(
            "INSERT INTO tokens (session_id, token, tf, idf_dirty) VALUES (?1, ?2, ?3, 1)",
            params![session_id, tok, c],
        )?;
    }
    Ok(())
}

/// Recompute the full IDF table unconditionally. Cheap for N < 10k sessions.
/// Clears the `idf_dirty` flag on every token row after a successful pass.
pub fn recompute_idf(conn: &Connection) -> Result<()> {
    let n: i64 = conn
        .query_row(
            "SELECT COUNT(DISTINCT session_id) FROM tokens",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    conn.execute("DELETE FROM idf", [])?;
    if n == 0 {
        conn.execute("UPDATE tokens SET idf_dirty = 0", [])?;
        return Ok(());
    }
    let mut stmt = conn.prepare(
        "SELECT token, COUNT(DISTINCT session_id) FROM tokens GROUP BY token",
    )?;
    let rows: Vec<(String, i64)> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<Result<Vec<_>>>()?;
    for (tok, df) in rows {
        let idf = ((n as f64 + 1.0) / (df as f64 + 1.0)).ln() + 1.0;
        conn.execute(
            "INSERT INTO idf (token, df, idf) VALUES (?1, ?2, ?3)",
            params![tok, df, idf],
        )?;
    }
    conn.execute("UPDATE tokens SET idf_dirty = 0", [])?;
    Ok(())
}

/// Recompute IDF only if any token row is marked dirty. Returns `true` when
/// a recompute ran, `false` if the corpus was already clean.
pub fn recompute_idf_if_stale(conn: &Connection) -> Result<bool> {
    let dirty: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM tokens WHERE idf_dirty = 1",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    if dirty == 0 {
        return Ok(false);
    }
    recompute_idf(conn)?;
    Ok(true)
}

/// Fetch a session's (token → tf·idf) sparse vector.
pub fn session_vector(conn: &Connection, session_id: &str) -> Result<HashMap<String, f64>> {
    let mut stmt = conn.prepare(
        "SELECT t.token, t.tf, COALESCE(i.idf, 1.0)
         FROM tokens t
         LEFT JOIN idf i ON i.token = t.token
         WHERE t.session_id = ?1",
    )?;
    let rows = stmt.query_map(params![session_id], |r| {
        Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)? as f64, r.get::<_, f64>(2)?))
    })?;
    let mut v = HashMap::<String, f64>::new();
    for row in rows {
        let (tok, tf_v, idf_v) = row?;
        v.insert(tok, tf_v * idf_v);
    }
    Ok(v)
}

/// Compute a TF·IDF vector for ad-hoc query text, using existing corpus IDF.
pub fn query_vector(conn: &Connection, text: &str) -> Result<HashMap<String, f64>> {
    let toks = tokenise(text);
    let counts = tf(&toks);
    let mut v = HashMap::<String, f64>::new();
    for (tok, c) in counts {
        // SAFETY: OOV tokens (not in `idf`) get neutral IDF=1.0 by design.
        let idf: f64 = conn
            .query_row(
                "SELECT idf FROM idf WHERE token = ?1",
                params![tok],
                |r| r.get(0),
            )
            .unwrap_or(1.0);
        v.insert(tok, c as f64 * idf);
    }
    Ok(v)
}

/// Pull (session_id → tf·idf vector) for every session that shares at least
/// one token with `q_tokens`. Single SQL JOIN; row errors propagate.
fn vectors_for_overlapping_sessions(
    conn: &Connection,
    q_tokens: &[String],
) -> Result<HashMap<String, HashMap<String, f64>>> {
    let placeholders: String = q_tokens.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let sql = format!(
        "SELECT t.session_id, t.token, t.tf, COALESCE(i.idf, 1.0)
         FROM tokens t
         LEFT JOIN idf i ON i.token = t.token
         WHERE t.token IN ({placeholders})"
    );
    let mut stmt = conn.prepare(&sql)?;
    let params_iter: Vec<&dyn rusqlite::ToSql> =
        q_tokens.iter().map(|t| t as &dyn rusqlite::ToSql).collect();
    let rows = stmt.query_map(params_iter.as_slice(), |r| {
        Ok((
            r.get::<_, String>(0)?,
            r.get::<_, String>(1)?,
            r.get::<_, i64>(2)? as f64,
            r.get::<_, f64>(3)?,
        ))
    })?;
    let mut per_session: HashMap<String, HashMap<String, f64>> = HashMap::new();
    for row in rows {
        let (sid, tok, tf_v, idf_v) = row?;
        per_session.entry(sid).or_default().insert(tok, tf_v * idf_v);
    }
    Ok(per_session)
}

/// Return the top-k sessions by cosine similarity against `query`.
///
/// Single-JOIN rewrite: one prepared SELECT pulls every (session_id, token,
/// tf·idf) row whose token appears in the query vocabulary, then we fold
/// per-session vectors in Rust and run cosine. Replaces the prior N+1 path
/// (one `session_vector` call per candidate session). Row errors propagate
/// instead of being silently dropped.
pub fn top_similar(
    conn: &Connection,
    query: &str,
    limit: usize,
) -> Result<Vec<(String, f64)>> {
    recompute_idf_if_stale(conn)?;
    let q = query_vector(conn, query)?;
    if q.is_empty() {
        return Ok(vec![]);
    }
    let q_tokens: Vec<String> = q.keys().cloned().collect();
    let per_session = vectors_for_overlapping_sessions(conn, &q_tokens)?;
    let mut scored: Vec<(String, f64)> = per_session
        .into_iter()
        .map(|(sid, v)| (sid, cosine_tfidf(&q, &v)))
        .filter(|(_, s)| *s > 0.0)
        .collect();
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    scored.truncate(limit);
    Ok(scored)
}
