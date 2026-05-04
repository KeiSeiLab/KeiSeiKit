//! Formula persistence helpers — write and read against `blocks`,
//! `block_predicates`, `block_deps` (schema v2-v4).
//!
//! Constructor Pattern: each helper is one SQL operation; the public
//! `register_formula` / `load_formula` in `mod.rs` compose them inside a
//! single transaction.

use anyhow::Result;
use rusqlite::{params, Connection, OptionalExtension};
use std::collections::BTreeSet;

use super::sha::formula_sha;
use super::types::{predicate_kind, BlockFormula, EffectKind, FormulaSource, Predicate, TypeSignature};

/// Update the four formula columns on a `blocks` row. Does NOT touch
/// `block_predicates` / `block_deps` — caller composes the full write.
pub(super) fn write_block_columns(conn: &Connection, formula: &BlockFormula) -> Result<()> {
    let sha = formula_sha(formula);
    conn.execute(
        "UPDATE blocks SET type_sig_json = ?1, effects_json = ?2, \
         formula_source = ?3, formula_sha = ?4 WHERE id = ?5",
        params![
            serde_json::to_string(&formula.r#type)?,
            serde_json::to_string(&formula.effects)?,
            serde_json::to_string(&formula.source)?,
            sha,
            formula.block_id,
        ],
    )?;
    Ok(())
}

/// Replace all predicate rows for `block_id` with the supplied list.
pub(super) fn replace_predicates(
    conn: &Connection,
    block_id: i64,
    invariants: &[Predicate],
) -> Result<()> {
    conn.execute(
        "DELETE FROM block_predicates WHERE block_id = ?1",
        params![block_id],
    )?;
    for (seq, pred) in invariants.iter().enumerate() {
        conn.execute(
            "INSERT INTO block_predicates (block_id, seq, kind, args_json) \
             VALUES (?1, ?2, ?3, ?4)",
            params![block_id, seq as i64, predicate_kind(pred), serde_json::to_string(pred)?],
        )?;
    }
    Ok(())
}

/// Replace all dep rows for `block_id` with the supplied set, marking each
/// as `dep_kind = "runtime"`. Future variants may set other kinds; today
/// `register_formula` writes runtime-only.
pub(super) fn replace_deps(conn: &Connection, block_id: i64, deps: &BTreeSet<String>) -> Result<()> {
    conn.execute(
        "DELETE FROM block_deps WHERE block_id = ?1",
        params![block_id],
    )?;
    for dep_dna in deps {
        conn.execute(
            "INSERT INTO block_deps (block_id, dep_dna, dep_kind) VALUES (?1, ?2, ?3)",
            params![block_id, dep_dna, "runtime"],
        )?;
    }
    Ok(())
}

/// Read the four formula columns from `blocks`. Returns `None` when
/// `formula_sha` is NULL — i.e. no formula has been registered yet.
pub(super) fn read_block_columns(
    conn: &Connection,
    block_id: i64,
) -> Result<Option<(TypeSignature, BTreeSet<EffectKind>, FormulaSource)>> {
    let row: Option<(Option<String>, Option<String>, Option<String>, Option<String>)> = conn
        .query_row(
            "SELECT type_sig_json, effects_json, formula_source, formula_sha \
             FROM blocks WHERE id = ?1",
            params![block_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
        )
        .optional()?;
    let Some((Some(ts), Some(eff), Some(src), Some(_sha))) = row else {
        return Ok(None);
    };
    Ok(Some((
        serde_json::from_str(&ts)?,
        serde_json::from_str(&eff)?,
        serde_json::from_str(&src)?,
    )))
}

/// Read predicates for a block, ordered by `seq` so the round-trip order
/// matches the original write.
pub(super) fn read_predicates(conn: &Connection, block_id: i64) -> Result<Vec<Predicate>> {
    let mut stmt = conn.prepare(
        "SELECT args_json FROM block_predicates WHERE block_id = ?1 ORDER BY seq ASC",
    )?;
    let rows = stmt.query_map(params![block_id], |r| r.get::<_, String>(0))?;
    let mut out = Vec::new();
    for row in rows {
        out.push(serde_json::from_str(&row?)?);
    }
    Ok(out)
}

/// Read all dep DNAs for a block. Returns a `BTreeSet` so callers see them
/// in canonical sorted order.
pub(super) fn read_deps(conn: &Connection, block_id: i64) -> Result<BTreeSet<String>> {
    let mut stmt =
        conn.prepare("SELECT dep_dna FROM block_deps WHERE block_id = ?1")?;
    let rows = stmt.query_map(params![block_id], |r| r.get::<_, String>(0))?;
    let mut out = BTreeSet::new();
    for row in rows {
        out.insert(row?);
    }
    Ok(out)
}
