//! TF-IDF over session documents — fresh reimplementation.
//!
//! Constructor Pattern: one cube, one responsibility.
//! 
//! Pure classical text-retrieval: tokens, term-frequency, inverse-doc-freq,
//! cosine similarity between (session_id, prompt) document vectors.
//!
//! Document identity = session_id. Corpus = all ingested sessions.

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
/// for the same session (idempotent ingest).
pub fn index_document(conn: &Connection, session_id: &str, text: &str) -> Result<()> {
    conn.execute("DELETE FROM tokens WHERE session_id = ?1", params![session_id])?;
    let toks = tokenise(text);
    let counts = tf(&toks);
    for (tok, c) in &counts {
        conn.execute(
            "INSERT INTO tokens (session_id, token, tf) VALUES (?1, ?2, ?3)",
            params![session_id, tok, c],
        )?;
    }
    recompute_idf(conn)?;
    Ok(())
}

/// Recompute the full IDF table. Called after each document ingest — cheap
/// for N < 10k sessions, and keeps the table in sync without an update trigger.
pub fn recompute_idf(conn: &Connection) -> Result<()> {
    let n: i64 = conn
        .query_row(
            "SELECT COUNT(DISTINCT session_id) FROM tokens",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    if n == 0 {
        conn.execute("DELETE FROM idf", [])?;
        return Ok(());
    }
    conn.execute("DELETE FROM idf", [])?;
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
    Ok(())
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

/// Return the top-k sessions by cosine similarity against `query`.
pub fn top_similar(
    conn: &Connection,
    query: &str,
    limit: usize,
) -> Result<Vec<(String, f64)>> {
    let q = query_vector(conn, query)?;
    if q.is_empty() {
        return Ok(vec![]);
    }
    let mut stmt = conn.prepare("SELECT DISTINCT session_id FROM tokens")?;
    let sessions: Vec<String> = stmt
        .query_map([], |r| r.get::<_, String>(0))?
        .filter_map(|r| r.ok())
        .collect();
    let mut scored: Vec<(String, f64)> = sessions
        .into_iter()
        .map(|sid| {
            let v = session_vector(conn, &sid).unwrap_or_default();
            let s = cosine_tfidf(&q, &v);
            (sid, s)
        })
        .filter(|(_, s)| *s > 0.0)
        .collect();
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    scored.truncate(limit);
    Ok(scored)
}
