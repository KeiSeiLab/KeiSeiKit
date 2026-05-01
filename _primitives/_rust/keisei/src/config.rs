//! SSoT for the active attach: `~/.keisei/attached.toml` (v0.22+).
//!
//! Schema v4 (v0.22, multi-brain per marker):
//! ```toml
//! schema_version = 4
//!
//! [[attachments]]
//! brain_path  = "/Volumes/Brain1"
//! brain_name  = "brain-a"
//! client_type = "claude-code"
//! config_path = "/Users/me/.claude/settings.json"
//! scope       = "user"
//! attached_at = "2026-04-22T14:23:00Z"
//! ```
//!
//! Older schemas (v1/v2/v3) still read transparently — migrated in-memory
//! to v4 on first `read()` (see `config_migrate.rs`). One-line stderr
//! notice fires so operators see the shape flip. Location migration
//! (v0.20 legacy path `~/.claude/keisei-attached.toml` → v0.21 path
//! `~/.keisei/attached.toml`) happens in the same pass.
//!
//! Constructor Pattern: single responsibility — read/write the attach
//! marker + one-shot location migration. Schema migration lives in
//! `config_migrate.rs`; time helpers in `time.rs`.
//!
//! Testability: `$KEISEI_HOME` overrides `$HOME` so integration tests
//! isolate state per tmpdir.

use crate::config_migrate::WireRecord;
use crate::error::Result;
use crate::scope::Scope;
use crate::time;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// v0.21+ filename. The marker lives at `$KEISEI_HOME/.keisei/attached.toml`.
pub const ATTACHED_FILENAME: &str = "attached.toml";

/// Legacy (v0.20 and earlier) filename, under `$KEISEI_HOME/.claude/`.
pub const LEGACY_ATTACHED_FILENAME: &str = "keisei-attached.toml";

/// Current on-disk schema version.
pub const CURRENT_SCHEMA: u32 = 4;

/// A single brain ⇄ client attachment. v4 pulls `brain_path`, `brain_name`,
/// and `attached_at` INTO the attachment so one marker can track multiple
/// brains wired to different clients simultaneously.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Attachment {
    pub brain_path: String,
    pub brain_name: String,
    pub client_type: String,
    pub config_path: String,
    #[serde(default)]
    pub scope: Scope,
    pub attached_at: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct AttachRecord {
    pub schema_version: u32,
    pub attachments: Vec<Attachment>,
}

impl AttachRecord {
    pub fn new(attachments: Vec<Attachment>) -> Self {
        Self {
            schema_version: CURRENT_SCHEMA,
            attachments,
        }
    }

    #[allow(dead_code)]
    pub fn has_client(&self, client: &str) -> bool {
        self.attachments.iter().any(|a| a.client_type == client)
    }

    #[allow(dead_code)]
    pub fn client_names(&self) -> Vec<String> {
        self.attachments
            .iter()
            .map(|a| a.client_type.clone())
            .collect()
    }

    #[allow(dead_code)]
    pub fn brain_names(&self) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        for a in &self.attachments {
            if !out.contains(&a.brain_name) {
                out.push(a.brain_name.clone());
            }
        }
        out
    }
}

/// Keisei's state directory — `$KEISEI_HOME/.keisei/`.
pub fn keisei_state_dir() -> PathBuf {
    crate::paths::keisei_state_dir()
}

/// Current marker path (v0.21+): `$KEISEI_HOME/.keisei/attached.toml`.
pub fn attached_path() -> PathBuf {
    keisei_state_dir().join(ATTACHED_FILENAME)
}

/// Legacy marker path (v0.20 and earlier).
pub fn legacy_attached_path() -> PathBuf {
    crate::paths::resolve_home()
        .join(".claude")
        .join(LEGACY_ATTACHED_FILENAME)
}

pub fn write(rec: &AttachRecord) -> Result<PathBuf> {
    let path = attached_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let text = toml::to_string_pretty(rec)?;
    std::fs::write(&path, text)?;
    apply_owner_perms(&path)?;
    Ok(path)
}

/// Read the marker, performing one-shot v0.20→v0.21 location migration if
/// the legacy file exists and the new file does not. Older schemas
/// (v1/v2/v3) are migrated in-memory to v4 on read, with a one-line
/// stderr notice so operators see the shape flip.
pub fn read() -> Result<Option<AttachRecord>> {
    migrate_from_legacy()?;
    let path = attached_path();
    if !path.is_file() {
        return Ok(None);
    }
    let raw = std::fs::read_to_string(&path)?;
    let wire: WireRecord = toml::from_str(&raw)?;
    let (rec, old_version) = wire.into_current();
    if let Some(from) = old_version {
        eprintln!(
            "keisei: migrated marker shape v{from} → v{CURRENT_SCHEMA} (in-memory; run any attach/detach to persist)"
        );
    }
    Ok(Some(rec))
}

pub fn delete() -> Result<bool> {
    let path = attached_path();
    if !path.is_file() {
        return Ok(false);
    }
    std::fs::remove_file(&path)?;
    Ok(true)
}

/// If `~/.claude/keisei-attached.toml` exists AND `~/.keisei/attached.toml`
/// does not, move the marker to the new location and emit a stderr notice.
pub fn migrate_from_legacy() -> Result<()> {
    let legacy = legacy_attached_path();
    let current = attached_path();
    if current.is_file() || !legacy.is_file() {
        return Ok(());
    }
    let raw = std::fs::read_to_string(&legacy)?;
    if let Some(parent) = current.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&current, &raw)?;
    apply_owner_perms(&current)?;
    std::fs::remove_file(&legacy)?;
    eprintln!(
        "keisei: migrated marker from ~/.claude/keisei-attached.toml to ~/.keisei/attached.toml"
    );
    Ok(())
}

/// On unix, restrict the marker to owner-only (0o600). No-op on windows.
fn apply_owner_perms(path: &std::path::Path) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(path)?.permissions();
        perms.set_mode(0o600);
        std::fs::set_permissions(path, perms)?;
    }
    #[cfg(not(unix))]
    {
        let _ = path;
    }
    Ok(())
}

/// Thin re-export so call-sites elsewhere in the crate don't have to
/// learn about the new `time` module.
pub fn now_utc_string() -> String {
    time::now_utc_string()
}
