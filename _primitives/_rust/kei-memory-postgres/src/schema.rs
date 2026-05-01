// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Schema bootstrap. One idempotent `CREATE TABLE IF NOT EXISTS` plus
//! two indexes. Anything richer (partitioning, GIN on tags, FTS) is a
//! caller's choice — keep this primitive minimal.

use crate::error::Result;
use tokio_postgres::Client;

/// SSoT DDL applied by [`apply_schema`]. Public so external callers
/// (migration tools, integration tests) can inspect or extend it.
pub const SCHEMA_SQL: &str = "\
CREATE TABLE IF NOT EXISTS memory_items ( \
    dna TEXT PRIMARY KEY, \
    parent_dna TEXT, \
    kind TEXT NOT NULL, \
    key TEXT NOT NULL, \
    value JSONB NOT NULL, \
    tags TEXT[] NOT NULL, \
    created_at_ms BIGINT NOT NULL \
); \
CREATE INDEX IF NOT EXISTS idx_kind_key ON memory_items(kind, key); \
CREATE INDEX IF NOT EXISTS idx_created ON memory_items(created_at_ms);";

/// Run [`SCHEMA_SQL`] against the supplied client. Idempotent — safe to
/// call on every cold start. Uses `batch_execute` so all statements run
/// in a single round-trip.
pub async fn apply_schema(client: &Client) -> Result<()> {
    client.batch_execute(SCHEMA_SQL).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_sql_non_empty() {
        assert!(!SCHEMA_SQL.is_empty());
        assert!(SCHEMA_SQL.len() > 64, "schema must define a real table");
    }

    #[test]
    fn schema_sql_creates_table() {
        assert!(SCHEMA_SQL.contains("CREATE TABLE"));
        assert!(SCHEMA_SQL.contains("memory_items"));
        assert!(SCHEMA_SQL.contains("PRIMARY KEY"));
    }

    #[test]
    fn schema_sql_uses_jsonb_and_indexes() {
        assert!(SCHEMA_SQL.contains("JSONB"), "value column must be JSONB");
        assert!(SCHEMA_SQL.contains("idx_kind_key"));
        assert!(SCHEMA_SQL.contains("idx_created"));
        assert!(SCHEMA_SQL.contains("TEXT[]"), "tags must be a TEXT array");
    }
}
