//! Placeholder check — reject unsubstituted `{{PLACEHOLDER}}` tokens.
//!
//! Constructor Pattern: one cube = one validation concern.
//! Extracted from `validator.rs` to keep that file under 200 LOC.

use crate::manifest::Manifest;

/// Reject manifests that still carry `{{PLACEHOLDER}}` tokens — the wizard
/// should have substituted them. Matches `{{...}}` conservatively (not
/// single braces).
pub fn check(m: &Manifest) -> Result<(), String> {
    let check = |field: &str, value: &str| -> Result<(), String> {
        if contains_placeholder(value) {
            Err(format!(
                "Unsubstituted template placeholder in field '{field}': {value}. Did the wizard skip a substitution?"
            ))
        } else {
            Ok(())
        }
    };

    check("name", &m.name)?;
    check("description", &m.description)?;
    check("model", &m.model)?;
    check("role", &m.role)?;
    for (i, t) in m.tools.iter().enumerate() {
        check(&format!("tools[{i}]"), t)?;
    }
    for (i, b) in m.blocks.iter().enumerate() {
        check(&format!("blocks[{i}]"), b)?;
    }
    for (i, d) in m.domain_in.iter().enumerate() {
        check(&format!("domain_in[{i}]"), d)?;
    }
    for (i, d) in m.forbidden_domain.iter().enumerate() {
        check(&format!("forbidden_domain[{i}]"), d)?;
    }
    for (i, h) in m.handoff.iter().enumerate() {
        check(&format!("handoff[{i}].target"), &h.target)?;
        check(&format!("handoff[{i}].trigger"), &h.trigger)?;
    }
    for (i, o) in m.output_extra_fields.iter().enumerate() {
        check(&format!("output_extra_fields[{i}]"), o)?;
    }
    if let Some(v) = &m.substrate_role {
        check("substrate_role", v)?;
    }
    if let Some(v) = &m.memory_project {
        check("memory_project", v)?;
    }
    if let Some(v) = &m.project_claudemd {
        check("project_claudemd", v)?;
    }
    if let Some(r) = &m.references {
        for (i, e) in r.extra.iter().enumerate() {
            check(&format!("references.extra[{i}]"), e)?;
        }
    }
    Ok(())
}

fn contains_placeholder(s: &str) -> bool {
    if let Some(start) = s.find("{{") {
        if s[start + 2..].contains("}}") {
            return true;
        }
    }
    false
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

    #[test]
    fn rejects_placeholder_in_memory_project() {
        let mut m = base();
        m.memory_project = Some("{{MEMORY_PROJECT}}".into());
        let err = check(&m).unwrap_err();
        assert!(err.contains("memory_project"), "err = {err}");
        assert!(err.contains("{{MEMORY_PROJECT}}"), "err = {err}");
    }

    #[test]
    fn accepts_single_braces() {
        let mut m = base();
        m.description = "hello {world}".into();
        assert!(check(&m).is_ok());
    }

    #[test]
    fn accepts_empty_manifest() {
        assert!(check(&base()).is_ok());
    }

    #[test]
    fn rejects_placeholder_in_role() {
        let mut m = base();
        m.role = "do {{THING}}".into();
        assert!(check(&m).is_err());
    }
}
