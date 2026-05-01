//! Discovery — locate `llama-cli` / `llama-server` on PATH.
//!
//! ENV override `KEI_LLAMA_CPP_DIR` lets non-standard installs point at
//! a custom directory (binaries inside it take precedence over PATH).
//! Version detection runs `<bin> --version` through the Runner trait and
//! parses the first numeric token after "version".

use crate::error::Result;
use crate::runner::{bin_in_path, Runner};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const ENV_OVERRIDE: &str = "KEI_LLAMA_CPP_DIR";
const ARGS_VERSION: &[&str] = &["--version"];

/// Discovered binaries + version (None if neither found).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct BinPaths {
    pub llama_cli: Option<PathBuf>,
    pub llama_server: Option<PathBuf>,
    pub version: Option<String>,
}

impl BinPaths {
    /// True if at least one of llama-cli / llama-server is present.
    /// Used by the CLI to choose exit 0 vs 2.
    pub fn any_found(&self) -> bool {
        self.llama_cli.is_some() || self.llama_server.is_some()
    }
}

/// Locate both binaries (PATH + ENV override) and ask the runner for
/// their version string. Each step is independent; missing binaries
/// produce `None`, missing version produces `None`.
pub async fn discover<R: Runner + ?Sized>(runner: &R) -> Result<BinPaths> {
    let cli = locate("llama-cli");
    let server = locate("llama-server");
    let version = match (&cli, &server) {
        (Some(p), _) | (_, Some(p)) => fetch_version(runner, p).await,
        (None, None) => None,
    };
    Ok(BinPaths { llama_cli: cli, llama_server: server, version })
}

/// Resolve a binary by name. ENV override wins over PATH.
fn locate(name: &str) -> Option<PathBuf> {
    if let Some(dir) = std::env::var_os(ENV_OVERRIDE) {
        let candidate = PathBuf::from(dir).join(name);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    bin_in_path(name)
}

/// Run `<path> --version` and pull the version string. We accept
/// "version 1234" or "v1.2.3" or any first dotted/numeric token.
async fn fetch_version<R: Runner + ?Sized>(runner: &R, path: &std::path::Path) -> Option<String> {
    let bin = path.to_string_lossy().into_owned();
    let args: Vec<String> = ARGS_VERSION.iter().map(|s| (*s).to_string()).collect();
    let out = runner.run(&bin, &args).await.ok()?;
    parse_version(&out.stdout).or_else(|| parse_version(&out.stderr))
}

/// Best-effort version parse. Looks for "version <X>" or "v<X>" or any
/// dotted/numeric token. Empty input → None.
pub fn parse_version(text: &str) -> Option<String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }
    // Pattern 1: "version 4203" or "version b4203"
    let re_version = Regex::new(r"version\s+(b?\d[\w\-.]*)").ok()?;
    if let Some(cap) = re_version.captures(trimmed) {
        return Some(cap[1].to_string());
    }
    // Pattern 2: "v1.2.3"
    let re_v = Regex::new(r"\bv(\d[\w\-.]*)").ok()?;
    if let Some(cap) = re_v.captures(trimmed) {
        return Some(format!("v{}", &cap[1]));
    }
    // Pattern 3: bare numeric/dotted token
    let re_num = Regex::new(r"\b(\d+\.\d+(?:\.\d+)?[\w\-]*)").ok()?;
    if let Some(cap) = re_num.captures(trimmed) {
        return Some(cap[1].to_string());
    }
    None
}
