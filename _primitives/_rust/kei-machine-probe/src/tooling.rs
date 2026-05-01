//! Local-LLM tooling detection.
//!
//! For each binary of interest:
//!   1. `which <bin>` — present? (non-zero exit ⇒ absent)
//!   2. `<bin> --version` — extract version string (best-effort).
//!
//! Bins probed: `ollama`, `brew`, `llama-server` (llama.cpp's HTTP
//! daemon — the binary kei-llm-llamacpp will spawn). All optional —
//! every detector returns `None` on failure rather than erroring.

use crate::profile::ToolingInfo;
use crate::runner::Runner;
use regex::Regex;

pub fn detect_tooling(runner: &dyn Runner) -> ToolingInfo {
    ToolingInfo {
        ollama: detect_one(runner, "ollama", &["--version"]),
        homebrew: detect_one(runner, "brew", &["--version"]),
        llama_cpp: detect_one(runner, "llama-server", &["--version"]),
    }
}

fn detect_one(runner: &dyn Runner, bin: &str, version_args: &[&str]) -> Option<String> {
    let path = runner.run("which", &[bin]).ok()?;
    if path.trim().is_empty() {
        return None;
    }
    let raw = runner.run(bin, version_args).ok()?;
    Some(extract_version(&raw))
}

/// Pull a version token from `<bin> --version` output. Tries, in order:
///   1. `version[: ] NNNN` style (llama-server, e.g. `version: 4297`)
///   2. `vX.Y.Z` semver (ollama, brew, generic GNU tools)
/// Falls back to the trimmed first non-empty line if neither matched.
fn extract_version(text: &str) -> String {
    if let Some(v) = match_first(r"version[:\s]+(\d+(?:\.\d+){0,3})", text) {
        return v;
    }
    if let Some(v) = match_first(r"v?(\d+\.\d+(?:\.\d+){0,2})", text) {
        return v;
    }
    text.trim().lines().next().unwrap_or("").to_string()
}

fn match_first(pattern: &str, text: &str) -> Option<String> {
    let re = Regex::new(pattern).ok()?;
    let caps = re.captures(text)?;
    Some(caps.get(1)?.as_str().to_string())
}
