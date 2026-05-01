//! Agent-side helpers — periodic memory review, nudge scheduling, and
//! the prompt template used by the background reviewer.
//!
//! Constructor Pattern: each cube is a single-responsibility module.
//! Public surface is intentionally narrow — only the scheduler trigger
//! and the review-task entry point are reachable from outside.
//!
//! Frozen-snapshot invariant: nothing in this module mutates the
//! parent agent's in-flight system prompt. Background reviews write
//! exclusively to disk-backed memory stores. The next session picks
//! up the new snapshot via the normal load path; the running session
//! is left undisturbed (preserves Anthropic prefix-cache hits).

pub mod anthropic_memory_invoker;
pub mod memory_nudge;
pub mod memory_persist;
pub mod memory_review_prompt;
pub mod memory_review_task;
