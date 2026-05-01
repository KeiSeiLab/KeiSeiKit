//! Models — recursively scan a directory for `.gguf` files.
//!
//! Default search dirs:
//!   - `~/.cache/llama.cpp/`
//!   - `~/Library/Application Support/llama.cpp/models/` (macOS)
//!
//! Quant detection is conservative: only well-known patterns map to a
//! quant string (Q4_0 / Q4_K_M / Q5_K_S / Q6_K / Q8_0 / F16 / F32).
//! Unknown filenames produce `quant: None`.

use crate::error::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const QUANT_PATTERN: &str =
    r"(?i)\b(Q[2-8]_K(?:_[SML])?|Q4_0|Q4_1|Q5_0|Q5_1|Q8_0|F16|F32|BF16)\b";

/// One discovered .gguf file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelEntry {
    pub path: PathBuf,
    pub name: String,
    pub size_bytes: u64,
    pub quant: Option<String>,
}

/// Scan `dir` recursively for .gguf files. Non-existent dir → empty Vec.
/// Errors during recursion are silently skipped (best-effort discovery).
pub fn list_models(dir: &Path) -> Result<Vec<ModelEntry>> {
    let mut out = Vec::new();
    if !dir.exists() {
        return Ok(out);
    }
    walk_dir(dir, &mut out);
    out.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(out)
}

/// Default search roots that should be probed by the `models` subcommand
/// when no `--dir` is supplied. macOS-specific path is included
/// unconditionally — it just won't exist on Linux.
pub fn default_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Some(home) = std::env::var_os("HOME") {
        let home = PathBuf::from(home);
        dirs.push(home.join(".cache").join("llama.cpp"));
        dirs.push(
            home.join("Library")
                .join("Application Support")
                .join("llama.cpp")
                .join("models"),
        );
    }
    dirs
}

/// Recursively traverse `dir`, appending .gguf entries to `out`.
fn walk_dir(dir: &Path, out: &mut Vec<ModelEntry>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk_dir(&path, out);
        } else if is_gguf(&path) {
            if let Some(model) = build_entry(&path) {
                out.push(model);
            }
        }
    }
}

fn is_gguf(path: &Path) -> bool {
    path.extension().and_then(|e| e.to_str()) == Some("gguf")
}

/// Build a `ModelEntry` from a .gguf path. Returns None on metadata error.
fn build_entry(path: &Path) -> Option<ModelEntry> {
    let meta = path.metadata().ok()?;
    let name = path.file_name()?.to_string_lossy().into_owned();
    let quant = detect_quant(&name);
    Some(ModelEntry {
        path: path.to_path_buf(),
        name,
        size_bytes: meta.len(),
        quant,
    })
}

/// Conservative quant detection. Returns canonical uppercase form.
pub fn detect_quant(name: &str) -> Option<String> {
    let re = Regex::new(QUANT_PATTERN).ok()?;
    re.captures(name)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_uppercase())
}
