//! Thin read-only client over `~/.claude/registry.sqlite`.
//!
//! Fetches rule-fragment content by logical name (`rule::section`).
//! The registry stores the real filesystem path; this module reads that path.
//!
//! Constructor Pattern: one responsibility — lookup + read fragment body.
//! No writes. No schema migration. Opens DB read-only.

use rusqlite::{Connection, OpenFlags};
use std::path::{Path, PathBuf};

/// Open the registry at `db_path` in read-only mode.
pub fn open_read_only(db_path: &Path) -> Result<Connection, String> {
    Connection::open_with_flags(db_path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .map_err(|e| format!("open registry {}: {e}", db_path.display()))
}

/// Default path: `$KEI_REGISTRY_DB` (if set) or `~/.claude/registry.sqlite`.
pub fn default_db_path() -> PathBuf {
    if let Some(v) = std::env::var_os("KEI_REGISTRY_DB") {
        return PathBuf::from(v);
    }
    let home = std::env::var_os("HOME").unwrap_or_default();
    PathBuf::from(home).join(".claude/registry.sqlite")
}

/// Look up a rule fragment by `name` (e.g. `"karpathy-behavioral::1-think-before-coding"`).
///
/// Returns:
/// - `Ok(Some(body))` — fragment found and file readable.
/// - `Ok(None)` — name not in registry, or registry path does not exist on disk.
///   Caller should warn-and-skip.
/// - `Err(msg)` — DB query failure (not a missing-path issue). Propagate.
pub fn find_rule(conn: &Connection, name: &str) -> Result<Option<String>, String> {
    let path = match query_path(conn, name)? {
        Some(p) => p,
        None => return Ok(None),
    };
    read_fragment_body(name, &path)
}

/// Query the `path` column for the active row with `name` and `block_type='rule'`.
fn query_path(conn: &Connection, name: &str) -> Result<Option<String>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT path FROM blocks \
             WHERE name = ?1 AND block_type = 'rule' AND superseded_by IS NULL \
             LIMIT 1",
        )
        .map_err(|e| format!("prepare query for {name}: {e}"))?;
    let row: Option<String> = stmt
        .query_row(rusqlite::params![name], |r| r.get(0))
        .optional()
        .map_err(|e| format!("query registry for {name}: {e}"))?;
    Ok(row)
}

/// Read the fragment body from `path`. Returns `Ok(None)` when the file is absent.
fn read_fragment_body(name: &str, path: &str) -> Result<Option<String>, String> {
    match std::fs::read_to_string(path) {
        Ok(body) => Ok(Some(body)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            eprintln!(
                "warn [assembler]: registry fragment for '{name}' has path '{path}' but file is missing — skipping. \
                 Run `kei-decompose decompose-rules --rebuild-fragments` to restore."
            );
            Ok(None)
        }
        Err(e) => Err(format!("read fragment for {name} at {path}: {e}")),
    }
}

trait OptionalExt<T>: Sized {
    fn optional(self) -> rusqlite::Result<Option<T>>;
}

impl<T> OptionalExt<T> for rusqlite::Result<T> {
    fn optional(self) -> rusqlite::Result<Option<T>> {
        match self {
            Ok(v) => Ok(Some(v)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }
}
