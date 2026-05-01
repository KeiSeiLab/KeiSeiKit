//! kei-spawn — automation envelope around kei-agent-runtime + kei-ledger.
//!
//! Orchestrator flow pre-kei-spawn:
//!   1. Write task.toml manually
//!   2. Run `kei-agent-runtime prepare`
//!   3. Invoke Agent tool (harness-internal, orchestrator-only)
//!   4. Run `kei-ledger fork`
//!   5. On return, run `kei-agent-runtime verify`
//!
//! With kei-spawn, steps 2 + 4 collapse to one `kei-spawn spawn <task.toml>` call
//! and step 5 collapses to one `kei-spawn verify <agent-id> <worktree>` call.
//! Step 3 (the actual Agent tool invocation) STILL belongs to the orchestrator
//! because Claude Code's `Agent` tool is harness-internal — it can't be invoked
//! from Rust. `kei-spawn` emits a JSON bundle the orchestrator pastes.
//!
//! Design constraints:
//!   - Constructor Pattern: one module = one responsibility, ≤200 LOC file,
//!     ≤30 LOC fn.
//!   - Optional HTTP via the `http-driver` Cargo feature (reqwest::blocking +
//!     rustls). Off by default — v0.1 ships `ManualDriver` only.
//!   - No git / no shell — ledger interactions go through `kei-ledger` as a
//!     subprocess to avoid adding kei-ledger as a direct dep while it still
//!     lacks a lib.rs (can't link to a bin-only crate).
//!
//! Per RULE 0.13: kei-spawn NEVER creates branches or commits. The orchestrator
//! owns git state. kei-spawn only writes into `tasks/<agent-id>/` and invokes
//! `kei-ledger` (which itself only writes to SQLite).

pub mod drive;
#[cfg(feature = "http-driver")]
pub mod drive_http;
#[cfg(feature = "http-driver")]
pub mod drive_http_parse;
pub mod ledger_sh;
pub mod pipeline;
pub mod precedent;
pub mod spawn;
pub mod verify;

pub use drive::{
    drive_with, not_implemented_message, AgentResult, AnthropicDriver, DriveError, HttpDriver,
    ManualDriver,
};
pub use pipeline::{
    derive_chain_from_role, derive_steps, emit_pipeline_json, pipeline_from_role,
    pipeline_json_path, scaffold_downstream_tasks, PipelineChain, PipelineStep,
};
pub use spawn::{spawn_from_task, spawn_with_pipeline, SpawnOutput};
pub use verify::{verify_agent, VerifyOutput};
