//! Configuration loader — see CLEANUP-RUNTIME-SPEC.md §3.5.
//!
//! Loads `cleanup.toml` from the workspace root. Falls back to
//! Constructor-Pattern defaults (file ≤200 LOC, fn ≤30 LOC) when
//! absent. v0.1 only honours the `[loc]` and `[scanners]` sections;
//! the rest of §3.5 (`[naming_pairs]`, `[coverage_map]`,
//! `[todo_age]`) is forward-compatible-parsed but ignored.

use crate::error::CleanupError;
use serde::Deserialize;
use std::path::Path;

/// Top-level config — Constructor Pattern: 1 struct = 1 concern.
#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    /// LOC limits (Rule ZERO).
    #[serde(default)]
    pub loc: LocConfig,
    /// Which scanners to run (empty = all).
    #[serde(default)]
    pub scanners: ScannersConfig,
}

/// `[loc]` section — see CLEANUP-RUNTIME-SPEC.md §3.5.
#[derive(Clone, Debug, Deserialize)]
pub struct LocConfig {
    /// Max LOC per file (default 200, Constructor Pattern).
    #[serde(default = "default_file_max")]
    pub file_max: usize,
    /// Max LOC per function (default 30).
    #[serde(default = "default_fn_max")]
    pub fn_max: usize,
    /// Path substrings to exclude from LOC scan.
    #[serde(default = "default_exclude")]
    pub exclude_paths: Vec<String>,
}

/// `[scanners]` section — see CLEANUP-RUNTIME-SPEC.md §3.5.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct ScannersConfig {
    /// Enabled scanner names (empty = run all defaults).
    #[serde(default)]
    pub enabled: Vec<String>,
}

fn default_file_max() -> usize {
    200
}
fn default_fn_max() -> usize {
    30
}
fn default_exclude() -> Vec<String> {
    vec!["target/".into(), "_archive/".into(), ".git/".into()]
}

impl Default for LocConfig {
    fn default() -> Self {
        Self {
            file_max: default_file_max(),
            fn_max: default_fn_max(),
            exclude_paths: default_exclude(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            loc: LocConfig::default(),
            scanners: ScannersConfig::default(),
        }
    }
}

impl Config {
    /// Load from `<workspace>/cleanup.toml` if present, else defaults.
    pub fn load_or_default(workspace: &Path) -> Result<Self, CleanupError> {
        let cfg_path = workspace.join("cleanup.toml");
        if !cfg_path.exists() {
            return Ok(Self::default());
        }
        let text = std::fs::read_to_string(&cfg_path)?;
        toml::from_str::<Self>(&text).map_err(|e| CleanupError::Manifest {
            path: cfg_path,
            detail: e.to_string(),
        })
    }

    /// Whether `name` is enabled (empty whitelist = all enabled).
    pub fn scanner_enabled(&self, name: &str) -> bool {
        self.scanners.enabled.is_empty() || self.scanners.enabled.iter().any(|s| s == name)
    }

    /// Whether `path` matches any exclude entry.
    pub fn excluded(&self, path: &Path) -> bool {
        let s = path.to_string_lossy();
        self.loc.exclude_paths.iter().any(|p| s.contains(p))
    }
}
