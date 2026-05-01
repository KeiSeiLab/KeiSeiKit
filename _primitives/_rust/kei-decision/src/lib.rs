//! kei-decision — research output → action pipeline.
//!
//! Reads `MASTER-REPORT.md` from `/research` Variant C output, extracts
//! the "Actionable plan" table, classifies each row into an [`ActionKind`],
//! topo-sorts by deps + scores, emits one `task.toml` per action in a form
//! `kei-spawn` can consume directly. Optionally chains to `kei-spawn spawn`
//! and `kei-ledger fork` so each action is queued for execution before the
//! orchestrator returns from the `/research` skill.
//!
//! Constructor Pattern: each module owns one responsibility, ≤ 200 LOC,
//! ≤ 30 LOC per fn. No async, no network, no md crate (regex-only).

pub mod cli;
pub mod classifier;
pub mod emitter;
pub mod executor;
pub mod graph;
pub mod ledger;
pub mod parser;
pub mod ranker;
pub mod sleep_link;

pub use classifier::{classify, ActionKind};
pub use emitter::{emit_task_toml, EmitOutput};
pub use executor::{execute_action, ExecuteOutput};
pub use graph::{merge_graphs, GraphMergeOutput};
pub use ledger::{pre_fork_ledger, LedgerPreForkOutput};
pub use parser::{parse_master_report, ParseError, RawAction};
pub use ranker::{rank_actions, RankedAction};
pub use sleep_link::{scan_research_sources, SleepScanOutput};
