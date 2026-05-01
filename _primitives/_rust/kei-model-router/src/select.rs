//! Decision rule — the heart of the router.
//!
//! m*(d̂) = argmin_{m ∈ M} { c(d̂, m) | P[q(d̂, m) ≥ q*] ≥ 1 − δ }
//!
//! Implementation:
//! 1. Compute `task_class_dna` from full DNA.
//! 2. For each model m ∈ {Haiku, Sonnet, Opus}:
//!    a. Pull posterior from ledger for (task_class, m).
//!    b. If n=0 → optionally smooth via kernel from similar task_classes.
//!    c. Compute q_lower(δ).
//! 3. Filter to models where q_lower ≥ q*.
//! 4. Among feasible: pick cheapest (smallest expected cost).
//! 5. If feasible set empty → fallback.
//!
//! Per RULE -1: empty feasible set → return fallback (top tier), NOT an
//! error. Router never refuses; it surfaces uncertainty by selecting
//! safer model.
//!
//! Constructor Pattern: this is the orchestrating cube. SQL is delegated
//! to `posterior`, math to `pricing`, similarity to `kernel`.

use crate::complexity::{self, ComplexityEstimate};
use crate::dna_class;
use crate::kernel::{self, KernelWeights};
use crate::pricing::{cost_micro_cents, Model};
use crate::posterior::Posterior;
use rusqlite::{Connection, Result as SqlResult};

#[derive(Debug, Clone)]
pub struct DecisionInput {
    pub full_dna: String,
    pub prompt: String,
    pub q_threshold: f64,
    pub delta: f64,
    pub fallback: Model,
    /// Pinned override: if Some, skip routing and use this. For per-agent pins.
    pub pinned: Option<Model>,
    pub kernel_weights: KernelWeights,
    /// Estimated input/output token counts; if None, use defaults.
    pub tokens_in: Option<u64>,
    pub tokens_out: Option<u64>,
}

impl DecisionInput {
    /// Sensible defaults for a typical Agent spawn (~ 4k in, 1.5k out).
    pub const DEFAULT_TOKENS_IN: u64 = 4_000;
    pub const DEFAULT_TOKENS_OUT: u64 = 1_500;

