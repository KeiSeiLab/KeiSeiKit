//! Candidate row returned by `candidates()`.
//!
//! Constructor Pattern: one cube = one DTO. Serializable so CLI can
//! emit JSON without extra mapping.

use serde::{Deserialize, Serialize};

/// A single pruning candidate — an agent that is eligible for retirement
/// based on age + status + not-yet-retired.
///
/// Fields:
/// - `id` — ledger `agents.id` (primary key).
/// - `dna` — ledger `agents.dna` (may be empty string when NULL in DB).
/// - `last_used_ts` — `COALESCE(finished_ts, started_ts)` for the row.
/// - `age_days` — `(now - started_ts) / 86400` (integer truncation).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PruneCandidate {
    pub id: String,
    pub dna: String,
    pub last_used_ts: i64,
    pub age_days: i64,
}
