//! Claude Code adapter — writes MCP server entry into
//! `~/.claude/settings.json` (user scope) or `./.claude/settings.json`
//! (project scope). Config shape merges under `mcpServers.keisei` so we
//! never clobber unrelated entries.
//!
//! Detection: `$CWD/.claude/settings.json` exists OR
//! `$KEISEI_HOME/.claude` (or `$HOME/.claude`) is a directory.
//! `$KEISEI_HOME` overrides `$HOME` for tests.
//!
//! Security (v0.19 audit): if an entry at `mcpServers["keisei"]` already
//! exists and doesn't match what keisei would write, attach fails with
//! `NameConflict` instead of silently clobbering the user's config.
//!
//! v0.21.1: the JSON merge/remove/persist logic lives in `jsonmcp`
//! (shared with Cursor + Zed); this file is now just the client-specific
//! path resolution + scope table.

use crate::adapter::ClientAdapter;
use crate::adapters::jsonmcp;
use crate::brain::Brain;
use crate::error::Result;
use crate::paths;
use crate::scope::Scope;
use std::path::PathBuf;

pub const MCP_ENTRY_KEY: &str = "keisei";
pub const CLIENT_NAME: &str = "claude-code";
const OUTER_KEY: &str = "mcpServers";

pub struct ClaudeCodeAdapter;

impl ClaudeCodeAdapter {
    pub fn new() -> Self {
        Self
    }

    fn user_config_dir(&self) -> PathBuf {
        paths::resolve_home().join(".claude")
    }

    fn project_config_dir(&self) -> Option<PathBuf> {
        std::env::current_dir().ok().map(|p| p.join(".claude"))
    }

    fn dir_for_scope(&self, scope: Scope) -> PathBuf {
        match scope {
            Scope::User => self.user_config_dir(),
            Scope::Project => self
                .project_config_dir()
                .unwrap_or_else(|| PathBuf::from(".claude")),
            // `Auto` is a CLI-level intent, resolved before any adapter
            // call. If one leaks here, treat it as the safe default.
            Scope::Auto => self.user_config_dir(),
        }
    }
}

impl Default for ClaudeCodeAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientAdapter for ClaudeCodeAdapter {
    fn name(&self) -> &str {
        CLIENT_NAME
    }

    fn detect(&self) -> bool {
        let cwd_local = std::env::current_dir()
            .map(|p| p.join(".claude/settings.json").is_file())
            .unwrap_or(false);
        cwd_local || self.user_config_dir().is_dir()
    }

    fn supported_scopes(&self) -> &[Scope] {
        &[Scope::User, Scope::Project]
    }

    fn auto_scope(&self) -> Scope {
        // Project-scope if the CWD carries a `.claude/` dir (either
        // `settings.json` already in place, OR an empty `.claude/`
        // directory the user has scaffolded). Otherwise user-scope.
        let Ok(cwd) = std::env::current_dir() else {
            return Scope::User;
        };
        let dot_claude = cwd.join(".claude");
        if dot_claude.join("settings.json").is_file() || dot_claude.is_dir() {
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
        self.dir_for_scope(scope).join("settings.json")
    }

    fn post_attach_hint(&self, brain: &Brain, scope: Scope) -> String {
        format!(
            "run /help in Claude Code ({} scope) — verify '{}' is in mcpServers",
            scope,
            brain.name()
        )
    }
}