    pub fn new(full_dna: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            full_dna: full_dna.into(),
            prompt: prompt.into(),
            q_threshold: 0.70,
            delta: 0.10,
            fallback: Model::Opus47,
            pinned: None,
            kernel_weights: KernelWeights::default(),
            tokens_in: None,
            tokens_out: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Decision {
    pub model: Model,
    pub expected_cost_micro_cents: u64,
    pub quality_lower_bound: f64,
    pub posterior_n: u32,
    pub complexity: ComplexityEstimate,
    pub reason: &'static str,
}

pub fn select(input: &DecisionInput, conn: &Connection) -> SqlResult<Decision> {
    let role = dna_class::role(&input.full_dna);
    let complexity = complexity::estimate(&input.prompt, role);

    if let Some(m) = input.pinned {
        return Ok(Decision {
            model: m,
            expected_cost_micro_cents: estimated_cost(input, m),
            quality_lower_bound: 1.0,
            posterior_n: 0,
            complexity,
            reason: "pinned",
        });
    }

    let task_class = match dna_class::task_class_dna(&input.full_dna) {
        Some(t) => t.to_string(),
        None => {
            return Ok(fallback_decision(input, complexity, "empty_dna"));
        }
    };

    let mut feasible: Vec<(Model, Posterior, f64, u64)> = Vec::new();
    for m in Model::all() {
        let mut post = Posterior::from_ledger(conn, &task_class, m)?;
        if post.n == 0 {
            post = smooth_via_kernel(conn, &task_class, m, input.kernel_weights)?;
        }
        let lb = post.quality_lower_bound(input.delta);
        if lb >= input.q_threshold {
            let cost = estimated_cost(input, m);
            feasible.push((m, post, lb, cost));
        }
    }

    if feasible.is_empty() {
        return Ok(fallback_decision(input, complexity, "no_feasible"));
    }

    // Cheapest feasible.
    feasible.sort_by_key(|(_, _, _, c)| *c);
    let (model, post, lb, cost) = feasible[0];
    Ok(Decision {
        model,
        expected_cost_micro_cents: cost,
        quality_lower_bound: lb,
        posterior_n: post.n,
        complexity,
        reason: "argmin_cost_feasible",
    })
}

fn estimated_cost(input: &DecisionInput, m: Model) -> u64 {
    let t_in = input.tokens_in.unwrap_or(DecisionInput::DEFAULT_TOKENS_IN);
    let t_out = input.tokens_out.unwrap_or(DecisionInput::DEFAULT_TOKENS_OUT);
    cost_micro_cents(m, t_in, t_out)
}

fn fallback_decision(
    input: &DecisionInput,
    complexity: ComplexityEstimate,
    reason: &'static str,
) -> Decision {
    Decision {
        model: input.fallback,
        expected_cost_micro_cents: estimated_cost(input, input.fallback),
        quality_lower_bound: 0.0,
        posterior_n: 0,
        complexity,
        reason,
    }
}

/// Pull all (task_class_dna, model) posteriors weighted by kernel(task_class, *).
/// O(rows) — for large ledgers add an index-only scan; for our scale (≤10k rows)
/// this is fine.
fn smooth_via_kernel(
    conn: &Connection,
    target_task_class: &str,
    model: Model,
    weights: KernelWeights,
) -> SqlResult<Posterior> {
    let mut stmt = conn.prepare(
        "SELECT task_class_dna,
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
         GROUP BY task_class_dna",
    )?;

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

    let mut weighted_alpha = 1.0_f64;
    let mut weighted_beta = 1.0_f64;
    let mut weighted_n = 0_u32;

    for row in rows {
        let (other_tc, np, nm) = row?;
        let sim = kernel::similarity(target_task_class, &other_tc, weights);
        if sim <= 0.0 {
            continue;
        }
        weighted_alpha += sim * np as f64;
        weighted_beta += sim * nm as f64;
        weighted_n = weighted_n.saturating_add((np + nm) as u32);
    }

    Ok(Posterior {
        alpha: weighted_alpha,
        beta: weighted_beta,
        n: weighted_n,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn fresh_db_with_schema() -> Connection {
        let c = Connection::open_in_memory().unwrap();
        c.execute_batch(
            "CREATE TABLE agents (
                id TEXT,
                task_class_dna TEXT,
                model TEXT,
                outcome TEXT,
                escalation_depth INTEGER DEFAULT 0
            );",
        )
        .unwrap();
        c
    }

    #[test]
    fn no_data_falls_back_to_top_tier() {
        let c = fresh_db_with_schema();
        let inp = DecisionInput::new(
            "Explore::?::abcd1234::deadbeef-cafef00d",
            "find files",
        );
        let d = select(&inp, &c).unwrap();
        assert_eq!(d.model, Model::Opus47);
        assert_eq!(d.reason, "no_feasible");
    }

    #[test]
    fn pinned_short_circuits() {
        let c = fresh_db_with_schema();
        let mut inp = DecisionInput::new("any::dna::1234::5678-90ab", "anything");
        inp.pinned = Some(Model::Haiku45);
        let d = select(&inp, &c).unwrap();
        assert_eq!(d.model, Model::Haiku45);
        assert_eq!(d.reason, "pinned");
    }

    #[test]
    fn many_haiku_successes_route_to_haiku() {
        let c = fresh_db_with_schema();
        // 30 successful Haiku runs on this task class
        for i in 0..30 {
            c.execute(
                "INSERT INTO agents VALUES (?1, 'tc1', 'haiku', 'functional', 0)",
                rusqlite::params![format!("a{i}")],
            )
            .unwrap();
        }
        let mut inp = DecisionInput::new(
            "tc1-a-b1234567",
            "do the thing",
        );
        // make full_dna's task_class_dna = "tc1"
        inp.full_dna = "tc1-deadbeef".to_string();
        let d = select(&inp, &c).unwrap();
        assert_eq!(d.model, Model::Haiku45);
        assert!(d.quality_lower_bound > 0.70);
    }

    #[test]
    fn cost_minimization_picks_cheapest_among_feasible() {
        let c = fresh_db_with_schema();
        // All three models have plenty of successes
        for m in &["haiku", "sonnet", "opus"] {
            for i in 0..30 {
                c.execute(
                    "INSERT INTO agents VALUES (?1, 'tc-shared', ?2, 'functional', 0)",
                    rusqlite::params![format!("{m}{i}"), m],
                )
                .unwrap();
            }
        }
        let mut inp = DecisionInput::new("tc-shared-deadbeef", "anything");
        inp.full_dna = "tc-shared-deadbeef".to_string();
        let d = select(&inp, &c).unwrap();
        assert_eq!(d.model, Model::Haiku45);
        assert_eq!(d.reason, "argmin_cost_feasible");
    }
}
