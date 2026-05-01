//! Block CRUD over the registry SQLite store.
//!
//! Constructor Pattern: this cube owns the SQL row mapping + register
//! idempotency rule. Schema lives in `store.rs`; DNA composition in
//! `dna_block.rs`. The supersede chain is the only non-trivial state
//! transition — see `register` below.

use anyhow::{Context, Result};
use rusqlite::{params, Connection, OptionalExtension, Row};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::block::{Block, BlockType};
use crate::dna_block::{compose_for_block_with_nonce, fresh_nonce, short_sha8};

const SELECT_COLS: &str = "id, dna, block_type, name, path, caps, scope_sha, body_sha, \
                           nonce, created, modified, superseded_by";

fn now_secs() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

fn row_to_block(row: &Row) -> rusqlite::Result<Block> {
    let block_type_str: String = row.get(2)?;
    Ok(Block {
        id: row.get(0)?,
        dna: row.get(1)?,
        block_type: BlockType::from_str(&block_type_str).map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(2, rusqlite::types::Type::Text, e.into())
        })?,
        name: row.get(3)?,
        path: row.get(4)?,
        caps: row.get(5)?,
        scope_sha: row.get(6)?,
        body_sha: row.get(7)?,
        nonce: row.get(8)?,
        created: row.get(9)?,
        modified: row.get(10)?,
        superseded_by: row.get(11)?,
    })
}

/// Register a block. Idempotency rule:
/// 1. If active row exists with matching (path, body_sha) → return it untouched.
/// 2. Else if active row exists with matching path but different body_sha
///    → mark old row superseded_by(new DNA), insert new row.
/// 3. Else → fresh insert with new nonce.
pub fn register(
    conn: &Connection,
    block_type: BlockType,
    name: &str,
    path: &str,
    body: &[u8],
    caps: &str,
) -> Result<Block> {
    let body_sha = short_sha8(body);
    if let Some(existing) = active_by_path(conn, path)? {
        if existing.body_sha == body_sha {
            return Ok(existing);
        }
        return supersede_and_insert(conn, &existing, block_type, name, path, body, caps);
    }
    insert_fresh(conn, block_type, name, path, body, caps)
}

fn active_by_path(conn: &Connection, path: &str) -> Result<Option<Block>> {
    let mut stmt = conn.prepare(&format!(
        "SELECT {SELECT_COLS} FROM blocks WHERE path = ?1 AND superseded_by IS NULL LIMIT 1"
    ))?;
    let row = stmt.query_row(params![path], row_to_block).optional()?;
    Ok(row)
}

fn insert_fresh(
    conn: &Connection,
    block_type: BlockType,
    name: &str,
    path: &str,
    body: &[u8],
    caps: &str,
) -> Result<Block> {
    let nonce = fresh_nonce();
    let dna = compose_for_block_with_nonce(block_type, name, path, body, caps, &nonce);
    let body_sha = short_sha8(body);
    let scope_sha = short_sha8(path.as_bytes());
    do_insert(conn, &dna, block_type, name, path, caps, &scope_sha, &body_sha, &nonce)?;
    get_by_dna(conn, &dna)?.context("re-fetch after insert")
}

#[allow(clippy::too_many_arguments)]
fn do_insert(
    conn: &Connection,
    dna: &str,
    block_type: BlockType,
    name: &str,
    path: &str,
    caps: &str,
    scope_sha: &str,
    body_sha: &str,
    nonce: &str,
) -> Result<()> {
    let now = now_secs();
    conn.execute(
        "INSERT INTO blocks (dna, block_type, name, path, caps, scope_sha, body_sha, nonce, \
         created, modified, superseded_by) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, NULL)",
        params![dna, block_type.as_str(), name, path, caps, scope_sha, body_sha, nonce, now, now],
    )
    .with_context(|| format!("insert block {} {}", block_type, path))?;
    Ok(())
}

fn supersede_and_insert(
    conn: &Connection,
    old: &Block,
    block_type: BlockType,
    name: &str,
    path: &str,
    body: &[u8],
    caps: &str,
) -> Result<Block> {
    let new_block = insert_fresh(conn, block_type, name, path, body, caps)?;
    mark_superseded(conn, old.id, &new_block.dna)?;
    Ok(new_block)
}

/// Set `superseded_by` on the row with id `old_id` to `new_dna`. Touches
/// `modified`. Caller is responsible for ensuring `new_dna` exists.
pub fn mark_superseded(conn: &Connection, old_id: i64, new_dna: &str) -> Result<()> {
    let now = now_secs();
    conn.execute(
        "UPDATE blocks SET superseded_by = ?1, modified = ?2 WHERE id = ?3",
        params![new_dna, now, old_id],
    )?;
    Ok(())
}

/// Fetch a block by integer id.
pub fn get(conn: &Connection, id: i64) -> Result<Option<Block>> {
    let mut stmt =
        conn.prepare(&format!("SELECT {SELECT_COLS} FROM blocks WHERE id = ?1 LIMIT 1"))?;
    Ok(stmt.query_row(params![id], row_to_block).optional()?)
}

/// Fetch a block by full DNA string.
pub fn get_by_dna(conn: &Connection, dna: &str) -> Result<Option<Block>> {
    let mut stmt = conn.prepare(&format!(
        "SELECT {SELECT_COLS} FROM blocks WHERE dna = ?1 LIMIT 1"
    ))?;
    Ok(stmt.query_row(params![dna], row_to_block).optional()?)
}

/// List active blocks (or all if `include_superseded` is true), bounded by `limit`.
pub fn list(conn: &Connection, include_superseded: bool, limit: i64) -> Result<Vec<Block>> {
    let where_clause = if include_superseded {
        ""
    } else {
        " WHERE superseded_by IS NULL"
    };
    let sql = format!("SELECT {SELECT_COLS} FROM blocks{where_clause} ORDER BY id ASC LIMIT ?1");
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params![limit], row_to_block)?;
    Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
}

/// List all active blocks of one type (no limit; caller paginates if needed).
pub fn list_by_type(conn: &Connection, block_type: BlockType) -> Result<Vec<Block>> {
    let sql = format!(
        "SELECT {SELECT_COLS} FROM blocks WHERE block_type = ?1 AND superseded_by IS NULL \
         ORDER BY name ASC"
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params![block_type.as_str()], row_to_block)?;
    Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
}

/// Find the active row at `path`, if any.
pub fn find_by_path(conn: &Connection, path: &str) -> Result<Option<Block>> {
    active_by_path(conn, path)
}
