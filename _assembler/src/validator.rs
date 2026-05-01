//! Manifest validator. Enforces Constructor Pattern invariants.
//! Hard-fails on missing obligatory blocks, missing handoffs, unknown blocks.
//!
//! Detailed sub-checks live in their own cubes:
//!   - `placeholders::check`      — {{PLACEHOLDER}} substitution guard
//!   - `schemas_export::load`     — dynamic artifact-schema whitelist loader
//!   - this file                  — structural checks + artifact-schema names

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
    for required in OBLIGATORY {
        if !m.blocks.iter().any(|b| b == required) {
            return Err(format!("missing obligatory block: {required}"));
        }
    }

    if m.handoff.is_empty() {
        return Err("at least one handoff required".into());
    }

    for block in &m.blocks {
        let path = blocks_dir.join(format!("{block}.md"));
        if !path.exists() {
            return Err(format!("block '{block}' not found at {}", path.display()));
        }
    }

    if m.domain_in.is_empty() {
        return Err("domain_in must have at least one entry".into());
    }
    if m.forbidden_domain.is_empty() {
        return Err("forbidden_domain must have at least one entry".into());
    }
    if m.role.trim().is_empty() {
        return Err("role must not be empty".into());
    }

    placeholders::check(m)?;
    let known = schemas_export::load(blocks_dir);
    check_artifact_schemas(m, &known)?;
    check_substrate_role(m, blocks_dir)?;

    Ok(())
}

/// If a manifest declares `substrate_role`, verify the role file exists
/// and every capability it references has a `text.md`. Keeping the check
/// here (not only at assemble time) turns mistakes into up-front failures.
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
fn check_artifact_schemas(m: &Manifest, known: &BTreeSet<String>) -> Result<(), String> {
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
mod tests {
    use super::*;
    use crate::manifest::{Handoff, Manifest};

    fn base() -> Manifest {
        Manifest {
            name: "test".into(),
            description: "d".into(),
            tools: vec!["Read".into()],
            model: "opus".into(),
            role: "r".into(),
            blocks: vec!["baseline".into(), "evidence-grading".into(), "memory-protocol".into()],
            domain_in: vec!["x".into()],
            forbidden_domain: vec!["y".into()],
            handoff: vec![Handoff {
                target: "a".into(),
                trigger: "b".into(),
                expects_artifact: None,
                produces_artifact: None,
            }],
            output_extra_fields: vec![],
            memory_project: None,
            project_claudemd: None,
            references: None,
            produces_artifact: None,
            substrate_role: None,
            rule_blocks: vec![],
        }
    }

    fn builtin_set() -> BTreeSet<String> {
        schemas_export::BUILTIN.iter().map(|s| (*s).to_string()).collect()
    }

    #[test]
    fn artifact_schemas_absent_passes() {
        let m = base();
        assert!(check_artifact_schemas(&m, &builtin_set()).is_ok());
    }

    #[test]
    fn artifact_schemas_known_names_pass() {
        let mut m = base();
        m.produces_artifact = Some("spec".into());
        m.handoff[0].expects_artifact = Some("plan".into());
        m.handoff[0].produces_artifact = Some("patch".into());
        assert!(check_artifact_schemas(&m, &builtin_set()).is_ok());
    }

    #[test]
    fn artifact_schemas_reject_unknown_produces() {
        let mut m = base();
        m.produces_artifact = Some("not-a-schema".into());
        let err = check_artifact_schemas(&m, &builtin_set()).unwrap_err();
        assert!(err.contains("not-a-schema"), "err: {err}");
        assert!(err.contains("produces_artifact"), "err: {err}");
    }

    #[test]
    fn artifact_schemas_reject_unknown_expects_in_handoff() {
        let mut m = base();
        m.handoff[0].expects_artifact = Some("zzz".into());
        let err = check_artifact_schemas(&m, &builtin_set()).unwrap_err();
        assert!(err.contains("zzz"), "err: {err}");
        assert!(err.contains("handoff[0].expects_artifact"), "err: {err}");
    }

    #[test]
    fn builtin_schemas_do_not_drift_from_kei_artifact() {
        // Structural drift test (no runtime dep on kei-artifact): read the
        // primitive's source and confirm its BUILTIN list matches ours.
        let primitive = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("_primitives/_rust/kei-artifact/src/schemas.rs");
        if !primitive.exists() {
            eprintln!("skip drift test: primitive not at {}", primitive.display());
            return;
        }
        let src = std::fs::read_to_string(&primitive).unwrap();
        let mut names: Vec<String> = Vec::new();
        for line in src.lines() {
            let t = line.trim();
            if let Some(rest) = t.strip_prefix("(\"") {
                if let Some(end) = rest.find("\",") {
                    names.push(rest[..end].to_string());
                }
            }
        }
        let mine: Vec<String> = schemas_export::BUILTIN
            .iter()
            .map(|s| (*s).to_string())
            .collect();
        assert_eq!(
            names, mine,
            "kei-artifact BUILTIN and schemas_export::BUILTIN drifted"
        );
    }
}
