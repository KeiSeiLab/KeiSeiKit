//! Kernel-smoothed posterior fallback for the empirical selector.
//!
//! When a task-class has no direct ledger entries, borrows posterior mass
//! from neighbouring task-classes weighted by DNA similarity.
//!
//! Constructor Pattern: SQL cube — separated from select.rs to keep both files <200 LOC.

use crate::kernel::{self, KernelWeights};
use crate::posterior::Posterior;
use crate::pricing::Model;
use rusqlite::{Connection, Result as SqlResult};

const QUERY: &str = "SELECT task_class_dna,
        SUM(CASE WHEN outcome = 'functional'
                  AND COALESCE(escalation_depth, 0) = 0
                 THEN 1 ELSE 0 END) AS np,
        SUM(CASE WHEN outcome IS NOT NULL
                  AND NOT (outcome = 'functional'
                           AND COALESCE(escalation_depth, 0) = 0)
                 THEN 1 ELSE 0 END) AS nm
 FROM agents
 WHERE task_class_dna IS NOT NULL
   AND task_class_dna != ?1
   AND model = ?2
 GROUP BY task_class_dna";

/// Weighted-sum posterior borrowing from neighbour task-classes.
///
/// Returns a Beta posterior with `alpha`/`beta` inflated by kernel similarity.
/// Starts from a uniform prior (alpha=1, beta=1) and accumulates evidence.
pub fn smooth(
    conn: &Connection,
    target_task_class: &str,
    model: Model,
    weights: KernelWeights,
) -> SqlResult<Posterior> {
    let mut stmt = conn.prepare(QUERY)?;

    let rows = stmt.query_map(
        rusqlite::params![target_task_class, model.slug()],
        |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, Option<i64>>(1)?.unwrap_or(0),
                r.get::<_, Option<i64>>(2)?.unwrap_or(0),
            ))
        },
    )?;

    accumulate_weighted(rows, target_task_class, weights)
}

fn accumulate_weighted(
    rows: impl Iterator<Item = rusqlite::Result<(String, i64, i64)>>,
    target: &str,
    weights: KernelWeights,
) -> SqlResult<Posterior> {
    let mut alpha = 1.0_f64;
    let mut beta = 1.0_f64;
    let mut n = 0_u32;

    for row in rows {
        let (other_tc, np, nm) = row?;
        let sim = kernel::similarity(target, &other_tc, weights);
        if sim <= 0.0 {
            continue;
        }
        alpha += sim * np as f64;
        beta += sim * nm as f64;
        n = n.saturating_add((np + nm) as u32);
    }

    Ok(Posterior { alpha, beta, n })
}
