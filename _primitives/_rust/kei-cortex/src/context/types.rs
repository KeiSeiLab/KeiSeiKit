//! Public types shared across the `context` submodules.
//!
//! Constructor Pattern: types-only file, no behaviour. Behaviour lives in
//! `discover.rs`, `skill_loader.rs`, `inject.rs`. Anything that holds state
//! crossing module boundaries is declared here so call-sites stay flat.

use std::path::PathBuf;

/// Which class of context file this is. Drives both rendering order in
/// `inject::build_system_prompt` and de-duplication when CLAUDE.md and
/// AGENTS.md happen to point at the same on-disk file via symlink.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextKind {
    /// `CLAUDE.md` — project / repo / parent context for Claude Code.
    ClaudeMd,
    /// `AGENTS.md` — cross-tool agent context (cursor, codex, generic).
    AgentsMd,
    /// `SOUL.md` — KeiSei-specific persona overlay at project level.
    SoulMd,
    /// Any other `.md` discovered alongside (reserved for future use).
    OtherMd,
}

/// One context file discovered during walk-up. `path` is absolute,
/// `content` is the file body (truncated to 1 MB; truncation is signalled
/// by an inline `[truncated]` marker, not by a flag).
#[derive(Debug, Clone)]
pub struct DiscoveredFile {
    pub path: PathBuf,
    pub content: String,
    pub kind: ContextKind,
}

/// A `/skill-name`-matched skill body, with its source path for
/// observability and the matched name for error messages.
#[derive(Debug, Clone)]
pub struct LoadedSkill {
    pub name: String,
    pub path: PathBuf,
    pub body: String,
}
