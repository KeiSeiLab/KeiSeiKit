//! Shared subprocess helper for backend adapters.
//!
//! Centralises `std::process::Command` so both Hetzner and Vultr backends
//! have a single JSON-exec path. Makes test-time PATH injection uniform.
//!
//! DO NOT pass secrets as CLI args — env-only per RULE 0.8. Error
//! messages redact argv to `<bin> <N args>` and truncate stderr to 200
//! chars to avoid info-disclosure in logs (future-proofing against
//! accidental `--api-key $X` refactors + vultr-cli stderr leaking
//! request URL query params).

use anyhow::{anyhow, Context, Result};
use std::process::Command;

/// Max stderr length retained in error messages before truncation.
const STDERR_MAX: usize = 200;

/// Redact CLI args to `<bin> <N args>` — never echo full argv.
/// Protects against future secret-in-arg refactors (RULE 0.8).
fn redact_args(bin: &str, args: &[&str]) -> String {
    format!("{bin} <{} args>", args.len())
}

/// Truncate stderr to `STDERR_MAX` chars, UTF-8 safe (char-boundary aware).
/// Vultr-cli stderr may echo request URLs with enumeration-useful query
/// params; truncation limits leakage into Claude logs / CI artefacts.
fn truncate_stderr(s: &str) -> String {
    let s = s.trim();
    if s.chars().count() <= STDERR_MAX {
        return s.to_string();
    }
    let mut out = String::with_capacity(STDERR_MAX + 20);
    for (i, ch) in s.chars().enumerate() {
        if i >= STDERR_MAX {
            break;
        }
        out.push(ch);
    }
    out.push_str("... (truncated)");
    out
}

/// Run `bin args…` and return parsed JSON on exit code 0.
/// Returns `Ok(None)` when the child exits non-zero (caller decides if
/// that's an error or an "absent" signal).
pub fn run_json(bin: &str, args: &[&str]) -> Result<Option<serde_json::Value>> {
    let output = Command::new(bin)
        .args(args)
        .output()
        .with_context(|| format!("failed to spawn `{bin}`"))?;
    if !output.status.success() {
        return Ok(None);
    }
    let stdout = String::from_utf8(output.stdout)
        .with_context(|| format!("`{bin}` stdout not utf-8"))?;
    if stdout.trim().is_empty() {
        return Ok(None);
    }
    let v: serde_json::Value = serde_json::from_str(&stdout)
        .with_context(|| format!("`{bin}` did not emit valid JSON"))?;
    Ok(Some(v))
}

/// Run `bin args…` and fail loudly on non-zero (create/delete paths).
/// Returns the parsed JSON or `None` for empty output.
pub fn run_json_strict(bin: &str, args: &[&str]) -> Result<Option<serde_json::Value>> {
    let output = Command::new(bin)
        .args(args)
        .output()
        .with_context(|| format!("failed to spawn `{bin}`"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!(
            "`{}` failed (code {:?}): {}",
            redact_args(bin, args),
            output.status.code(),
            truncate_stderr(&stderr)
        ));
    }
    let stdout = String::from_utf8(output.stdout)
        .with_context(|| format!("`{bin}` stdout not utf-8"))?;
    if stdout.trim().is_empty() {
        return Ok(None);
    }
    let v: serde_json::Value = serde_json::from_str(&stdout)
        .with_context(|| format!("`{bin}` did not emit valid JSON"))?;
    Ok(Some(v))
}

/// Plain void run — success = ok, failure = err with stderr context.
pub fn run_void(bin: &str, args: &[&str]) -> Result<()> {
    let output = Command::new(bin)
        .args(args)
        .output()
        .with_context(|| format!("failed to spawn `{bin}`"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!(
            "`{}` failed (code {:?}): {}",
            redact_args(bin, args),
            output.status.code(),
            truncate_stderr(&stderr)
        ));
    }
    Ok(())
}

/// Assert a CLI binary is on PATH (friendly error).
pub fn require_cli(bin: &str, install_hint: &str) -> Result<()> {
    which(bin).map(|_| ()).ok_or_else(|| {
        anyhow!("`{bin}` not found on PATH. Install: {install_hint}")
    })
}

fn which(bin: &str) -> Option<std::path::PathBuf> {
    let path = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path) {
        let candidate = dir.join(bin);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}

/// Assert an env var is set + non-empty (friendly error).
pub fn require_env(var: &str) -> Result<String> {
    match std::env::var(var) {
        Ok(v) if !v.is_empty() => Ok(v),
        _ => Err(anyhow!(
            "env {var} not set. Source ~/.claude/secrets/.env first (RULE 0.8)."
        )),
    }
}
