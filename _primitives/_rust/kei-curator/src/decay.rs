//! Exponential decay on cross_edges.

use crate::config::Config;
use anyhow::Result;
use chrono::Utc;
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct DecayReport {
    pub updated: usize,
    pub pruned: usize,
}

pub fn decay_edges(conn: &Connection, cfg: &Config) -> Result<DecayReport> {
    let now = Utc::now().timestamp();
    let (updates, deletes) = compute_decay(conn, cfg, now)?;
    apply_decay(conn, &updates, &deletes)
}

fn compute_decay(conn: &Connection, cfg: &Config, now: i64)
    -> Result<(Vec<(i64, f64)>, Vec<i64>)>
{
    let mut stmt = conn.prepare("SELECT id, from_uri, weight, created_at FROM cross_edges")?;
    let rows = stmt.query_map([], |r| Ok((
        r.get::<_, i64>(0)?, r.get::<_, String>(1)?,
        r.get::<_, f64>(2)?, r.get::<_, i64>(3)?,
    )))?;
    let mut updates: Vec<(i64, f64)> = Vec::new();
    let mut deletes: Vec<i64> = Vec::new();
    for row in rows {
        let (id, from_uri, weight, created_at) = row?;
        let lambda = cfg.lambda_for(extract_domain(&from_uri));
        let age_days = (now - created_at) as f64 / 86_400.0;
        if age_days <= 0.0 { continue; }
        let new_w = weight * (-lambda * age_days).exp();
        if new_w < cfg.prune_threshold {
            deletes.push(id);
        } else if (new_w - weight).abs() > 0.001 {
            updates.push((id, new_w));
        }
    }
    Ok((updates, deletes))
}

fn apply_decay(conn: &Connection, updates: &[(i64, f64)], deletes: &[i64])
    -> Result<DecayReport>
{
    let mut r = DecayReport::default();
    for (id, w) in updates {
        conn.execute("UPDATE cross_edges SET weight=?1 WHERE id=?2", params![w, id])?;
        r.updated += 1;
    }
    for id in deletes {
        conn.execute("DELETE FROM cross_edges WHERE id=?1", params![id])?;
        r.pruned += 1;
    }
    Ok(r)
}

fn extract_domain(uri: &str) -> &str {
    match uri.find("://") {
        Some(i) if i > 0 => &uri[..i],
        _ => "",
    }
}
