//! Conflict input schema (mirror of kei-conflict-scan output).
//!
//! Deserialized locally so this crate does not depend on kei-conflict-scan
//! as a library — the pipe is JSON, both sides speak the same contract.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct Conflict {
    pub category: String,
    pub severity: String,
    pub files: Vec<String>,
    pub evidence: String,
    pub suggested_fix: String,
    pub auto_resolvable: bool,
}

#[derive(Debug, Deserialize)]
struct Wrapper {
    #[serde(default)]
    pub conflicts: Vec<Conflict>,
}

pub fn read_conflicts(path: &Path) -> Result<Vec<Conflict>> {
    let bytes = fs::read(path).with_context(|| format!("read {}", path.display()))?;
    let w: Wrapper = serde_json::from_slice(&bytes).context("parse JSON")?;
    Ok(w.conflicts)
}

pub fn read_from_stdin() -> Result<Vec<Conflict>> {
    use std::io::Read;
    let mut buf = String::new();
    std::io::stdin()
        .read_to_string(&mut buf)
        .context("read stdin")?;
    let w: Wrapper = serde_json::from_str(&buf).context("parse JSON")?;
    Ok(w.conflicts)
}
