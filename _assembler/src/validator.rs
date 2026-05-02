//! Manifest validator. Enforces Constructor Pattern invariants.
//! Hard-fails on missing obligatory blocks, missing handoffs, unknown blocks.
//!
//! Detailed sub-checks live in their own cubes:
//!   - `placeholders::check`      — {{PLACEHOLDER}} substitution guard
//!   - `schemas_export::load`     — dynamic artifact-schema whitelist loader
//!   - `validator_tests`          — unit tests (split per Constructor Pattern)

use crate::manifest::Manifest;
use crate::placeholders;
use crate::schemas_export;
use crate::substrate;
use std::collections::BTreeSet;
use std::path::Path;

pub const OBLIGATORY: &[&str] = &["baseline", "evidence-grading", "memory-protocol"];

/// Back-compat alias for external callers. The SSoT lives in
/// `schemas_export::BUILTIN`.
#[allow(dead_code)]
pub const KNOWN_ARTIFACT_SCHEMAS: &[&str] = schemas_export::BUILTIN;

pub fn validate(m: &Manifest, blocks_dir: &Path) -> Result<(), String> {
    check_obligatory_blocks(m)?;
    check_blocks_exist(m, blocks_dir)?;
    check_handoff_targets(m, blocks_dir)?;
    check_domain_fields(m)?;
    placeholders::check(m)?;
    let known = schemas_export::load(blocks_dir);
    check_artifact_schemas(m, &known)?;
    check_substrate_role(m, blocks_dir)?;
    Ok(())
}

fn check_obligatory_blocks(m: &Manifest) -> Result<(), String> {
    for required in OBLIGATORY {
        if !m.blocks.iter().any(|b| b == required) {
            return Err(format!("missing obligatory block: {required}"));
        }
    }
    if m.handoff.is_empty() {
        return Err("at least one handoff required".into());
    }
    Ok(())
}

fn check_blocks_exist(m: &Manifest, blocks_dir: &Path) -> Result<(), String> {
    for block in &m.blocks {
        let path = blocks_dir.join(format!("{block}.md"));
        if !path.exists() {
            return Err(format!("block '{block}' not found at {}", path.display()));
        }
    }
    Ok(())
}

fn check_domain_fields(m: &Manifest) -> Result<(), String> {
    if m.domain_in.is_empty() {
        return Err("domain_in must have at least one entry".into());
    }
    if m.forbidden_domain.is_empty() {
        return Err("forbidden_domain must have at least one entry".into());
    }
    if m.role.trim().is_empty() {
        return Err("role must not be empty".into());
    }
    Ok(())
}

/// Verify every `handoff[i].target` resolves to `_manifests/<target>.toml`.
/// Prevents dangling references to unauthored manifests from silently
/// passing validation. See audit 2026-05-02.
pub fn check_handoff_targets(m: &Manifest, blocks_dir: &Path) -> Result<(), String> {
    let manifests_dir = blocks_dir
        .parent()
        .ok_or_else(|| "blocks_dir has no parent (can't locate _manifests/)".to_string())?
        .join("_manifests");
    for (i, h) in m.handoff.iter().enumerate() {
        let target_path = manifests_dir.join(format!("{}.toml", h.target));
        if !target_path.exists() {
            return Err(format!(
                "handoff[{i}].target '{}' has no manifest at {}",
                h.target,
                target_path.display()
            ));
        }
    }
    Ok(())
}

/// If a manifest declares `substrate_role`, verify the role file exists
/// and every capability it references has a `text.md`.
fn check_substrate_role(m: &Manifest, blocks_dir: &Path) -> Result<(), String> {
    let Some(role) = &m.substrate_role else { return Ok(()); };
    let root = blocks_dir
        .parent()
        .ok_or_else(|| "blocks_dir has no parent (can't locate _roles/)".to_string())?;
    let caps = substrate::load_role_capabilities(root, role)?;
    for cap in &caps {
        substrate::load_capability_text(root, cap)?;
    }
    Ok(())
}

/// v0.15: if a manifest references artifact schema names, they must be in the
/// known whitelist. Missing fields are allowed (non-breaking extension).
pub fn check_artifact_schemas(m: &Manifest, known: &BTreeSet<String>) -> Result<(), String> {
    if let Some(name) = &m.produces_artifact {
        check_known(name, "produces_artifact", known)?;
    }
    for (i, h) in m.handoff.iter().enumerate() {
        if let Some(name) = &h.expects_artifact {
            check_known(name, &format!("handoff[{i}].expects_artifact"), known)?;
        }
        if let Some(name) = &h.produces_artifact {
            check_known(name, &format!("handoff[{i}].produces_artifact"), known)?;
        }
    }
    Ok(())
}

fn check_known(name: &str, field: &str, known: &BTreeSet<String>) -> Result<(), String> {
    if known.contains(name) {
        return Ok(());
    }
    let list: Vec<&str> = known.iter().map(String::as_str).collect();
    Err(format!(
        "unknown artifact schema '{name}' in field '{field}' — must be one of {list:?}"
    ))
}

#[cfg(test)]
#[path = "validator_tests.rs"]
mod tests;
