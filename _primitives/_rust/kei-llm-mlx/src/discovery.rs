//! Binary discovery — `which mlx_lm.generate` / `which mlx_lm.server`.
//!
//! Constructor Pattern: ONE cube finds the two mlx_lm CLI entry points and
//! captures their version. Goes through `Runner` so tests can simulate
//! present-OR-absent without `pip install mlx_lm`.
//!
//! ENV override: `KEI_MLX_BIN=/path/to/dir` — when set, `which` is
//! bypassed and we look for `mlx_lm.generate` / `mlx_lm.server` directly
//! under that directory. Useful for sandbox/CI hosts.

use crate::runner::Runner;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const GEN_BIN: &str = "mlx_lm.generate";
const SRV_BIN: &str = "mlx_lm.server";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct MlxBins {
    /// Absolute path to `mlx_lm.generate`, if found.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate: Option<PathBuf>,
    /// Absolute path to `mlx_lm.server`, if found.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<PathBuf>,
    /// Best-effort version string parsed from `--help` first line.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl MlxBins {
    pub fn any_present(&self) -> bool {
        self.generate.is_some() || self.server.is_some()
    }
}

/// Public API — discover binaries via `Runner`.
pub fn discover(runner: &dyn Runner) -> MlxBins {
    if let Some(dir) = std::env::var_os("KEI_MLX_BIN") {
        return discover_in_dir(PathBuf::from(dir), runner);
    }
    discover_via_which(runner)
}

fn discover_in_dir(dir: PathBuf, runner: &dyn Runner) -> MlxBins {
    let gen_p = dir.join(GEN_BIN);
    let srv_p = dir.join(SRV_BIN);
    let generate = if gen_p.exists() { Some(gen_p) } else { None };
    let server = if srv_p.exists() { Some(srv_p) } else { None };
    let version = generate.as_ref().and_then(|p| version_via_help(p, runner));
    MlxBins { generate, server, version }
}

fn discover_via_which(runner: &dyn Runner) -> MlxBins {
    let generate = which_one(runner, GEN_BIN);
    let server = which_one(runner, SRV_BIN);
    let version = generate.as_ref().and_then(|p| version_via_help(p, runner));
    MlxBins { generate, server, version }
}

/// Single `which X` lookup. Returns `None` when stdout empty / non-zero
/// exit / Runner error.
fn which_one(runner: &dyn Runner, bin: &str) -> Option<PathBuf> {
    match runner.run("which", &[bin]) {
        Ok(r) if r.is_success() => {
            let trimmed = r.stdout.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(PathBuf::from(trimmed))
            }
        }
        _ => None,
    }
}

/// Parse a version stamp from `<bin> --help` first line. mlx_lm prints
/// e.g. `usage: mlx_lm.generate ...` then `MLX-LM 0.20.4` somewhere in
/// the body. Best-effort regex; returns `None` if no match.
fn version_via_help(bin: &std::path::Path, runner: &dyn Runner) -> Option<String> {
    let bin_s = bin.to_string_lossy();
    let r = runner.run(&bin_s, &["--help"]).ok()?;
    if !r.is_success() {
        return None;
    }
    extract_version(&r.stdout)
}

/// Pull `X.Y.Z` from typical mlx_lm help output. Public so tests can
/// pin behaviour.
pub fn extract_version(text: &str) -> Option<String> {
    let re = regex::Regex::new(r"(?i)mlx[-_ ]lm[^0-9]*([0-9]+\.[0-9]+\.[0-9]+)").ok()?;
    re.captures(text)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
}
