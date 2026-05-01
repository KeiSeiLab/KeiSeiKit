//! kei-ledger — public library surface.
//!
//! Constructor Pattern: the binary (`main.rs`) and the library share the
//! same module tree via `mod` declarations here. External crates depend
//! on `kei_ledger::record_cost` directly without re-exposing the CLI's
//! clap-driven dispatch surface.
//!
//! Wave 40 (2026-04-24): added so `kei-cortex` can plumb cost recording
//! through `kei_ledger::record_cost(conn, id, cents, provider, model)`
//! after each chat turn. Prior to v6, the only consumer was the CLI
//! binary itself, so a `[lib]` target was unnecessary.

pub mod cost;
pub mod descendants;
pub mod error;
pub mod ledger;
pub mod migrations_list;
pub mod row;
pub mod schema;
pub mod skill_aggregator;
pub mod skill_aggregator_cli;
pub mod skill_metrics;

pub use cost::{
    compose_micro_cents, display_cents_from_micro, read_cost, read_cost_micro, record_cost,
    record_cost_micro, replace_cost, replace_cost_micro, MICRO_CENTS_PER_CENT,
};
pub use error::{LedgerError, MAX_TREE_DEPTH};
pub use ledger::{done, fail, fork, list, merged, open, tree, validate, AgentRow};
pub use row::SELECT_COLS;
pub use schema::{migrate, MAX_BRANCH_LEN, REQUIRED_ARTEFACTS, SCHEMA_VERSION};
pub use skill_aggregator::{aggregate_skills, SkillAggregate, SkillRecommendation};
pub use skill_metrics::{
    last_used, record_invocation, success_rate, unused_skills, usage_count, SkillInvocation,
};
