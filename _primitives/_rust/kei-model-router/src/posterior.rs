//! Beta posterior over per-(task-class, model) success rate.
//!
//! For each (task_class_dna, model) pair in the ledger we count:
//!   n+ = rows with outcome='functional' AND escalation_depth=0
//!   n- = rows with anything else
//!
//! Model identity is keyed by `Model::slug()` — the canonical model id
//! string (e.g. `claude-sonnet-4-6`) stored in `agents.model`.
//!
//! Constructor Pattern: SQL is one query, math is pure-fn.

use crate::pricing::Model;
use rusqlite::{params, Connection, OptionalExtension, Result as SqlResult};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Posterior {
    pub alpha: f64,
    pub beta: f64,
    pub n: u32,
}

impl Posterior {
    pub const PRIOR: Posterior = Posterior { alpha: 1.0, beta: 1.0, n: 0 };

    /// Posterior mean q̄ = α / (α + β).
    pub fn mean(&self) -> f64 {
        self.alpha / (self.alpha + self.beta)
    }

    /// Variance Var[q] = αβ / ((α+β)² (α+β+1))
    pub fn variance(&self) -> f64 {
        let s = self.alpha + self.beta;
        (self.alpha * self.beta) / (s * s * (s + 1.0))
    }

    /// Wilson-style normal-approx lower confidence bound.
    pub fn quality_lower_bound(&self, delta: f64) -> f64 {
        let z = z_one_sided(delta);
        let lb = self.mean() - z * self.variance().sqrt();
        lb.clamp(0.0, 1.0)
    }

    /// Bayesian update with new observation.
    pub fn observe(self, success: bool) -> Self {
        if success {
            Self { alpha: self.alpha + 1.0, beta: self.beta, n: self.n + 1 }
        } else {
            Self { alpha: self.alpha, beta: self.beta + 1.0, n: self.n + 1 }
        }
    }

    /// Build posterior from ledger rows for (task_class_dna, model).
    pub fn from_ledger(
        conn: &Connection,
        task_class: &str,
        model: Model,
    ) -> SqlResult<Self> {
        let row: Option<(i64, i64)> = conn
            .query_row(
                "SELECT
                    SUM(CASE WHEN outcome = 'functional'
                              AND COALESCE(escalation_depth, 0) = 0
                             THEN 1 ELSE 0 END) AS n_plus,
                    SUM(CASE WHEN outcome IS NOT NULL
                              AND NOT (outcome = 'functional'
                                       AND COALESCE(escalation_depth, 0) = 0)
                             THEN 1 ELSE 0 END) AS n_minus
                 FROM agents
                 WHERE task_class_dna = ?1 AND model = ?2",
                params![task_class, model.slug()],
                |r| Ok((
                    r.get::<_, Option<i64>>(0)?.unwrap_or(0),
                    r.get::<_, Option<i64>>(1)?.unwrap_or(0),
                )),
            )
            .optional()?;
        let (n_plus, n_minus) = row.unwrap_or((0, 0));
        Ok(Posterior {
            alpha: 1.0 + n_plus as f64,
            beta: 1.0 + n_minus as f64,
            n: (n_plus + n_minus) as u32,
        })
    }
}

fn z_one_sided(delta: f64) -> f64 {
    match delta {
        d if d <= 0.01 => 2.326,
        d if d <= 0.05 => 1.645,
        d if d <= 0.10 => 1.282,
        d if d <= 0.20 => 0.842,
        _ => 0.0,
    }
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
    fn prior_mean_is_one_half() {
        let p = Posterior::PRIOR;
        assert!((p.mean() - 0.5).abs() < 1e-9);
    }

    #[test]
    fn observe_success_shifts_mean_up() {
        let p = Posterior::PRIOR.observe(true).observe(true).observe(true);
        assert!(p.mean() > 0.5);
        assert_eq!(p.n, 3);
    }

    #[test]
    fn observe_failure_shifts_mean_down() {
        let p = Posterior::PRIOR.observe(false).observe(false);
        assert!(p.mean() < 0.5);
    }

    #[test]
    fn ledger_no_rows_returns_uniform_prior() {
        let c = fresh_db();
        let p = Posterior::from_ledger(&c, "missing", Model::Haiku45).unwrap();
        assert_eq!(p, Posterior::PRIOR);
    }

    #[test]
    fn ledger_aggregates_by_model_slug() {
        let c = fresh_db();
        // Use canonical model ids (matching Model::slug())
        let haiku = Model::Haiku45.slug();
        let opus = Model::Opus47.slug();
        c.execute(
            "INSERT INTO agents VALUES ('1','tc1',?1,'functional',0)",
            rusqlite::params![haiku],
        ).unwrap();
        c.execute(
            "INSERT INTO agents VALUES ('2','tc1',?1,'functional',0)",
            rusqlite::params![haiku],
        ).unwrap();
        c.execute(
            "INSERT INTO agents VALUES ('3','tc1',?1,'partial',0)",
            rusqlite::params![haiku],
        ).unwrap();
        c.execute(
            "INSERT INTO agents VALUES ('4','tc1',?1,'functional',0)",
            rusqlite::params![opus],
        ).unwrap();
        let h = Posterior::from_ledger(&c, "tc1", Model::Haiku45).unwrap();
        assert_eq!(h.n, 3);
        assert!((h.mean() - 0.6).abs() < 1e-9);
        let o = Posterior::from_ledger(&c, "tc1", Model::Opus47).unwrap();
        assert_eq!(o.n, 1);
    }

    #[test]
    fn escalated_success_counts_as_failure_for_first_pass() {
        let c = fresh_db();
        let slug = Model::Haiku45.slug();
        c.execute(
            "INSERT INTO agents VALUES ('1','tc',?1,'functional',1)",
            rusqlite::params![slug],
        ).unwrap();
        let p = Posterior::from_ledger(&c, "tc", Model::Haiku45).unwrap();
        assert_eq!(p.alpha, 1.0);
        assert_eq!(p.beta, 2.0);
    }

    #[test]
    fn lower_bound_at_high_n_concentrates_near_mean() {
        let mut p = Posterior::PRIOR;
        for _ in 0..100 {
            p = p.observe(true);
        }
        let lb = p.quality_lower_bound(0.10);
        assert!(lb > 0.95, "lb={}", lb);
    }

    #[test]
    fn lower_bound_with_no_data_is_conservative() {
        let p = Posterior::PRIOR;
        let lb = p.quality_lower_bound(0.10);
        assert!(lb < 0.30);
    }
}
