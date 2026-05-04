//! Block formula 4-tuple per `arch/MATH-DNA-DESIGN.md` §1.1.
//!
//! Public API:
//!   * [`register_formula`] — atomically write a `BlockFormula` to a row in
//!     `blocks` and replace its rows in `block_predicates` + `block_deps`.
//!   * [`load_formula`] — reverse-trip a row back into a `BlockFormula`,
//!     returning `Ok(None)` when no formula has been registered.
//!
//! Constructor Pattern: this module is the public surface; the type
//! catalogue lives in [`types`], the SHA function in [`sha`], and the SQL
//! plumbing in [`io`]. Each helper is one cube with one responsibility.

mod io;
mod sha;
mod types;

pub use sha::formula_sha;
pub use types::{
    BlockFormula, EffectKind, FormulaSource, Predicate, SymbolKind, TypeAtom, TypeSignature,
};

use anyhow::Result;
use rusqlite::Connection;

/// Register a formula for a block. Wraps the three writes in a transaction
/// so a crash mid-write leaves the row intact under its previous formula
/// (or no formula at all).
///
/// The replacement semantics are deliberately destructive: any prior
/// predicates + deps for `formula.block_id` are removed before the new
/// ones are inserted. This matches the "formula is the truth, derive or
/// re-declare" contract — callers should not partially patch a formula.
pub fn register_formula(conn: &Connection, formula: &BlockFormula) -> Result<()> {
    let tx = conn.unchecked_transaction()?;
    io::write_block_columns(&tx, formula)?;
    io::replace_predicates(&tx, formula.block_id, &formula.invariants)?;
    io::replace_deps(&tx, formula.block_id, &formula.deps)?;
    tx.commit()?;
    Ok(())
}

/// Load the formula for a block. Returns `Ok(None)` when the block exists
/// but has no formula (i.e. `formula_sha IS NULL`); returns an `Err` only
/// on storage faults or malformed JSON.
pub fn load_formula(conn: &Connection, block_id: i64) -> Result<Option<BlockFormula>> {
    let Some((type_sig, effects, source)) = io::read_block_columns(conn, block_id)? else {
        return Ok(None);
    };
    let invariants = io::read_predicates(conn, block_id)?;
    let deps = io::read_deps(conn, block_id)?;
    Ok(Some(BlockFormula {
        block_id,
        r#type: type_sig,
        invariants,
        effects,
        deps,
        source,
    }))
}
