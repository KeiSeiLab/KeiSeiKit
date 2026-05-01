// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! SQL schema for the kei-memory-sqlite `MemoryBackend`.
//!
//! Constructor Pattern: schema only, no business logic.
//!
//! Single table `memory_items` keyed by DNA (PRIMARY KEY). Tags are kept
//! in a single TEXT column as a comma-bordered CSV (`,t1,t2,`) so an
//! exact-token `LIKE '%,<tag>,%'` filter does not match prefixes.
//!
//! Indexes:
//! - `idx_memory_items_kind_key`   — supports kind + key-prefix queries.
//! - `idx_memory_items_created_at` — supports `since_ms` filter and
//!   `compact(since_ms)` deletion ordering.

use rusqlite::{Connection, Result};

/// DDL applied by [`apply_schema`]. Idempotent (`IF NOT EXISTS` everywhere).
pub const DDL: &str = "
    CREATE TABLE IF NOT EXISTS memory_items (
        dna           TEXT PRIMARY KEY,
        parent_dna    TEXT,
        kind          TEXT NOT NULL,
        key           TEXT NOT NULL,
        value         TEXT NOT NULL,
        tags_csv      TEXT NOT NULL,
        created_at_ms INTEGER NOT NULL
    );
    CREATE INDEX IF NOT EXISTS idx_memory_items_kind_key
        ON memory_items(kind, key);
    CREATE INDEX IF NOT EXISTS idx_memory_items_created_at
        ON memory_items(created_at_ms);
";

/// Apply the full schema. Idempotent — safe to call on every connection
/// open. Stores no version pragma on purpose: the backend's only schema
/// today is v1; bumps go through additive migrations on a future bump.
pub fn apply_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(DDL)?;
    Ok(())
}

/// Encode tag list as `,t1,t2,…,` so exact-token `LIKE '%,<tag>,%'`
/// matches without prefix collisions (e.g. tag "rem" must not match "remix").
pub fn encode_tags(tags: &[String]) -> String {
    if tags.is_empty() {
        return String::from(",");
    }
    let mut s = String::with_capacity(2 + tags.iter().map(|t| t.len() + 1).sum::<usize>());
    s.push(',');
    for t in tags {
        s.push_str(t);
        s.push(',');
    }
    s
}

/// Inverse of [`encode_tags`]. Robust to empty input (returns empty Vec).
pub fn decode_tags(csv: &str) -> Vec<String> {
    csv.split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_tags_encode_to_single_separator() {
        assert_eq!(encode_tags(&[]), ",");
    }

    #[test]
    fn tag_csv_roundtrip() {
        let tags = vec!["rem".to_string(), "sleep".to_string(), "wave6".to_string()];
        let enc = encode_tags(&tags);
        assert_eq!(enc, ",rem,sleep,wave6,");
        let dec = decode_tags(&enc);
        assert_eq!(dec, tags);
    }

    #[test]
    fn exact_token_match_does_not_collide_with_prefix() {
        let enc = encode_tags(&["remix".to_string()]);
        // Exact lookup for "rem" via the LIKE pattern '%,rem,%' must NOT match.
        assert!(!enc.contains(",rem,"));
        assert!(enc.contains(",remix,"));
    }

    #[test]
    fn schema_applies_idempotently() {
        let conn = Connection::open_in_memory().unwrap();
        apply_schema(&conn).unwrap();
        apply_schema(&conn).unwrap(); // second call must not error
    }
}
