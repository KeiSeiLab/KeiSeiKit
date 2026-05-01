//! Model discovery — scan HuggingFace cache for MLX-quantised entries.
//!
//! mlx_lm caches models in `~/.cache/huggingface/hub/`, with each entry
//! named `models--<org>--<repo>`. We treat an entry as "MLX-quantised" if
//! its repo segment matches one of:
//!   *-mlx-q4 / *-mlx-q8 / *-4bit / *-8bit / *-mlx (suffix)
//! and infer `quant_bits` from the suffix.
//!
//! Constructor Pattern: this cube ONLY scans the directory tree and
//! classifies via regex. No network, no Runner, no I/O beyond `read_dir`.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelEntry {
    /// HuggingFace id, e.g. `mlx-community/Llama-3.2-3B-Instruct-4bit`.
    pub hf_id: String,
    /// Absolute on-disk cache directory.
    pub local_path: PathBuf,
    /// Best-effort quantisation width parsed from the repo suffix.
    /// `None` when the repo just ends in `-mlx` without bit info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quant_bits: Option<u8>,
}

/// Public API — scan a HuggingFace hub cache directory and return all
/// MLX-quantised model entries. Missing dir = empty list (NOT an error).
pub fn list_models(cache_dir: &Path) -> Vec<ModelEntry> {
    let mut out = Vec::new();
    let read = match std::fs::read_dir(cache_dir) {
        Ok(r) => r,
        Err(_) => return out,
    };
    for entry in read.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let name = match path.file_name().and_then(|s| s.to_str()) {
            Some(s) => s.to_string(),
            None => continue,
        };
        if let Some(model) = classify(&name, &path) {
            out.push(model);
        }
    }
    out.sort_by(|a, b| a.hf_id.cmp(&b.hf_id));
    out
}

/// Decide whether `name` looks like a HF cache dir for an MLX-quantised
/// model and, if so, build a `ModelEntry`. Visible for unit tests.
pub fn classify(name: &str, path: &Path) -> Option<ModelEntry> {
    let hf_id = hf_id_from_dirname(name)?;
    if !is_mlx_quantised(&hf_id) {
        return None;
    }
    let quant_bits = parse_quant_bits(&hf_id);
    Some(ModelEntry { hf_id, local_path: path.to_path_buf(), quant_bits })
}

/// Convert `models--org--repo` into `org/repo`. Returns `None` for any
/// non-conforming name (skipped by caller).
fn hf_id_from_dirname(name: &str) -> Option<String> {
    let stripped = name.strip_prefix("models--")?;
    let mut parts = stripped.splitn(2, "--");
    let org = parts.next()?;
    let repo = parts.next()?;
    Some(format!("{org}/{repo}"))
}

/// Match the five canonical suffix patterns. Visible for unit tests.
pub fn is_mlx_quantised(hf_id: &str) -> bool {
    let lower = hf_id.to_lowercase();
    lower.ends_with("-mlx-q4")
        || lower.ends_with("-mlx-q8")
        || lower.ends_with("-4bit")
        || lower.ends_with("-8bit")
        || lower.ends_with("-mlx")
}

/// Parse 4 or 8 from the suffix. `None` for plain `-mlx`.
pub fn parse_quant_bits(hf_id: &str) -> Option<u8> {
    let lower = hf_id.to_lowercase();
    if lower.ends_with("-4bit") || lower.ends_with("-mlx-q4") {
        Some(4)
    } else if lower.ends_with("-8bit") || lower.ends_with("-mlx-q8") {
        Some(8)
    } else {
        None
    }
}

/// Default cache dir, `~/.cache/huggingface/hub`. Returns `None` if HOME
/// is unset (rare; CI/tests pass an explicit dir).
pub fn default_cache_dir() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|h| {
        let mut p = PathBuf::from(h);
        p.push(".cache/huggingface/hub");
        p
    })
}
