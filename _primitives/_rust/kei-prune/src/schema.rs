//! Schema for the pruning sidecar.
//!
//! Constructor Pattern: one cube = sidecar DDL + idempotent installer.
//! We deliberately do NOT touch the `agents` table CHECK constraint —
//! that is owned by kei-ledger and cannot be extended from outside
//! without risking a migration desync. Instead we keep a lightweight
//! companion table keyed on the same `agent_id`.

use crate::error::PruneError;
use rusqlite::Connection;

/// DDL for the `prune_retirements` sidecar.
///
/// - `agent_id` — matches `agents.id` (no FK because SQLite foreign_keys
///   pragma is often off; we validate in `mark_retired` instead).
/// - `retired_ts` — unix-seconds timestamp when retirement was recorded.
///
/// The table is append-only in spirit: `mark_retired` is idempotent and
/// leaves the original `retired_ts` untouched on repeat calls.
pub const SIDECAR_DDL: &str = "\
CREATE TABLE IF NOT EXISTS prune_retirements (
    agent_id TEXT PRIMARY KEY,
    retired_ts INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_prune_retired_ts ON prune_retirements(retired_ts);
";

/// Create the sidecar table if it does not yet exist.
///
/// Idempotent: safe to call on every CLI invocation. Does not migrate
/// — if the shape ever needs to change we add a migration runner here
/// mirroring kei-ledger's pattern.
pub fn ensure_schema(conn: &Connection) -> Result<(), PruneError> {
    conn.execute_batch(SIDECAR_DDL)?;
    Ok(())
}
