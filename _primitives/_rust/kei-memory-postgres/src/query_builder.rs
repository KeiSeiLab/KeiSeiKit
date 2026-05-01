// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Dynamic WHERE composition for [`MemoryQuery`]. Returns a parameterised
//! SQL string + a parallel parameter vector typed for tokio-postgres.
//!
//! Kept in its own module so `backend.rs` stays under the 200-LOC cube
//! limit and the SQL composition can be unit-tested in isolation.

use kei_runtime_core::traits::memory::MemoryQuery;
use tokio_postgres::types::ToSql;

/// Output of [`build_select`]: the full statement plus the boxed
/// parameter list, ready for `Client::query`.
pub struct BuiltQuery {
    pub sql: String,
    pub params: Vec<Box<dyn ToSql + Sync + Send>>,
}

const BASE_SELECT: &str = "\
SELECT dna, parent_dna, kind, key, value, tags, created_at_ms \
FROM memory_items";

/// Compose a SELECT for the given query. `LIMIT` defaults to 1000 when
/// the caller leaves it unset — keeps an unbounded query from melting
/// the wire.
pub fn build_select(q: &MemoryQuery) -> BuiltQuery {
    let mut where_clauses: Vec<String> = Vec::new();
    let mut params: Vec<Box<dyn ToSql + Sync + Send>> = Vec::new();

    if let Some(kind) = &q.kind {
        params.push(Box::new(kind.clone()));
        where_clauses.push(format!("kind = ${}", params.len()));
    }
    if let Some(prefix) = &q.key_prefix {
        // `LIKE prefix || '%'` keeps the prefix as a parameter (no
        // string-concat into the SQL), so the index on (kind, key) is
        // still usable when the prefix is a literal-leading pattern.
        params.push(Box::new(prefix.clone()));
        where_clauses.push(format!("key LIKE ${} || '%'", params.len()));
    }
    if let Some(since) = q.since_ms {
        params.push(Box::new(since));
        where_clauses.push(format!("created_at_ms >= ${}", params.len()));
    }
    if !q.tag_any.is_empty() {
        params.push(Box::new(q.tag_any.clone()));
        // `&&` is the "array overlap" operator — true if any element
        // matches. Index-friendly when a GIN index exists on `tags`.
        where_clauses.push(format!("tags && ${}", params.len()));
    }

    let mut sql = String::from(BASE_SELECT);
    if !where_clauses.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&where_clauses.join(" AND "));
    }
    sql.push_str(" ORDER BY created_at_ms DESC");

    let limit_i64 = q.limit.unwrap_or(1000) as i64;
    params.push(Box::new(limit_i64));
    sql.push_str(&format!(" LIMIT ${}", params.len()));

    BuiltQuery { sql, params }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_query_has_no_where() {
        let q = MemoryQuery::default();
        let b = build_select(&q);
        assert!(!b.sql.contains("WHERE"));
        assert!(b.sql.contains("ORDER BY created_at_ms DESC"));
        assert!(b.sql.contains("LIMIT $1"));
        assert_eq!(b.params.len(), 1, "limit is the only param");
    }

    #[test]
    fn full_query_composes_all_clauses() {
        let q = MemoryQuery {
            kind: Some("trace".into()),
            key_prefix: Some("agent/".into()),
            tag_any: vec!["sleep".into(), "rem".into()],
            limit: Some(50),
            since_ms: Some(1_700_000_000_000),
        };
        let b = build_select(&q);
        assert!(b.sql.contains("kind = $1"));
        assert!(b.sql.contains("key LIKE $2 || '%'"));
        assert!(b.sql.contains("created_at_ms >= $3"));
        assert!(b.sql.contains("tags && $4"));
        assert!(b.sql.contains("LIMIT $5"));
        assert_eq!(b.params.len(), 5);
    }

    #[test]
    fn limit_default_is_1000() {
        let q = MemoryQuery {
            kind: Some("x".into()),
            ..Default::default()
        };
        let b = build_select(&q);
        // last param is the limit; can't introspect Box<dyn ToSql>
        // directly, so check the SQL placeholder count.
        assert!(b.sql.ends_with("LIMIT $2"));
        assert_eq!(b.params.len(), 2);
    }
}
