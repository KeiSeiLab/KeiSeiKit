//! `.KEI_FORK_META.toml` — on-disk metadata written once by `create()`
//! and read by `list()` / `collect()` / `rescue()` / `gc()`.
//!
//! Layout is stable: `agent_id`, `started_ts`, `base_branch`, `ledger_id`.
//! Never add fields without bumping a schema version.

use crate::error::Error;
use crate::handle::ForkHandle;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const META_FILENAME: &str = ".KEI_FORK_META.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForkMeta {
    pub agent_id: String,
    pub started_ts: i64,
    pub base_branch: String,
    pub ledger_id: String,
}

impl ForkMeta {
    pub fn branch(&self) -> String {
        format!("fork/{}", self.agent_id)
    }

    pub fn into_handle(self, worktree: PathBuf) -> ForkHandle {
        let branch = self.branch();
        ForkHandle {
            agent_id: self.agent_id,
            worktree,
            branch,
            ledger_id: self.ledger_id,
            started_ts: self.started_ts,
        }
    }
}

pub fn write_meta(worktree: &Path, meta: &ForkMeta) -> Result<(), Error> {
    let body = toml::to_string(meta).map_err(|e| Error::Meta(e.to_string()))?;
    fs::write(worktree.join(META_FILENAME), body)?;
    Ok(())
}

pub fn read_meta(worktree: &Path) -> Result<ForkMeta, Error> {
    let raw = fs::read_to_string(worktree.join(META_FILENAME))?;
    toml::from_str(&raw).map_err(|e| Error::Meta(e.to_string()))
}
