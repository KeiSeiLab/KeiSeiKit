//! Aggregate counts over the registry.
//!
//! Constructor Pattern: pure SQL aggregation, no I/O beyond the connection.
//! Returns a `Stats` struct with per-type counts and supersede ratios.

use anyhow::Result;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::block::BlockType;

/// Per-type and global aggregate counts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub total_active: i64,
    pub total_superseded: i64,
    pub by_type: BTreeMap<String, TypeStats>,
    pub schema_version: u32,
}

/// Per-block-type counts and timestamp bracket.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeStats {
    pub active: i64,
    pub superseded: i64,
    pub oldest: Option<i64>,
    pub newest: Option<i64>,
}

/// Compute aggregates for the entire `blocks` table.
pub fn compute_stats(conn: &Connection) -> Result<Stats> {
    let total_active = scalar_count(conn, "WHERE superseded_by IS NULL")?;
    let total_superseded = scalar_count(conn, "WHERE superseded_by IS NOT NULL")?;
    let mut by_type: BTreeMap<String, TypeStats> = BTreeMap::new();
    for bt in BlockType::all() {
        by_type.insert(bt.to_string(), per_type_stats(conn, *bt)?);
    }
    Ok(Stats {
        total_active,
        total_superseded,
        by_type,
        schema_version: crate::store::SCHEMA_VERSION,
    })
}

fn scalar_count(conn: &Connection, where_clause: &str) -> Result<i64> {
    let sql = format!("SELECT COUNT(*) FROM blocks {where_clause}");
    let n: i64 = conn.query_row(&sql, [], |r| r.get(0))?;
    Ok(n)
}

fn per_type_stats(conn: &Connection, block_type: BlockType) -> Result<TypeStats> {
    let active: i64 = conn.query_row(
        "SELECT COUNT(*) FROM blocks WHERE block_type = ?1 AND superseded_by IS NULL",
        [block_type.as_str()],
        |r| r.get(0),
    )?;
    let superseded: i64 = conn.query_row(
        "SELECT COUNT(*) FROM blocks WHERE block_type = ?1 AND superseded_by IS NOT NULL",
        [block_type.as_str()],
        |r| r.get(0),
    )?;
    let bracket: (Option<i64>, Option<i64>) = conn.query_row(
        "SELECT MIN(created), MAX(created) FROM blocks WHERE block_type = ?1",
        [block_type.as_str()],
        |r| Ok((r.get(0)?, r.get(1)?)),
    )?;
    Ok(TypeStats {
        active,
        superseded,
        oldest: bracket.0,
        newest: bracket.1,
    })
}
