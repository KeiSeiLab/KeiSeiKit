//! Unit tests for manifest validator. Split per Constructor Pattern (file <200 LOC).
//! Imported by `validator.rs` as `#[cfg(test)] #[path = "validator_tests.rs"] mod tests;`

use super::{check_artifact_schemas, check_handoff_targets};
use crate::manifest::{Handoff, Manifest};
use crate::schemas_export;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

fn base() -> Manifest {
    Manifest {
        name: "test".into(),
        description: "d".into(),
        tools: vec!["Read".into()],
        model: "opus".into(),
        role: "r".into(),
        blocks: vec![
            "baseline".into(),
            "evidence-grading".into(),
            "memory-protocol".into(),
        ],
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
fn handoff_target_present_passes() {
    let tmp = tempfile::tempdir().unwrap();
    let blocks = tmp.path().join("project/_blocks");
    let manifests = tmp.path().join("project/_manifests");
    fs::create_dir_all(&blocks).unwrap();
    fs::create_dir_all(&manifests).unwrap();
    fs::write(manifests.join("target-agent.toml"), b"").unwrap();
    let mut m = base();
    m.handoff[0].target = "target-agent".into();
    assert!(check_handoff_targets(&m, &blocks).is_ok());
}

#[test]
fn handoff_target_missing_fails() {
    let tmp = tempfile::tempdir().unwrap();
    let blocks = tmp.path().join("project/_blocks");
    let manifests = tmp.path().join("project/_manifests");
    fs::create_dir_all(&blocks).unwrap();
    fs::create_dir_all(&manifests).unwrap();
    // Do NOT write target-agent.toml — tests that missing file → error.
    let mut m = base();
    m.handoff[0].target = "ghost-agent".into();
    let err = check_handoff_targets(&m, &blocks).unwrap_err();
    assert!(err.contains("ghost-agent"), "err: {err}");
    assert!(err.contains("handoff[0].target"), "err: {err}");
}

#[test]
fn builtin_schemas_do_not_drift_from_kei_artifact() {
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
