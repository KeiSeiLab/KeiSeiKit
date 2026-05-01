//! kei-refactor-engine — library surface.
//!
//! Consumes `kei-conflict-scan` JSON; produces a structured refactor plan
//! (markdown) and, optionally, an auto-resolve review markdown
//! (NOT a unified diff — see patch.rs header, v0.14.1 retraction).
//!
//! Zero-conflict guarantee: any conflict whose `auto_resolvable = false`
//! is included in the plan under "Requires human decision" and EXCLUDED
//! from the auto-resolve markdown.

pub mod input;
pub mod plan;
pub mod patch;
pub mod render;

pub use input::{read_conflicts, Conflict};
pub use plan::{Plan, PlanItem, Resolution};
