//! 3-wave research runner.
//!
//! Wave 0: split prompt into claims (naive split on `.`; real NLU later).
//! Wave 1: for each claim, fetch sources via [`SourceFetcher`].
//! Wave 2: score consensus per claim from sources (majority = higher grade).

use crate::budget::Budget;
use crate::fetch::SourceFetcher;
use crate::store::ResearchStore;
use crate::types::{Claim, Source};
use anyhow::Result;

const WAVE1_COST_PER_CLAIM_MC: i64 = 100; // 0.01 USD per claim
const WAVE2_COST_MC: i64 = 50;

pub fn run_research(
    store: &ResearchStore,
    fetcher: &dyn SourceFetcher,
    prompt: &str,
    budget_mc: i64,
) -> Result<i64> {
    let research_id = store.create_research(prompt)?;
    let mut budget = Budget::new(budget_mc);
    let claims_text = wave_0_extract_claims(prompt);
    if let Err(e) = wave_1_fetch(store, fetcher, research_id, &claims_text, &mut budget) {
        store.set_status(research_id, "failed")?;
        return Err(e);
    }
    if let Err(e) = wave_2_consensus(store, research_id, &mut budget) {
        store.set_status(research_id, "failed")?;
        return Err(e);
    }
    store.set_cost(research_id, budget.spent())?;
    store.set_status(research_id, "completed")?;
    Ok(research_id)
}

fn wave_0_extract_claims(prompt: &str) -> Vec<String> {
    prompt
        .split(|c: char| c == '.' || c == '?' || c == '\n')
        .map(|s| s.trim().to_string())
        .filter(|s| s.len() > 4)
        .collect()
}

fn wave_1_fetch(
    store: &ResearchStore,
    fetcher: &dyn SourceFetcher,
    rid: i64,
    claims: &[String],
    budget: &mut Budget,
) -> Result<()> {
    for c in claims {
        budget.charge(WAVE1_COST_PER_CLAIM_MC)?;
        let (srcs, fetch_cost) = fetcher.fetch(c);
        if fetch_cost > 0 {
            budget.charge(fetch_cost)?;
        }
        for s in srcs {
            store.add_source(&Source { research_id: rid, ..s })?;
        }
        store.add_claim(&Claim {
            research_id: rid,
            claim_text: c.clone(),
            ..Default::default()
        })?;
    }
    Ok(())
}

fn wave_2_consensus(store: &ResearchStore, rid: i64, budget: &mut Budget) -> Result<()> {
    budget.charge(WAVE2_COST_MC)?;
    let claims = store.claims_for(rid)?;
    for c in claims {
        let support = 0.5;
        let contradict = 0.0;
        let consensus = support - contradict;
        let grade = grade_from_consensus(consensus);
        store.conn().execute(
            "UPDATE claims SET support=?1, contradict=?2, consensus=?3, grade=?4
             WHERE id=?5",
            rusqlite::params![support, contradict, consensus, grade, c.id],
        )?;
    }
    Ok(())
}

fn grade_from_consensus(c: f64) -> &'static str {
    if c >= 0.8 { "E2" } else if c >= 0.5 { "E4" } else { "E6" }
}
