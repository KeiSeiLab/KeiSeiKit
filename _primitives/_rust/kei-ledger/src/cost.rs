//! `record_cost` — write cost-tracking columns for an existing agent row.
//!
//! Constructor Pattern: one cube = one mutator. Wave 44c (2026-04-24)
//! flipped the semantics from last-write-wins to ADDITIVE — the chat
//! handler under-counted multi-turn conversations because each turn
//! UPDATEd the same `conversation_id`-derived row, overwriting the
//! prior turn's cents. Now consecutive `record_cost` calls accumulate;
//! a separate `replace_cost` exists for the (rare) explicit-overwrite
//! flow that previously used `record_cost`.
//!
//! Schema dependency: requires the v7 migration (`cost_cents` /
//! `cost_micro_cents` / `provider` / `model` columns on the `agents`
//! table). Caller is expected to have opened the ledger via
//! `ledger::open()` so migrations run before this function is reachable.
//!
//! Why additive (vs MAX or last-write): every chat turn under the same
//! `conversation_id` shares a row. With last-write-wins, a 10-turn
//! conversation billed only the final turn — silent under-charge in
//! `/usage` aggregation. ADD avoids both that under-charge AND the
//! double-count risk of legacy retry overwrites by exposing
//! `replace_cost` as the explicit "I am rewriting, not adding" call.

use rusqlite::{params, Connection, Result as SqlResult};

/// 1 cent = 1_000_000 micro-cents. Public so external tooling (the
/// `/usage` JSON serializer in particular) can convert without needing
/// to hard-code the magic number.
pub const MICRO_CENTS_PER_CENT: u64 = 1_000_000;

/// Add cost-tracking metadata to `agent_id`. Callers that ALREADY have
/// a cents value should also pass `micro_cents = cents * MICRO_CENTS_PER_CENT`
/// to keep both columns coherent. Use `compose_micro_cents` if you only
/// have token counts.
///
/// Returns the number of rows updated (0 = no agent matched, 1 = success).
/// Provider / model are LAST-WRITE: only the cost columns accumulate, so
/// switching providers mid-conversation reflects the latest provider in
/// the row while still summing all turns' cost. The agent must already
/// exist (created via `fork`); this function does NOT insert.
pub fn record_cost(
    conn: &Connection,
    agent_id: &str,
    cents: u64,
    provider: &str,
    model: &str,
) -> SqlResult<usize> {
    let micro = cents.saturating_mul(MICRO_CENTS_PER_CENT);
    record_cost_micro(conn, agent_id, cents, micro, provider, model)
}

/// Same as `record_cost` but accepts the EXACT micro-cents accumulator.
/// `cents` is the rounded display value the caller already computed via
/// `display_cents_from_micro`. Both columns increment in one UPDATE so
/// the row stays internally consistent under concurrent writes.
pub fn record_cost_micro(
    conn: &Connection,
    agent_id: &str,
    cents: u64,
    micro_cents: u64,
    provider: &str,
    model: &str,
) -> SqlResult<usize> {
    let safe_cents: i64 = cents.min(i64::MAX as u64) as i64;
    let safe_micro: i64 = micro_cents.min(i64::MAX as u64) as i64;
    conn.execute(
        "UPDATE agents
         SET cost_cents = COALESCE(cost_cents, 0) + ?1,
             cost_micro_cents = COALESCE(cost_micro_cents, 0) + ?2,
             provider = ?3,
             model = ?4
         WHERE id = ?5",
        params![safe_cents, safe_micro, provider, model, agent_id],
    )
}

/// Explicit OVERWRITE of cost columns. Use ONLY for retry / amend
/// flows where the prior write recorded a partial estimate that the
/// caller now wants to replace wholesale. All other callers should use
/// `record_cost` (additive). Returns rows updated (0 = no match).
pub fn replace_cost(
    conn: &Connection,
    agent_id: &str,
    cents: u64,
    provider: &str,
    model: &str,
) -> SqlResult<usize> {
    let micro = cents.saturating_mul(MICRO_CENTS_PER_CENT);
    replace_cost_micro(conn, agent_id, cents, micro, provider, model)
}

/// Same as `replace_cost` but with explicit micro-cents value (no
/// derivation from `cents`). Call this when the caller computed the
/// exact micro-cents from token counts and wants the row's micro-cent
/// column to reflect that exact value rather than `cents × 1M`.
pub fn replace_cost_micro(
    conn: &Connection,
    agent_id: &str,
    cents: u64,
    micro_cents: u64,
    provider: &str,
    model: &str,
) -> SqlResult<usize> {
    let safe_cents: i64 = cents.min(i64::MAX as u64) as i64;
    let safe_micro: i64 = micro_cents.min(i64::MAX as u64) as i64;
    conn.execute(
        "UPDATE agents
         SET cost_cents = ?1,
             cost_micro_cents = ?2,
             provider = ?3,
             model = ?4
         WHERE id = ?5",
        params![safe_cents, safe_micro, provider, model, agent_id],
    )
}

/// Read back the (cost_cents, provider, model) tuple for `agent_id`.
/// Returns `None` if the agent does not exist. Used by the CLI's JSON
/// response to confirm the write took effect. Callers that need
/// micro-cent resolution use `read_cost_micro` instead.
pub fn read_cost(
    conn: &Connection,
    agent_id: &str,
) -> SqlResult<Option<(i64, String, String)>> {
    use rusqlite::OptionalExtension;
    conn.query_row(
        "SELECT COALESCE(cost_cents, 0), COALESCE(provider, ''), COALESCE(model, '')
         FROM agents WHERE id = ?1",
        params![agent_id],
        |r| Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?, r.get::<_, String>(2)?)),
    )
    .optional()
}

/// Read both the cents and micro-cents columns alongside provider and
/// model. The `/usage` API boundary calls this so it can render rounded
/// dollar amounts from the exact micro-cent accumulator.
pub fn read_cost_micro(
    conn: &Connection,
    agent_id: &str,
) -> SqlResult<Option<(i64, i64, String, String)>> {
    use rusqlite::OptionalExtension;
    conn.query_row(
        "SELECT COALESCE(cost_cents, 0),
                COALESCE(cost_micro_cents, 0),
                COALESCE(provider, ''),
                COALESCE(model, '')
         FROM agents WHERE id = ?1",
        params![agent_id],
        |r| {
            Ok((
                r.get::<_, i64>(0)?,
                r.get::<_, i64>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, String>(3)?,
            ))
        },
    )
    .optional()
}

/// Compute EXACT micro-cents from a token usage and per-MTok cents
/// rates. No rounding loss — `tokens × cents_per_m` is exact in u128
/// before truncating to u64 micro-cents.
pub fn compose_micro_cents(
    input_tokens: u64,
    output_tokens: u64,
    in_cents_per_m: u32,
    out_cents_per_m: u32,
) -> u64 {
    let in_micro = (input_tokens as u128) * (in_cents_per_m as u128);
    let out_micro = (output_tokens as u128) * (out_cents_per_m as u128);
    let total = in_micro.saturating_add(out_micro);
    total.min(u64::MAX as u128) as u64
}

/// Render a micro-cents accumulator as a display cents value. Uses
/// banker's rounding via integer truncation + half-up: `(micro + 500_000) / 1M`.
/// API boundary lives at `/usage` — internal arithmetic stays exact.
pub fn display_cents_from_micro(micro_cents: u64) -> u64 {
    (micro_cents.saturating_add(MICRO_CENTS_PER_CENT / 2)) / MICRO_CENTS_PER_CENT
}
