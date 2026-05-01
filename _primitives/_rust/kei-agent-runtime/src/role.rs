//! Role expression resolver — Layer E.
//!
//! Parses `_roles/<name>.toml` and resolves `extends` chains with `relaxes`
//! subtraction, emitting a flat `ResolvedRole` for downstream consumers
//! (`compose`, `prepare`, `verify`, `dna`).
//!
//! Semantics:
//!   - `extends` — optional parent role slug; loaded recursively.
//!   - `required` (local) — merged on top of parent's resolved required.
//!   - `relaxes` — slugs in parent's resolved required to DROP. A warning is
//!     collected in `ResolvedRole::warnings` if a relaxed cap wasn't present
//!     in the inherited set (caller decides how to surface).
//!   - Cycle detection — visited set passed down the recursion; an error
//!     with a clear path is returned when a cycle is found.
//!   - Depth cap — `extends` chains deeper than `MAX_DEPTH = 16` are
//!     refused (`RoleError::MaxDepthExceeded`) to prevent stack overflow
//!     on malformed/hostile role trees.
//!   - Name validation — role slug must match `^[a-z][a-z0-9-]{0,63}$`,
//!     blocks `../../etc/passwd` path traversal before the `join`.
//!
//! Constructor Pattern: one cube = one responsibility (role expression only).
//! No I/O beyond `std::fs::read_to_string`. Dispatched from `compose::load_role`
//! and `verify::load_role_capabilities` so both share the same semantics.

use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashSet;
use std::path::Path;
use thiserror::Error;

/// Max depth for `extends` chain traversal. Guards against stack overflow
/// on malformed/hostile role files.
pub const MAX_DEPTH: usize = 16;

/// Role / capability slug pattern. Lowercase start, `[a-z0-9-]` body,
/// ≤64 chars total. Blocks `..`, `/`, `\`, upper-case, unicode,
/// whitespace — any of which enables path traversal via `Path::join`.
static NAME_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-z][a-z0-9-]{0,63}$").expect("compile NAME_RE"));

/// Structured errors from role resolution.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum RoleError {
    #[error("role `extends` chain exceeded MAX_DEPTH={depth}; trace: {trace:?}")]
    MaxDepthExceeded { depth: usize, trace: Vec<String> },
    #[error("invalid {kind} name `{value}` — must match ^[a-z][a-z0-9-]{{0,63}}$")]
    InvalidName { kind: &'static str, value: String },
    #[error("cycle detected in role `extends` chain at `{role}` (visited: {visited:?})")]
    Cycle {
        role: String,
        visited: Vec<String>,
    },
}

/// Flattened role ready for downstream composition.
#[derive(Debug, Clone, Default)]
pub struct ResolvedRole {
    /// Ordered capability names after `extends` merge + `relaxes` subtraction.
    pub required: Vec<String>,
    /// Non-fatal advisories surfaced during resolution (e.g. relaxed cap
    /// was not in the inherited set). Caller decides how to surface.
    pub warnings: Vec<String>,
}

/// Deserialized role file (raw shape, pre-resolution).
#[derive(Debug, Default, Deserialize)]
pub struct RoleFileRaw {
    #[serde(default)]
    pub capabilities: RoleCapsRaw,
}

#[derive(Debug, Default, Deserialize)]
pub struct RoleCapsRaw {
    #[serde(default)]
    pub extends: Option<String>,
    #[serde(default)]
    pub required: Vec<String>,
    #[serde(default)]
    pub relaxes: Vec<String>,
}

/// Validate a role-or-capability slug; returns typed error if malformed.
pub fn validate_name(kind: &'static str, value: &str) -> Result<(), RoleError> {
    if NAME_RE.is_match(value) {
        Ok(())
    } else {
        Err(RoleError::InvalidName {
            kind,
            value: value.to_string(),
        })
    }
}

/// Resolve a role by slug; read role file, walk `extends`, apply `relaxes`.
pub fn resolve_role(kit_root: &Path, role: &str) -> Result<ResolvedRole> {
    validate_name("role", role)?;
    let mut visited: HashSet<String> = HashSet::new();
    let mut warnings: Vec<String> = Vec::new();
    let required = resolve_inner(kit_root, role, &mut visited, &mut warnings, 0)?;
    Ok(ResolvedRole { required, warnings })
}

fn resolve_inner(
    kit_root: &Path,
    role: &str,
    visited: &mut HashSet<String>,
    warnings: &mut Vec<String>,
    depth: usize,
) -> Result<Vec<String>> {
    if depth > MAX_DEPTH {
        return Err(RoleError::MaxDepthExceeded {
            depth: MAX_DEPTH,
            trace: visited.iter().cloned().collect(),
        }
        .into());
    }
    if !visited.insert(role.to_string()) {
        return Err(RoleError::Cycle {
            role: role.to_string(),
            visited: visited.iter().cloned().collect(),
        }
        .into());
    }
    let raw = read_role_file(kit_root, role)?;
    let mut merged = match raw.capabilities.extends.as_deref() {
        Some(parent) => {
            validate_name("role", parent)?;
            resolve_inner(kit_root, parent, visited, warnings, depth + 1)?
        }
        None => Vec::new(),
    };
    for cap in &raw.capabilities.required {
        if !merged.iter().any(|c| c == cap) {
            merged.push(cap.clone());
        }
    }
    for dropped in &raw.capabilities.relaxes {
        let before = merged.len();
        merged.retain(|c| c != dropped);
        if merged.len() == before {
            warnings.push(format!(
                "role `{role}` relaxes `{dropped}` but it was not in the \
                 inherited capability set — no-op"
            ));
        }
    }
    visited.remove(role);
    Ok(merged)
}

fn read_role_file(kit_root: &Path, role: &str) -> Result<RoleFileRaw> {
    validate_name("role", role)?;
    let path = kit_root.join("_roles").join(format!("{role}.toml"));
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("read role file {}", path.display()))?;
    toml::from_str::<RoleFileRaw>(&text)
        .with_context(|| format!("parse role TOML {}", path.display()))
}
