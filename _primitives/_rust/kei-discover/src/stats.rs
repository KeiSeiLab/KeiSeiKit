//! `stats` — aggregate counts (total / installed / available).
//!
//! One-row SELECT with conditional SUMs. `available` equals
//! `total - installed`, kept as an explicit field so CLI users don't
//! have to subtract.

use crate::error::DiscoverError;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stats {
    pub total: i64,
    pub installed: i64,
    pub available: i64,
}

const SQL: &str = "\
    SELECT COUNT(*) AS total, \
           COALESCE(SUM(CASE WHEN installed = 1 THEN 1 ELSE 0 END), 0) AS installed \
    FROM discover_index";

pub fn stats(conn: &Connection) -> Result<Stats, DiscoverError> {
    let (total, installed): (i64, i64) = conn
        .query_row(SQL, [], |r| Ok((r.get::<_, i64>(0)?, r.get::<_, i64>(1)?)))?;
    Ok(Stats {
        total,
        installed,
        available: total - installed,
    })
}
