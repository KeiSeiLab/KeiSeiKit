//! Typed error — every kei-fork public op returns `Result<_, Error>`.
//!
//! Categories:
//!   - `Validate` — agent-id failed `kei_agent_runtime::validate`
//!   - `Duplicate` — worktree/branch for this agent-id already exists
//!   - `NotDone` — collect() called before the agent wrote `.DONE`
//!   - `Gone` — rescue() could not find the worktree (live or archived)
//!   - `InvalidRef` — branch / base-branch string rejected by refname guard
//!   - `Io` / `Git` / `Ledger` / `Meta` — subsystem failures

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid agent-id: {0}")]
    Validate(String),

    #[error("fork already exists for agent-id '{0}'")]
    Duplicate(String),

    #[error(".DONE marker missing for agent-id '{0}' (agent not finished)")]
    NotDone(String),

    #[error("no live or archived worktree found for agent-id '{0}'")]
    Gone(String),

    #[error("invalid ref name '{0}' (arg-injection guard)")]
    InvalidRef(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("git command failed ({cmd}): {stderr}")]
    Git { cmd: String, stderr: String },

    #[error("ledger command failed: {0}")]
    Ledger(String),

    #[error("meta file malformed: {0}")]
    Meta(String),
}
