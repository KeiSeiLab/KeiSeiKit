//! Offline calibration of kernel weights from observed ledger outcomes.
//!
//! Approach: leave-one-out on each ledger row, coarse grid search over
//! weight tuples (5 × 4 × 3 × 3 = 180 configs) minimising MSE.
//!
//! Constructor Pattern: pure-fn cube; no I/O outside passing a Connection.

use crate::kernel::{self, KernelWeights};
use crate::pricing::Model;
use rusqlite::{Connection, Result as SqlResult};

#[derive(Debug, Clone)]
pub struct CalibrationResult {
    pub best_weights: KernelWeights,
    pub best_mse: f64,
    pub baseline_mse: f64,
    pub rows_evaluated: usize,
}

#[derive(Debug, Clone)]
struct Observation {
    task_class: String,
    model: Model,
    success: bool,
}

pub fn calibrate(conn: &Connection) -> SqlResult<CalibrationResult> {
    let observations = load_observations(conn)?;
    let rows_evaluated = observations.len();
    if rows_evaluated < 5 {
        return Ok(CalibrationResult {
            best_weights: KernelWeights::default(),
            best_mse: f64::NAN,
            baseline_mse: f64::NAN,
            rows_evaluated,
        });
    }

    let baseline_mse = mse(&observations, KernelWeights::default());
    let mut best_weights = KernelWeights::default();
    let mut best_mse = baseline_mse;

    for ar in &[0.10, 0.25, 0.40, 0.55, 0.70] {
        for ac in &[0.05, 0.15, 0.25, 0.35] {
            for ascope in &[0.05, 0.15, 0.25] {
                for ab in &[0.0, 0.05, 0.10] {
                    let w = KernelWeights {
                        alpha_role: *ar,
                        alpha_caps: *ac,
                        alpha_scope: *ascope,
                        alpha_body: *ab,
                    };
                    let m = mse(&observations, w);
                    if m < best_mse {
                        best_mse = m;
                        best_weights = w;
                    }
                }
            }
        }
    }

    Ok(CalibrationResult { best_weights, best_mse, baseline_mse, rows_evaluated })
}

fn load_observations(conn: &Connection) -> SqlResult<Vec<Observation>> {
    let mut stmt = conn.prepare(
        "SELECT task_class_dna, model, outcome, COALESCE(escalation_depth, 0)
         FROM agents
         WHERE task_class_dna IS NOT NULL
           AND model IS NOT NULL AND model != ''
           AND outcome IS NOT NULL",
    )?;
    let rows = stmt.query_map([], |r| {
        Ok((
            r.get::<_, String>(0)?,
            r.get::<_, String>(1)?,
            r.get::<_, String>(2)?,
            r.get::<_, i64>(3)?,
        ))
    })?;
    let mut out = Vec::new();
    for row in rows {
        let (tc, model_slug, outcome, depth) = row?;
        let Some(model) = Model::from_slug(&model_slug) else { continue };
        let success = outcome == "functional" && depth == 0;
        out.push(Observation { task_class: tc, model, success });
    }
    Ok(out)
}

fn mse(observations: &[Observation], weights: KernelWeights) -> f64 {
    if observations.is_empty() {
        return 0.0;
    }
    let mut sum_sq = 0.0_f64;
    for (i, target) in observations.iter().enumerate() {
        let q_hat = predict_loo(observations, i, target, weights);
        let actual = if target.success { 1.0 } else { 0.0 };
        sum_sq += (actual - q_hat).powi(2);
    }
    sum_sq / observations.len() as f64
}

fn predict_loo(
    observations: &[Observation],
    skip: usize,
    target: &Observation,
    weights: KernelWeights,
) -> f64 {
    let mut weighted_alpha = 1.0_f64;
    let mut weighted_beta = 1.0_f64;
    for (j, obs) in observations.iter().enumerate() {
        if j == skip || obs.model != target.model {
            continue;
        }
        let sim = kernel::similarity(&target.task_class, &obs.task_class, weights);
        if sim <= 0.0 {
            continue;
        }
        if obs.success {
            weighted_alpha += sim;
        } else {
            weighted_beta += sim;
        }
    }
    weighted_alpha / (weighted_alpha + weighted_beta)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn fresh_db() -> Connection {
        let c = Connection::open_in_memory().unwrap();
        c.execute_batch(
            "CREATE TABLE agents (
                id TEXT, task_class_dna TEXT, model TEXT,
                outcome TEXT, escalation_depth INTEGER DEFAULT 0
            );",
        )
        .unwrap();
        c
    }

    #[test]
    fn empty_ledger_returns_default_weights() {
        let c = fresh_db();
        let r = calibrate(&c).unwrap();
        assert_eq!(r.rows_evaluated, 0);
        assert!(r.best_mse.is_nan());
    }

    #[test]
    fn calibration_improves_or_matches_baseline() {
        let c = fresh_db();
        let haiku = Model::Haiku45.slug();
        for i in 0..15 {
            c.execute(
                "INSERT INTO agents VALUES (?1,'roleA::caps::scope::body12',?2,'functional',0)",
                rusqlite::params![format!("a{i}"), haiku],
            ).unwrap();
        }
        for i in 0..5 {
            c.execute(
                "INSERT INTO agents VALUES (?1,'roleB::caps::scope::body12',?2,'partial',0)",
                rusqlite::params![format!("b{i}"), haiku],
            ).unwrap();
        }
        let r = calibrate(&c).unwrap();
        assert_eq!(r.rows_evaluated, 20);
        assert!(r.best_mse <= r.baseline_mse + 1e-9);
    }
}
