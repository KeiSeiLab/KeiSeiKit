//! precedent — env-gated advisory check against `kei-dna-index`.
//!
//! When `KEI_SPAWN_PRECEDENT_CHECK=1`, spawn.rs calls `run_advisory` after
//! it has composed the body but before `kei-ledger fork`. We shell out
//! to `kei-dna-index precedent --body <sha>`; if the primitive returns
//! a non-empty JSON array of prior-agent matches, we eprintln a WARN
//! and return the hit count. We never block the spawn — this is a
//! human-facing signal, not a gate.
//!
//! Constructor Pattern: one module = one responsibility (precedent
//! advisory only). No git, no ledger write, no filesystem mutation.

use anyhow::{anyhow, Result};
use std::process::Command;

/// Env flag that enables this advisory. Absent → `run_advisory` is a
/// silent no-op returning `Ok(0)`.
pub const ENABLE_ENV: &str = "KEI_SPAWN_PRECEDENT_CHECK";

/// Env override for the `kei-dna-index` binary path. Default = PATH lookup.
pub const BIN_ENV: &str = "KEI_DNA_INDEX_BIN";

/// Run the advisory. Returns the number of prior-agent hits reported
/// by `kei-dna-index precedent`. When the env flag is absent, returns
/// `Ok(0)` without invoking anything — keep fast path fast.
pub fn run_advisory(body_sha: &str) -> Result<usize> {
    if std::env::var(ENABLE_ENV).is_err() {
        return Ok(0);
    }
    if body_sha.is_empty() {
        return Err(anyhow!("precedent advisory called with empty body_sha"));
    }
    let bin = dna_index_bin();
    let stdout = match capture_stdout(&bin, body_sha) {
        Some(s) => s,
        None => return Ok(0),
    };
    let hits = parse_hits(&stdout);
    warn_if_hits(hits, body_sha);
    Ok(hits)
}

/// Shell to kei-dna-index and capture stdout on success. Any failure
/// (binary missing, non-zero exit) is logged + returns `None` so the
/// caller short-circuits to `Ok(0)` (best-effort advisory).
fn capture_stdout(bin: &str, body_sha: &str) -> Option<String> {
    let mut cmd = Command::new(bin);
    cmd.args(["precedent", "--body", body_sha]);
    let output = match cmd.output() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("kei-spawn precedent: skipped ({bin} not runnable: {e})");
            return None;
        }
    };
    if !output.status.success() {
        eprintln!(
            "kei-spawn precedent: skipped (exit {}: {})",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        );
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).to_string())
}

fn warn_if_hits(hits: usize, body_sha: &str) {
    if hits > 0 {
        eprintln!(
            "kei-spawn precedent WARN: {hits} prior agent(s) share body_sha={body_sha} — review before spawn",
        );
    }
}

/// Resolve the kei-dna-index binary, env override first.
fn dna_index_bin() -> String {
    if let Ok(b) = std::env::var(BIN_ENV) {
        return b;
    }
    "kei-dna-index".into()
}

/// Minimal JSON-array-length parser. kei-dna-index `precedent` emits a
/// JSON array. We only need the element count — avoid adding serde_json
/// coupling to this single call site by counting top-level `{`. On
/// malformed output, return 0 (advisory is best-effort).
fn parse_hits(stdout: &str) -> usize {
    let trimmed = stdout.trim();
    if trimmed.is_empty() || trimmed == "[]" {
        return 0;
    }
    serde_json::from_str::<serde_json::Value>(trimmed)
        .ok()
        .and_then(|v| v.as_array().map(|a| a.len()))
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_array() {
        assert_eq!(parse_hits("[]"), 0);
        assert_eq!(parse_hits(""), 0);
        assert_eq!(parse_hits("   "), 0);
    }

    #[test]
    fn parse_two_element_array() {
        let n = parse_hits(r#"[{"id":"a"},{"id":"b"}]"#);
        assert_eq!(n, 2);
    }

    #[test]
    fn parse_malformed_returns_zero() {
        assert_eq!(parse_hits("not json"), 0);
        assert_eq!(parse_hits("{}"), 0);
    }
}
