//! Zed adapter — writes MCP/context-server entry into Zed settings.
//!
//! Config path [UNVERIFIED for exact schema key-name]:
//!   - macOS:   `~/Library/Application Support/Zed/settings.json`
//!   - Linux:   `~/.config/zed/settings.json`
//!   - Windows: not supported in this adapter (Zed Windows is preview)
//!
//! Schema (under a top-level `context_servers` object):
//! ```json
//! {
//!   "context_servers": {
//!     "keisei": {
//!       "command": "/path/to/kei-mcp-server",
//!       "args": [],
//!       "env": { "KEISEI_BRAIN_ROOT": "..." }
//!     }
//!   }
//! }
//! ```
//!
//! NOTE: Zed's `context_servers` key is the documented extension point for
//! MCP at time of writing — but the full schema (arg handling,
//! environment) is [UNVERIFIED] in this session. If a future Zed release
//! diverges, update this module.
//!
//! Security (v0.19 audit): collision-safe — if `context_servers["keisei"]`
//! already exists with different content, attach fails with
//! `NameConflict` rather than silently clobbering.
//!
//! v0.21.1: JSON merge/remove/persist logic lives in `jsonmcp` (shared
//! with Claude Code + Cursor).

use crate::adapter::ClientAdapter;
use crate::adapters::jsonmcp;
use crate::brain::Brain;
use crate::error::Result;
use crate::paths;
use crate::scope::Scope;
use std::path::PathBuf;

pub const ENTRY_KEY: &str = "keisei";
pub const CLIENT_NAME: &str = "zed";
const OUTER_KEY: &str = "context_servers";

pub struct ZedAdapter;

impl ZedAdapter {
    pub fn new() -> Self {
        Self
    }

    fn settings_dir(&self) -> PathBuf {
        let base = paths::resolve_home();
        if cfg!(target_os = "macos") {
            base.join("Library/Application Support/Zed")
        } else {
            base.join(".config/zed")
        }
    }

    fn settings_file(&self) -> PathBuf {
        self.settings_dir().join("settings.json")
    }
}

impl Default for ZedAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientAdapter for ZedAdapter {
    fn name(&self) -> &str {
        CLIENT_NAME
    }

    fn detect(&self) -> bool {
        self.settings_dir().is_dir()
    }

    fn supported_scopes(&self) -> &[Scope] {
        // Zed config is host-global — no per-project settings.json today.
        &[Scope::User]
    }

    fn attach(&self, brain: &Brain, _scope: Scope) -> Result<()> {
        let cfg = self.config_path(Scope::User);
        if let Some(parent) = cfg.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut doc = jsonmcp::load_json_or_empty(&cfg)?;
        let entry = jsonmcp::build_mcp_entry(brain)?;
        jsonmcp::upsert_under_key(&mut doc, OUTER_KEY, ENTRY_KEY, entry, CLIENT_NAME)?;
        jsonmcp::persist(&doc, &cfg)
    }

    fn detach(&self, _brain_name: &str, _scope: Scope) -> Result<()> {
        let cfg = self.config_path(Scope::User);
        if !cfg.is_file() {
            return Ok(());
        }
        let mut doc = jsonmcp::load_json_or_empty(&cfg)?;
        jsonmcp::remove_under_key(&mut doc, OUTER_KEY, ENTRY_KEY);
        jsonmcp::persist(&doc, &cfg)
    }

    fn config_path(&self, _scope: Scope) -> PathBuf {
        self.settings_file()
    }

    fn post_attach_hint(&self, brain: &Brain, _scope: Scope) -> String {
        format!(
            "run Zed ':reload' — '{}' registers under context_servers",
            brain.name()
        )
    }
}
