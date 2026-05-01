//! `Registry` — load `models.toml`, expose query helpers.
//!
//! Resolution order for the catalog path:
//! 1. Explicit `--models-toml <path>` flag (caller-supplied)
//! 2. `KEI_MODEL_REGISTRY` env var
//! 3. `<CARGO_MANIFEST_DIR>/data/models.toml` (compiled-in default)
//!
//! The crate ships `data/models.toml` next to its `Cargo.toml`, so the
//! compiled-in default works for both `cargo run` and a copied release binary
//! provided the binary is invoked from inside the repo. Downstream callers
//! should pass `--models-toml` or set the env var explicitly.

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

use crate::model::{Capability, Model, Provider, Status};

#[derive(Debug, Deserialize)]
struct CatalogFile {
    #[serde(default)]
    models: Vec<Model>,
}

#[derive(Debug, Clone)]
pub struct Registry {
    models: Vec<Model>,
    source: PathBuf,
}

impl Registry {
    /// Load the catalog from a path, returning a parsed registry.
    pub fn load(path: &Path) -> Result<Self> {
        let txt = std::fs::read_to_string(path)
            .with_context(|| format!("read models.toml at {}", path.display()))?;
        let parsed: CatalogFile = toml::from_str(&txt)
            .with_context(|| format!("parse models.toml at {}", path.display()))?;
        Ok(Self { models: parsed.models, source: path.to_path_buf() })
    }

    /// Resolve catalog path: arg → env → compiled-in default.
    pub fn resolve_path(arg: Option<&Path>) -> Result<PathBuf> {
        if let Some(p) = arg {
            return Ok(p.to_path_buf());
        }
        if let Ok(env_path) = std::env::var("KEI_MODEL_REGISTRY") {
            return Ok(PathBuf::from(env_path));
        }
        let default = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/models.toml");
        if !default.exists() {
            return Err(anyhow!("default catalog missing: {}", default.display()));
        }
        Ok(default)
    }

    pub fn source_path(&self) -> &Path {
        &self.source
    }

    pub fn list_all(&self) -> &[Model] {
        &self.models
    }

    pub fn by_provider(&self, p: Provider) -> Vec<&Model> {
        self.models.iter().filter(|m| m.provider == p).collect()
    }

    pub fn by_cap(&self, c: Capability) -> Vec<&Model> {
        self.models.iter().filter(|m| m.capabilities.contains(&c)).collect()
    }

    pub fn by_status(&self, s: Status) -> Vec<&Model> {
        self.models.iter().filter(|m| m.status == s).collect()
    }

    pub fn by_role_tag(&self, tag: &str) -> Vec<&Model> {
        self.models.iter().filter(|m| m.has_role(tag)).collect()
    }

    pub fn get(&self, id: &str) -> Option<&Model> {
        self.models.iter().find(|m| m.id == id)
    }
}
