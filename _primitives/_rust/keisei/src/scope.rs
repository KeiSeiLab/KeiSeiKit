//! `Scope` — whether an attach targets the host-wide (User) config or the
//! project-local (Project) config for an AI client.
//!
//! Constructor Pattern: single responsibility — a plain enum + its (de)serde
//! projection. No I/O, no adapter knowledge. Lives in its own file to keep
//! `adapter.rs` at one-concept (the trait itself).
//!
//! v0.22: added `Scope::Auto` as the CLI default so
//! `cd team-repo; keisei attach <brain>` detects project-scope
//! automatically (if `./.claude/` or `./.cursor/` exists) without the user
//! having to type `--scope=project`. `Auto` is a CLI-level intent, never
//! persisted — `attach.rs` resolves it to concrete `User` / `Project` via
//! `adapter.auto_scope()` before writing the marker.
//!
//! Default on deserialization is `Scope::User` so v0.20 markers (written
//! before this field existed) round-trip transparently.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Scope {
    /// Host-wide config — e.g. `~/.claude/settings.json`, `~/.cursor/mcp.json`.
    User,
    /// Project-local config — e.g. `./.claude/settings.json`, `./.cursor/mcp.json`.
    Project,
    /// Ask the adapter to pick based on CWD heuristics. CLI-only intent —
    /// never written to the marker file. Resolved to `User` or `Project`
    /// by `adapter.auto_scope()` before persistence.
    Auto,
}

impl Default for Scope {
    fn default() -> Self {
        Scope::User
    }
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Scope::User => f.write_str("user"),
            Scope::Project => f.write_str("project"),
            Scope::Auto => f.write_str("auto"),
        }
    }
}

impl std::str::FromStr for Scope {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "user" => Ok(Scope::User),
            "project" => Ok(Scope::Project),
            "auto" => Ok(Scope::Auto),
            other => Err(format!(
                "unknown scope '{other}' — expected 'user', 'project', or 'auto'"
            )),
        }
    }
}
