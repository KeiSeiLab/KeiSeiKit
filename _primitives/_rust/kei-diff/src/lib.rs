//! kei-diff — structural JSON diff (RFC 6902 subset: add/remove/replace).
//!
//! ## Design
//! * Emits ONLY `add`, `remove`, `replace`. No `copy`/`move`/`test`.
//! * Arrays diffed by index (not LCS) — matches drift-detection semantics.
//! * Paths are RFC 6901 JSON Pointers (`~` → `~0`, `/` → `~1`).
//! * Correctness invariant: `apply(old, diff(old, new)) == new`.
//!
//! Consumed by `kei-replay` (drift detection between DNA-scoped agent runs)
//! and `kei-cache` (invalidation signals). Pure compute, zero sibling deps.

mod apply;
mod apply_error;
mod diff;
mod op;
mod path;

pub use apply::apply;
pub use apply_error::ApplyError;
pub use diff::diff;
pub use op::{Op, Patch};
pub use path::PathBuf as PointerBuf;
