//! Debug-only invariant helpers for the `update` verb.
//!
//! Split out of `verbs/update.rs` so that file stays within the
//! Constructor-Pattern 200-LOC cap. The functions here encode the FTS
//! reindex contract: columns NOT present in an UPDATE's input JSON
//! must not change during the UPDATE.
//!
//! `cfg(debug_assertions)` gates both the snapshot SELECT and the
//! assertion itself — release builds compile this module down to a
//! no-op snapshot that returns an empty map.

use crate::schema::EntitySchema;
use crate::verbs::pk::PkValue;
use serde_json::Value;

/// Snapshot FTS columns BEFORE an UPDATE runs. Debug builds read the
/// row via `read_existing_fts`; release builds skip the read and
/// return an empty map.
#[cfg(debug_assertions)]
pub(super) fn pre_update_snapshot(
    tx: &rusqlite::Transaction<'_>,
    schema: &EntitySchema,
    id: &PkValue,
) -> serde_json::Map<String, Value> {
    let Some(cols) = schema.fts_columns else {
        return serde_json::Map::new();
    };
    super::update::read_existing_fts(tx, schema, cols, id).unwrap_or_default()
}

#[cfg(not(debug_assertions))]
pub(super) fn pre_update_snapshot(
    _tx: &rusqlite::Transaction<'_>,
    _schema: &EntitySchema,
    _id: &PkValue,
) -> serde_json::Map<String, Value> {
    serde_json::Map::new()
}

/// Debug-only invariant check: every FTS column NOT present in `input`
/// must still hold its pre-UPDATE value after the UPDATE completes.
/// If this fires, a non-input column changed under the UPDATE (trigger,
/// computed column, etc.) and the `reindex_fts` contract is broken.
#[cfg(debug_assertions)]
pub(super) fn debug_assert_non_input_fts_stable(
    cols: &[&str],
    input: &serde_json::Map<String, Value>,
    pre_update: &serde_json::Map<String, Value>,
    existing: &serde_json::Map<String, Value>,
) {
    for c in cols {
        if input.contains_key(*c) {
            continue;
        }
        let pre = pre_update.get(*c);
        let post = existing.get(*c);
        debug_assert_eq!(
            pre, post,
            "reindex_fts invariant violated: non-input FTS column `{c}` \
             changed during UPDATE (pre: {pre:?}, post: {post:?}). A \
             trigger or computed column likely modified it. Snapshot \
             the row BEFORE the UPDATE instead of reading it after."
        );
    }
}
