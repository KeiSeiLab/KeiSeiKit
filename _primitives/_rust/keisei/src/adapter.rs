//! Adapter trait + registry — the pluggable surface for AI clients.
//!
//! Constructor Pattern: this file owns the trait + the "enumerate all
//! adapters" function + lookup-by-name helper. Each concrete adapter
//! lives in its own file under `adapters/`. `Scope` itself lives in
//! `scope.rs` (own file, own concept). The adapter list lives in
//! `adapters/_registry.rs` — this file delegates via `all()` so the
//! public API stays stable when adapters are added.
//!
//! v0.21: trait gained `Scope` parameter — adapters with both host-wide
//! and per-project config surfaces (claude-code, cursor) can be driven
//! to either location from one code path. Adapters that only expose a
//! global config (continue, zed) declare `supported_scopes() = [User]`.
//!
//! v0.22:
//! * `auto_scope()` — adapter-driven CWD heuristic that turns
//!   `Scope::Auto` into a concrete `User` / `Project`. Default is
//!   `Scope::User` (safe fallback); Claude Code + Cursor override.
//! * `post_attach_hint(brain, scope)` — templated hint so the CLI can
//!   interpolate the brain's name and the resolved scope into the
//!   client-specific reload instruction. Returns `String` (not
//!   `&'static str`) to accommodate `format!(...)`.

use crate::brain::Brain;
use crate::error::{Error, Result};
use crate::scope::Scope;
use std::path::PathBuf;

pub trait ClientAdapter {
    fn name(&self) -> &str;
    fn detect(&self) -> bool;

    /// Which scopes this adapter can write into.
    /// Default: user-only — the safe conservative choice for adapters that
    /// haven't explicitly opted into project-local configs.
    fn supported_scopes(&self) -> &[Scope] {
        &[Scope::User]
    }

    /// Resolve `Scope::Auto` into a concrete scope via adapter-specific CWD
    /// heuristics. Default: `Scope::User` — adapters that understand a
    /// project-local config surface (claude-code, cursor) override this to
    /// return `Scope::Project` when the CWD has the matching dot-dir.
    ///
    /// Only called by the attach flow when the user passed `Scope::Auto`;
    /// the resolved value is what lands in the marker.
    fn auto_scope(&self) -> Scope {
        Scope::User
    }

    fn config_path(&self, scope: Scope) -> PathBuf;
    fn attach(&self, brain: &Brain, scope: Scope) -> Result<()>;
    fn detach(&self, brain_name: &str, scope: Scope) -> Result<()>;

    /// One-line instruction the CLI prints after a successful attach so
    /// the user knows how to make the client see the new MCP server.
    /// Adapters override this with a client-specific phrasing (reload
    /// command, command palette entry, etc). Default is a generic
    /// fallback that keeps the orchestrator free of client-specific
    /// strings. Takes `&Brain` and `Scope` so adapters can interpolate
    /// the brain name and the resolved scope into the message.
    fn post_attach_hint(&self, _brain: &Brain, _scope: Scope) -> String {
        "reload your AI client to pick up the new MCP server".to_string()
    }

    /// Helper: does this adapter support the given scope?
    /// `Scope::Auto` always "supported" here — scope resolution happens
    /// in `attach.rs` before this check runs against a concrete scope.
    fn supports_scope(&self, scope: Scope) -> bool {
        if matches!(scope, Scope::Auto) {
            return true;
        }
        self.supported_scopes().contains(&scope)
    }
}

/// Enumerate all adapters the binary knows about, in priority order.
/// Thin delegate to `adapters::_registry::all_adapters`, which is the
/// canonical list (Constructor Pattern single-point-of-edit).
pub fn all() -> Vec<Box<dyn ClientAdapter>> {
    crate::adapters::_registry::all_adapters()
}

/// Return the first adapter whose `detect()` fires. `NoClientDetected`
/// otherwise.
pub fn detect_active() -> Result<Box<dyn ClientAdapter>> {
    for a in all() {
        if a.detect() {
            return Ok(a);
        }
    }
    Err(Error::NoClientDetected)
}

/// Look up an adapter by its `name()`. Used by the detach flow which
/// iterates client names from the saved marker.
pub fn by_name(name: &str) -> Option<Box<dyn ClientAdapter>> {
    all().into_iter().find(|a| a.name() == name)
}
