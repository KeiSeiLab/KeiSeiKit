//! Cursor adapter — writes MCP server entry to Cursor's MCP config.
//!
//! Scope:
//!   - `Scope::User`    → `~/.cursor/mcp.json`
//!   - `Scope::Project` → `$CWD/.cursor/mcp.json`
//!
//! Detection fires if either the user-scope dir or a project-scope dir
//! exists. Schema [UNVERIFIED — matches Claude Desktop MCP convention]:
//! `{ "mcpServers": { "keisei": { "command": "...", "args": [] } } }`.
//!
//! Security (v0.19 audit): collision-safe — if `mcpServers["keisei"]`
//! already exists with different content, attach fails with
//! `NameConflict` rather than silently clobbering.
//!
//! v0.21.1: JSON merge/remove/persist logic lives in `jsonmcp` (shared
//! with Claude Code + Zed).

use crate::adapter::ClientAdapter;
use crate::adapters::jsonmcp;
use crate::brain::Brain;
use crate::error::Result;
use crate::paths;
use crate::scope::Scope;
use std::path::PathBuf;

pub const MCP_ENTRY_KEY: &str = "keisei";
pub const CLIENT_NAME: &str = "cursor";
const OUTER_KEY: &str = "mcpServers";

pub struct CursorAdapter;

impl CursorAdapter {
    pub fn new() -> Self {
        Self
    }

    fn home_cursor_dir(&self) -> PathBuf {
        paths::resolve_home().join(".cursor")
    }

    fn project_cursor_dir(&self) -> Option<PathBuf> {
        std::env::current_dir().ok().map(|p| p.join(".cursor"))
    }

    fn dir_for_scope(&self, scope: Scope) -> PathBuf {
        match scope {
            Scope::User => self.home_cursor_dir(),
            Scope::Project => self
                .project_cursor_dir()
                .unwrap_or_else(|| PathBuf::from(".cursor")),
            // `Auto` is a CLI-level intent, resolved before any adapter
            // call. Safe fallback is user scope.
            Scope::Auto => self.home_cursor_dir(),
        }
    }
}

impl Default for CursorAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientAdapter for CursorAdapter {
    fn name(&self) -> &str {
        CLIENT_NAME
    }

    fn detect(&self) -> bool {
        let proj = self
            .project_cursor_dir()
            .map(|p| p.is_dir())
            .unwrap_or(false);
        proj || self.home_cursor_dir().is_dir()
    }

    fn supported_scopes(&self) -> &[Scope] {
        &[Scope::User, Scope::Project]
    }

    fn auto_scope(&self) -> Scope {
        // Project-scope if `./.cursor/` exists in the CWD. Mirrors the
        // claude-code heuristic shape.
        let Ok(cwd) = std::env::current_dir() else {
            return Scope::User;
        };
        if cwd.join(".cursor").is_dir() {
            Scope::Project
        } else {
            Scope::User
        }
    }

    fn attach(&self, brain: &Brain, scope: Scope) -> Result<()> {
        let cfg = self.config_path(scope);
        if let Some(parent) = cfg.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut doc = jsonmcp::load_json_or_empty(&cfg)?;
        let entry = jsonmcp::build_mcp_entry(brain)?;
        jsonmcp::upsert_under_key(&mut doc, OUTER_KEY, MCP_ENTRY_KEY, entry, CLIENT_NAME)?;
        jsonmcp::persist(&doc, &cfg)
    }

    fn detach(&self, _brain_name: &str, scope: Scope) -> Result<()> {
        let cfg = self.config_path(scope);
        if !cfg.is_file() {
            return Ok(());
        }
        let mut doc = jsonmcp::load_json_or_empty(&cfg)?;
        jsonmcp::remove_under_key(&mut doc, OUTER_KEY, MCP_ENTRY_KEY);
        jsonmcp::persist(&doc, &cfg)
    }

    fn config_path(&self, scope: Scope) -> PathBuf {
        self.dir_for_scope(scope).join("mcp.json")
    }

    fn post_attach_hint(&self, brain: &Brain, _scope: Scope) -> String {
        format!(
            "reload Cursor window (Cmd+Shift+P → Reload Window) — '{}' should appear in MCP servers list",
            brain.name()
        )
    }
}
