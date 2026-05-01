//! TOML config loader.
//!
//! Example `store-config.toml`:
//!
//! ```toml
//! [active]
//! backend = "github"
//!
//! [github]
//! url = "git@github.com:user/memory-repo.git"
//! ssh_key_env = "KEI_MEMORY_SSH_KEY"
//!
//! [filesystem]
//! path = "~/.claude/memory/sync-repo"
//! ```
//!
//! Secrets (PATs, SSH keys) live in `~/.claude/secrets/.env` per RULE 0.8;
//! this file only stores env-var NAMES.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Config {
    pub active: Active,
    #[serde(default)]
    pub filesystem: FilesystemCfg,
    #[serde(default)]
    pub github: GitRemoteCfg,
    #[serde(default)]
    pub forgejo: GitRemoteCfg,
    #[serde(default)]
    pub gitea: GitRemoteCfg,
    #[serde(default)]
    pub s3: S3Cfg,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Active {
    pub backend: String,
    #[serde(default = "default_local_path")]
    pub local_path: String,
}

fn default_local_path() -> String {
    "~/.claude/memory/sync-repo".to_string()
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FilesystemCfg {
    #[serde(default)]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct GitRemoteCfg {
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub ssh_key_env: Option<String>,
    #[serde(default)]
    pub pat_env: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct S3Cfg {
    #[serde(default)]
    pub endpoint: Option<String>,
    #[serde(default)]
    pub bucket: Option<String>,
    #[serde(default)]
    pub region: Option<String>,
    #[serde(default)]
    pub access_key_env: Option<String>,
    #[serde(default)]
    pub secret_key_env: Option<String>,
    /// Local cache / manifest root. REQUIRED — S3 impl stores a manifest
    /// there and (in stub mode) serves read/write from the cache.
    #[serde(default)]
    pub cache_path: Option<String>,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let text = fs::read_to_string(path)
            .with_context(|| format!("read {}", path.display()))?;
        let cfg: Config = toml::from_str(&text).context("parse store-config.toml")?;
        Ok(cfg)
    }

    pub fn expanded_local_path(&self) -> String {
        expand_tilde(&self.active.local_path)
    }
}

pub fn expand_tilde(p: &str) -> String {
    if let Some(rest) = p.strip_prefix("~/") {
        if let Ok(home) = std::env::var("HOME") {
            return format!("{home}/{rest}");
        }
    }
    p.to_string()
}
