//! kei-fork — managed git-worktree + ledger lifecycle for agent spawns.
//!
//! Public API: `create`, `collect`, `list`, `gc`, `rescue`. Each op is
//! backed by one module under `src/`, keeping every file ≤200 LOC and
//! every function ≤30 LOC (Constructor Pattern). Shell-out helpers for
//! `git` live in `git.rs`; TOML round-trip for `.KEI_FORK_META.toml`
//! lives in `meta.rs`; the `ForkHandle` value type and the
//! `ForkStatus` enum live in `handle.rs`.
//!
//! Ledger integration is optional at runtime: if env
//! `KEI_FORK_SKIP_LEDGER=1` is set, create/collect/gc skip the
//! `kei-ledger` subprocess call. Tests rely on this for hermeticity.

pub mod collect;
pub mod create;
pub mod error;
pub mod gc;
pub mod git;
pub mod handle;
pub mod list;
pub mod meta;
pub mod rescue;

pub use collect::{collect, CollectReport};
pub use create::create;
pub use error::Error;
pub use gc::{gc, GcReport};
pub use handle::{ForkHandle, ForkStatus};
pub use list::list;
pub use rescue::rescue;
