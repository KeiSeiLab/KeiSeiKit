//! Empirical-posterior argmin-cost selector.
//!
//! Entry point: `select(input, conn) -> SqlResult<Decision>`.
//! Reads the ledger, applies kernel smoothing for unseen task-classes,
//! then picks the cheapest model whose quality lower-bound exceeds the threshold.
//!
//! Constructor Pattern: separated from `select.rs` (pick + types) to keep
//! both cubes under 200 LOC.

use crate::complexity::{self, ComplexityEstimate};
use crate::dna_class;
use crate::posterior::Posterior;
use crate::pricing::{self, Model};
use crate::select::{Decision, DecisionInput};
use crate::select_kernel;
use rusqlite::{Connection, Result as SqlResult};

pub fn select(input: &DecisionInput, conn: &Connection) -> SqlResult<Decision> {
    let role = dna_class::role(&input.full_dna);
    let complexity = complexity::estimate(&input.prompt, role);

    if let Some(m) = input.pinned {
        return Ok(pinned_decision(input, complexity, m));
    }

    let task_class = match dna_class::task_class_dna(&input.full_dna) {
        Some(t) => t.to_string(),
        None => return Ok(fallback_decision(input, complexity, "empty_dna")),
    };

    let feasible = collect_feasible(conn, input, &task_class)?;
    if feasible.is_empty() {
        return Ok(fallback_decision(input, complexity, "no_feasible"));
    }

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

fn collect_feasible(
    conn: &Connection,
    input: &DecisionInput,
    task_class: &str,
) -> SqlResult<Vec<(Model, Posterior, f64, u64)>> {
    let mut feasible: Vec<(Model, Posterior, f64, u64)> = Vec::new();
    for m in Model::all() {
        let post = posterior_for(conn, task_class, m, input)?;
        let lb = post.quality_lower_bound(input.delta);
        if lb >= input.q_threshold {
            feasible.push((m, post, lb, estimated_cost(input, m)));
        }
    }
    feasible.sort_by_key(|(_, _, _, c)| *c);
    Ok(feasible)
}

fn posterior_for(
    conn: &Connection,
    task_class: &str,
    m: Model,
    input: &DecisionInput,
) -> SqlResult<Posterior> {
    let post = Posterior::from_ledger(conn, task_class, m)?;
    if post.n == 0 {
        select_kernel::smooth(conn, task_class, m, input.kernel_weights)
    } else {
        Ok(post)
    }
}

/// Finding 3: use registry-backed pricing when available; fallback table
/// for legacy call paths where no registry is threaded in.
fn estimated_cost(input: &DecisionInput, m: Model) -> u64 {
    let t_in = input.tokens_in.unwrap_or(DecisionInput::DEFAULT_TOKENS_IN);
    let t_out = input.tokens_out.unwrap_or(DecisionInput::DEFAULT_TOKENS_OUT);
    if let Some(reg) = &input.registry {
        if let Some(cost) = pricing::cost_micro_cents(m.slug(), t_in, t_out, reg) {
            return cost;
        }
        eprintln!("[kei-model-router] [FALLBACK: registry missing] model {} not found; using hardcoded table", m.slug());
    }
    // Hardcoded fallback — mirrors models.toml exactly (verified 2026-04-30).
    let (in_micro, out_micro): (u64, u64) = match m {
        Model::Haiku45 => (100_000_000, 500_000_000),
        Model::Sonnet46 => (300_000_000, 1_500_000_000),
        Model::Opus47 => (500_000_000, 2_500_000_000),
    };
    t_in.saturating_mul(in_micro) / 1_000_000
        + t_out.saturating_mul(out_micro) / 1_000_000
}

fn pinned_decision(input: &DecisionInput, complexity: ComplexityEstimate, m: Model) -> Decision {
    Decision {
        model: m,
        expected_cost_micro_cents: estimated_cost(input, m),
        quality_lower_bound: 1.0,
        posterior_n: 0,
        complexity,
        reason: "pinned",
    }
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

// ──────────────────────────────────────────────────────────────────────────────
// Tests
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pricing::Model;
    use crate::select::DecisionInput;
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
    fn no_data_falls_back_to_top_tier() {
        let c = fresh_db();
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
        let c = fresh_db();
        let mut inp = DecisionInput::new("any::dna::1234::5678-90ab", "anything");
        inp.pinned = Some(Model::Haiku45);
        let d = select(&inp, &c).unwrap();
        assert_eq!(d.model, Model::Haiku45);
        assert_eq!(d.reason, "pinned");
    }

    #[test]
    fn many_haiku_successes_route_to_haiku() {
        let c = fresh_db();
        for i in 0..30 {
            c.execute(
                "INSERT INTO agents VALUES (?1,'tc1','claude-haiku-4-5','functional',0)",
                rusqlite::params![format!("a{i}")],
            )
            .unwrap();
        }
        let mut inp = DecisionInput::new("tc1-deadbeef", "do the thing");
        inp.full_dna = "tc1-deadbeef".to_string();
        let d = select(&inp, &c).unwrap();
        assert_eq!(d.model, Model::Haiku45);
        assert!(d.quality_lower_bound > 0.70);
    }
}
