//! Agent-id validator — HIGH-security path-traversal defence.
//!
//! Every `agent_id` flowing from task.toml (or auto-gen) into a filesystem
//! path sink MUST pass `validate_agent_id` first. Without this gate, a
//! hostile task.toml with `agent-id = "../../../etc/foo"` reaches
//! `tasks/<agent-id>/` and writes arbitrary paths.
//!
//! Rules (enforced in order, first failure wins):
//!   - non-empty, length ≤ 64
//!   - ASCII-only, matches `^[A-Za-z0-9][A-Za-z0-9_.-]*$`
//!   - rejects `/`, `\`, `..`, leading `.`, leading `-`, NUL, `:`,
//!     whitespace, non-ASCII
//!   - rejects Windows-reserved names (case-insensitive):
//!     CON, PRN, AUX, NUL, COM1-9, LPT1-9
//!
//! Also hosts `autogen_agent_id` (moved from prepare.rs) so the auto-gen
//! output passes the validator by construction.

use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

/// Maximum permitted `agent_id` length (bytes = chars, since ASCII-only).
pub const MAX_AGENT_ID_LEN: usize = 64;

/// Typed error — the sole failure variant of `validate_agent_id`.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
#[error("invalid agent-id: {reason}")]
pub struct InvalidAgentId {
    pub reason: String,
}

impl InvalidAgentId {
    fn new(reason: impl Into<String>) -> Self {
        Self { reason: reason.into() }
    }
}

/// Validate an `agent_id` before it reaches any filesystem path.
pub fn validate_agent_id(raw: &str) -> Result<&str, InvalidAgentId> {
    check_basic_shape(raw)?;
    check_no_traversal_bytes(raw)?;
    check_character_class(raw)?;
    check_not_windows_reserved(raw)?;
    Ok(raw)
}

fn check_basic_shape(raw: &str) -> Result<(), InvalidAgentId> {
    if raw.is_empty() {
        return Err(InvalidAgentId::new("empty"));
    }
    if raw.len() > MAX_AGENT_ID_LEN {
        return Err(InvalidAgentId::new(format!(
            "length {} exceeds max {}",
            raw.len(),
            MAX_AGENT_ID_LEN
        )));
    }
    if !raw.is_ascii() {
        return Err(InvalidAgentId::new("contains non-ASCII"));
    }
    Ok(())
}

fn check_no_traversal_bytes(raw: &str) -> Result<(), InvalidAgentId> {
    if raw.contains("..") {
        return Err(InvalidAgentId::new("contains parent sequence '..'"));
    }
    if raw.contains('/') {
        return Err(InvalidAgentId::new("contains '/'"));
    }
    if raw.contains('\\') {
        return Err(InvalidAgentId::new("contains '\\'"));
    }
    if raw.contains('\0') {
        return Err(InvalidAgentId::new("contains NUL"));
    }
    if raw.contains(':') {
        return Err(InvalidAgentId::new("contains ':'"));
    }
    if raw.chars().any(char::is_whitespace) {
        return Err(InvalidAgentId::new("contains whitespace"));
    }
    Ok(())
}

fn check_character_class(raw: &str) -> Result<(), InvalidAgentId> {
    let first = raw.chars().next().expect("non-empty checked earlier");
    if !first.is_ascii_alphanumeric() {
        return Err(InvalidAgentId::new(format!(
            "must start with [A-Za-z0-9], got '{first}'"
        )));
    }
    for c in raw.chars() {
        if !(c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '-') {
            return Err(InvalidAgentId::new(format!(
                "disallowed character '{c}' (allowed: [A-Za-z0-9_.-])"
            )));
        }
    }
    Ok(())
}

fn check_not_windows_reserved(raw: &str) -> Result<(), InvalidAgentId> {
    let stem = raw.split('.').next().unwrap_or(raw);
    let up = stem.to_ascii_uppercase();
    if is_windows_reserved(&up) {
        return Err(InvalidAgentId::new(format!(
            "Windows-reserved name: '{stem}'"
        )));
    }
    Ok(())
}

fn is_windows_reserved(up: &str) -> bool {
    matches!(up, "CON" | "PRN" | "AUX" | "NUL") || is_com_or_lpt(up)
}

fn is_com_or_lpt(up: &str) -> bool {
    let (prefix, n) = match up.len() {
        4 if up.starts_with("COM") => ("COM", &up[3..]),
        4 if up.starts_with("LPT") => ("LPT", &up[3..]),
        _ => return false,
    };
    let _ = prefix; // already matched
    matches!(n, "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9")
}

/// Auto-generate a fresh `agent_id` whose output is validator-clean.
///
/// Format: `ag-<slugified-role>-<unix-ms-hex>-<4-hex-rand>`
pub fn autogen_agent_id(role: &str) -> String {
    let slug = slugify_role(role);
    let ts_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let rand_hex = format!("{:04x}", rand::random::<u16>());
    let candidate = format!("ag-{slug}-{ts_ms:x}-{rand_hex}");
    // Truncate to cap while preserving the rand-hex suffix.
    truncate_agent_id(&candidate, &rand_hex)
}

/// Slugify a role name into the validator's allowed class.
///
/// Non-allowed characters collapse to `_`; empty result becomes `x` so the
/// auto-gen output is never `ag--<ts>-<rand>` (leading-dash after `ag-`).
pub fn slugify_role(role: &str) -> String {
    let mut out = String::with_capacity(role.len());
    for c in role.chars() {
        if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' {
            out.push(c);
        } else {
            out.push('_');
        }
    }
    if out.is_empty() {
        return "x".to_string();
    }
    let trimmed = out.trim_matches(|c: char| c == '-' || c == '.' || c == '_');
    if trimmed.is_empty() {
        "x".to_string()
    } else {
        trimmed.to_string()
    }
}

fn truncate_agent_id(candidate: &str, rand_hex: &str) -> String {
    if candidate.len() <= MAX_AGENT_ID_LEN {
        return candidate.to_string();
    }
    let keep = MAX_AGENT_ID_LEN.saturating_sub(rand_hex.len() + 1);
    let head = &candidate[..keep.min(candidate.len())];
    let head_trimmed = head.trim_end_matches(|c: char| c == '-' || c == '.' || c == '_');
    format!("{head_trimmed}-{rand_hex}")
}
