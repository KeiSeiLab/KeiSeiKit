//! `context` — auto-discover CLAUDE.md / AGENTS.md / SOUL.md context
//! files by walking up from the chat process's cwd, optionally match a
//! `/skill-name` command at the start of the user message, and inject all
//! of it ahead of the persona prompt before the upstream call.
//!
//! Public surface:
//!   - [`discover`] — walk up, return nearest-first.
//!   - [`match_skill_command`] — pull leading `/<name>` from a user message.
//!   - [`build_system_prompt`] — concat persona + discovered + skill, capped.
//!
//! See `INTEGRATION.md` for the orchestrator-side patch in `chat.rs`.

pub mod discover;
pub mod inject;
pub mod skill_loader;
pub mod types;

pub use discover::discover;
pub use inject::{build_system_prompt, MAX_TOTAL_BYTES};
pub use skill_loader::match_skill_command;
pub use types::{ContextKind, DiscoveredFile, LoadedSkill};

#[cfg(test)]
mod tests {
    #[path = "discover_walks_up.rs"]
    mod discover_walks_up;

    #[path = "skill_command_match.rs"]
    mod skill_command_match;

    #[path = "inject_caps_at_50kb.rs"]
    mod inject_caps_at_50kb;

    #[path = "inject_order.rs"]
    mod inject_order;
}
