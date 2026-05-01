//! Tool registry + agentic loop substrate.
//!
//! Constructor Pattern: each tool is one cube (`read.rs`, `write.rs`,
//! `edit.rs`, `bash.rs`, `glob_tool.rs`, `grep.rs`, `webfetch.rs`,
//! `agent.rs`); the `registry` cube wires them by name; `schemas.rs`
//! holds the Anthropic JSON-Schema definitions; `loop_driver` runs the
//! turn-by-turn loop.
//!
//! Wave 44a security cubes:
//!   - `path_sandbox.rs` — chroot + basename + dotfile deny
//!   - `atomic_io.rs`    — shared atomic_write (used by write/edit)
//!   - `ip_filter.rs`    — SSRF deny-list (used by webfetch)
//!   - `bash_denylist.rs` — argv0 + substring deny + allow-list
//!
//! Public API: a daemon-side caller composes
//! `ToolRegistry::with_project_root(...)` + `tool_definitions()` and
//! hands them to `loop_driver::run_with_tools` along with a
//! `ModelInvoker` closure. See `INTEGRATION.md` for the exact patch the
//! orchestrator applies to `handlers/chat.rs`.

pub mod agent;
pub mod atomic_io;
pub mod bash;
mod bash_denylist;
mod dispatch;
pub mod edit;
pub mod glob_tool;
pub mod grep;
pub mod ip_filter;
pub mod loop_driver;
pub mod path_sandbox;
pub mod read;
pub mod registry;
pub mod schemas;
pub mod types;
pub mod webfetch;
pub mod write;

#[cfg(test)]
mod tests;

pub use loop_driver::{
    run_with_tools, ContentBlock, ConversationMessage, LoopEvent, ModelInvoker, ModelTurn,
    TokenUsage, MAX_TURNS,
};
pub use registry::{Executor, ToolRegistry};
pub use schemas::tool_definitions;
pub use types::{ToolCall, ToolError, ToolResult};
