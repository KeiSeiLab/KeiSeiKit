//! Aggregate chat stats.

use crate::store::Store;
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct Stats {
    pub total_sessions: i64,
    pub active_sessions: i64,
    pub archived_sessions: i64,
    pub total_messages: i64,
    pub total_tokens: i64,
    pub total_cost: f64,
}

pub fn stats(store: &Store) -> Result<Stats> {
    let mut s = Stats::default();
    s.total_sessions = store.conn()
        .query_row("SELECT COUNT(*) FROM chat_sessions", [], |r| r.get(0))?;
    s.active_sessions = store.conn()
        .query_row("SELECT COUNT(*) FROM chat_sessions WHERE status='active'", [], |r| r.get(0))?;
    s.archived_sessions = store.conn()
        .query_row("SELECT COUNT(*) FROM chat_sessions WHERE status='archived'", [], |r| r.get(0))?;
    s.total_messages = store.conn()
        .query_row("SELECT COUNT(*) FROM chat_messages", [], |r| r.get(0))?;
    s.total_tokens = store.conn()
        .query_row("SELECT COALESCE(SUM(total_tokens),0) FROM chat_sessions", [], |r| r.get(0))?;
    s.total_cost = store.conn()
        .query_row("SELECT COALESCE(SUM(total_cost),0) FROM chat_sessions", [], |r| r.get(0))?;
    Ok(s)
}
