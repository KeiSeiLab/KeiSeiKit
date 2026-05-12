// SPDX-License-Identifier: Apache-2.0
//! Synchronous SQL operations for the buddy store.
//!
//! Constructor Pattern: pure data-access functions, no async, no traits.
//! These are called from `spawn_blocking` closures in `store.rs`.

use rusqlite::Connection;

use crate::error::BuddyError;
use crate::state::OnboardState;

/// Unix epoch seconds.
pub(crate) fn now_epoch() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time before UNIX epoch")
        .as_secs() as i64
}

/// Read the onboarding state for `chat_id`. Returns `None` if no row.
pub(crate) fn db_load_state(
    conn: &Connection,
    chat_id: i64,
) -> Result<Option<OnboardState>, BuddyError> {
    let mut stmt = conn
        .prepare("SELECT state FROM buddy_state WHERE chat_id = ?1")
        .map_err(|e| BuddyError::Memory(e.to_string()))?;
    let mut rows = stmt
        .query([chat_id])
        .map_err(|e| BuddyError::Memory(e.to_string()))?;
    match rows.next().map_err(|e| BuddyError::Memory(e.to_string()))? {
        None => Ok(None),
        Some(row) => {
            let text: String = row.get(0).map_err(|e| BuddyError::Memory(e.to_string()))?;
            let state: OnboardState = serde_json::from_str(&text)
                .map_err(|e| BuddyError::Memory(e.to_string()))?;
            Ok(Some(state))
        }
    }
}

/// Upsert the onboarding state for `chat_id`.
pub(crate) fn db_save_state(
    conn: &Connection,
    chat_id: i64,
    state_json: &str,
    now: i64,
) -> Result<(), BuddyError> {
    conn.execute(
        "INSERT INTO buddy_state (chat_id, state, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?3)
         ON CONFLICT(chat_id) DO UPDATE SET
             state      = excluded.state,
             updated_at = excluded.updated_at",
        rusqlite::params![chat_id, state_json, now],
    )
    .map_err(|e| BuddyError::Memory(e.to_string()))?;
    Ok(())
}

/// Read the persona blob for `chat_id`. Returns `None` if not set.
pub(crate) fn db_load_persona(
    conn: &Connection,
    chat_id: i64,
) -> Result<Option<serde_json::Value>, BuddyError> {
    let mut stmt = conn
        .prepare("SELECT persona FROM buddy_state WHERE chat_id = ?1")
        .map_err(|e| BuddyError::Memory(e.to_string()))?;
    let mut rows = stmt
        .query([chat_id])
        .map_err(|e| BuddyError::Memory(e.to_string()))?;
    match rows.next().map_err(|e| BuddyError::Memory(e.to_string()))? {
        None => Ok(None),
        Some(row) => {
            let opt: Option<String> =
                row.get(0).map_err(|e| BuddyError::Memory(e.to_string()))?;
            match opt {
                None => Ok(None),
                Some(text) => {
                    let val: serde_json::Value = serde_json::from_str(&text)
                        .map_err(|e| BuddyError::Memory(e.to_string()))?;
                    Ok(Some(val))
                }
            }
        }
    }
}

/// Upsert the persona blob for `chat_id`. If no row exists yet, seeds
/// state with `"intro"` as a placeholder.
pub(crate) fn db_save_persona(
    conn: &Connection,
    chat_id: i64,
    persona_json: &str,
    now: i64,
) -> Result<(), BuddyError> {
    conn.execute(
        "INSERT INTO buddy_state (chat_id, state, persona, created_at, updated_at)
         VALUES (?1, '\"intro\"', ?2, ?3, ?3)
         ON CONFLICT(chat_id) DO UPDATE SET
             persona    = excluded.persona,
             updated_at = excluded.updated_at",
        rusqlite::params![chat_id, persona_json, now],
    )
    .map_err(|e| BuddyError::Memory(e.to_string()))?;
    Ok(())
}
