//! Block lookup by id-or-DNA-or-path.
//!
//! Constructor Pattern: this cube owns the polymorphic CLI lookup. Many
//! handlers accept "either an integer id or a full DNA string" — and a
//! few also support a filesystem path — so this helper centralises the
//! parse-cascade. Order: parse as i64 → DNA exact match → existing path.

use anyhow::Result;
use rusqlite::Connection;
use std::path::Path;

use crate::block::Block;
use crate::registry::{find_by_path, get, get_by_dna};

/// Resolve a CLI target to a Block. Returns `None` if no row matches any
/// of the three lookup strategies.
pub fn lookup_block(conn: &Connection, target: &str) -> Result<Option<Block>> {
    if let Ok(id) = target.parse::<i64>() {
        if let Some(b) = get(conn, id)? {
            return Ok(Some(b));
        }
    }
    if let Some(b) = get_by_dna(conn, target)? {
        return Ok(Some(b));
    }
    if Path::new(target).exists() {
        if let Some(b) = find_by_path(conn, target)? {
            return Ok(Some(b));
        }
    }
    Ok(None)
}
