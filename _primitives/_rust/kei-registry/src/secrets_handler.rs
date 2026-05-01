//! Handler for the `secrets` subcommand.
//!
//! Constructor Pattern: this cube owns the secrets command dispatch only.
//! Env-file resolution + report output. No scanner logic — that lives
//! in `secrets.rs`.

use anyhow::Result;
use std::path::{Path, PathBuf};

use crate::handlers::Outcome;
use crate::secrets::{compute_secrets_report, render_ascii};

/// Top-level handler wired from `handlers::dispatch`.
pub fn handle_secrets(
    mut env_files: Vec<PathBuf>,
    scan_root: PathBuf,
    format: String,
) -> Result<Outcome> {
    let root = scan_root.canonicalize().unwrap_or(scan_root);
    if env_files.is_empty() {
        env_files = resolve_default_env_files(&root);
    }
    let report = compute_secrets_report(&env_files, &root)?;
    match format.as_str() {
        "json" => println!("{}", serde_json::to_string_pretty(&report)?),
        _ => print!("{}", render_ascii(&report)),
    }
    Ok(Outcome::Ok)
}

/// Resolve default env files when user provides none.
///
/// Priority order:
/// 1. `~/.claude/secrets/.env` (umbrella SSoT per RULE 0.8)
/// 2. `<scan_root>/secrets/*.env` (per-project secrets)
fn resolve_default_env_files(scan_root: &Path) -> Vec<PathBuf> {
    let mut result = Vec::new();
    let umbrella = umbrella_env_path();
    if umbrella.exists() {
        result.push(umbrella);
    }
    let secrets_dir = scan_root.join("secrets");
    if let Ok(rd) = std::fs::read_dir(&secrets_dir) {
        let mut local: Vec<PathBuf> = rd
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("env"))
            .collect();
        local.sort();
        result.extend(local);
    }
    result
}

fn umbrella_env_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    PathBuf::from(home).join(".claude").join("secrets").join(".env")
}
