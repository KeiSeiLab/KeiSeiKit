//! HIGH-security agent-id validator tests.
//!
//! Covers every documented rejection class + the happy path for the shapes
//! actually produced in `autogen_agent_id` and used in fixtures.

use kei_agent_runtime::spawn::{load_task, resolve_agent_id};
use kei_agent_runtime::validate::{
    autogen_agent_id, slugify_role, validate_agent_id, InvalidAgentId, MAX_AGENT_ID_LEN,
};
use kei_agent_runtime::capability::TaskSpec;
use std::fs;
use tempfile::tempdir;

// ---- basic shape ---------------------------------------------------------

#[test]
fn empty_rejected() {
    let err = validate_agent_id("").unwrap_err();
    assert!(err.reason.contains("empty"), "got: {}", err.reason);
}

#[test]
fn too_long_rejected() {
    let raw = "a".repeat(MAX_AGENT_ID_LEN + 1);
    let err = validate_agent_id(&raw).unwrap_err();
    assert!(err.reason.contains("length"), "got: {}", err.reason);
}

#[test]
fn exactly_max_length_ok() {
    let raw = "a".repeat(MAX_AGENT_ID_LEN);
    assert!(validate_agent_id(&raw).is_ok());
}

#[test]
fn non_ascii_rejected() {
    let err = validate_agent_id("agent-кириллица").unwrap_err();
    assert!(err.reason.to_lowercase().contains("ascii") || err.reason.contains("character"));
}

// ---- traversal-class bytes -----------------------------------------------

#[test]
fn parent_dir_rejected() {
    let err = validate_agent_id("foo..bar").unwrap_err();
    assert!(err.reason.contains(".."), "got: {}", err.reason);
}

#[test]
fn literal_double_dot_rejected() {
    let err = validate_agent_id("..").unwrap_err();
    // "..": starts with '.', so leading-dot rule OR traversal rule fires first
    // — implementation currently flags `..` first; either class is fine.
    assert!(err.reason.contains("..") || err.reason.contains("start"));
}

#[test]
fn slash_rejected() {
    let err = validate_agent_id("foo/bar").unwrap_err();
    assert!(err.reason.contains('/'), "got: {}", err.reason);
}

#[test]
fn backslash_rejected() {
    let err = validate_agent_id("foo\\bar").unwrap_err();
    assert!(err.reason.contains('\\'), "got: {}", err.reason);
}

#[test]
fn leading_dot_rejected() {
    let err = validate_agent_id(".secret").unwrap_err();
    assert!(err.reason.contains("start"), "got: {}", err.reason);
}

#[test]
fn leading_dash_rejected() {
    let err = validate_agent_id("-xyz").unwrap_err();
    assert!(err.reason.contains("start"), "got: {}", err.reason);
}

#[test]
fn nul_rejected() {
    let err = validate_agent_id("foo\0bar").unwrap_err();
    assert!(err.reason.contains("NUL"), "got: {}", err.reason);
}

#[test]
fn colon_rejected() {
    let err = validate_agent_id("foo:bar").unwrap_err();
    assert!(err.reason.contains(':'), "got: {}", err.reason);
}

#[test]
fn whitespace_rejected() {
    let err = validate_agent_id("foo bar").unwrap_err();
    assert!(err.reason.contains("whitespace"), "got: {}", err.reason);
}

#[test]
fn tab_rejected() {
    let err = validate_agent_id("foo\tbar").unwrap_err();
    assert!(err.reason.contains("whitespace"), "got: {}", err.reason);
}

// ---- valid shapes --------------------------------------------------------

#[test]
fn valid_simple_passes() {
    assert!(validate_agent_id("abc123").is_ok());
}

#[test]
fn valid_with_dashes_and_underscores_passes() {
    assert!(validate_agent_id("ag-edit-local-xyz_1").is_ok());
    assert!(validate_agent_id("ag-code.impl-abc").is_ok());
}

#[test]
fn fixture_edit_local_forge_abc123_passes() {
    // Exact shape used in prepare_smoke.rs `happy_path_yields_full_invocation`.
    assert!(validate_agent_id("edit-local-forge-abc123").is_ok());
}

// ---- Windows-reserved (case-insensitive) ---------------------------------

#[test]
fn windows_reserved_con_rejected() {
    assert!(validate_agent_id("CON").is_err());
    assert!(validate_agent_id("con").is_err());
    assert!(validate_agent_id("Con").is_err());
}

#[test]
fn windows_reserved_nul_prn_aux_rejected() {
    for n in ["NUL", "nul", "PRN", "prn", "AUX", "aux"] {
        assert!(validate_agent_id(n).is_err(), "expected {n} to be rejected");
    }
}

#[test]
fn windows_reserved_com_lpt_rejected() {
    for n in ["COM1", "com2", "COM9", "LPT1", "lpt5", "LPT9"] {
        assert!(validate_agent_id(n).is_err(), "expected {n} to be rejected");
    }
}

#[test]
fn windows_reserved_with_extension_rejected() {
    assert!(validate_agent_id("CON.txt").is_err());
    assert!(validate_agent_id("com1.log").is_err());
}

#[test]
fn windows_com0_or_com10_not_reserved() {
    // Only COM1..COM9 and LPT1..LPT9 are reserved.
    assert!(validate_agent_id("com0").is_ok());
    assert!(validate_agent_id("com10").is_ok());
    assert!(validate_agent_id("lpt0").is_ok());
}

#[test]
fn not_reserved_similar_prefixes_ok() {
    assert!(validate_agent_id("console").is_ok());
    assert!(validate_agent_id("comedy").is_ok());
    assert!(validate_agent_id("auxiliary").is_ok());
}

// ---- autogen agrees with validator ---------------------------------------

#[test]
fn autogen_output_passes_validator_100_draws() {
    for role in ["edit-local", "edit-shared", "explorer", "read-only", "weird role!!"] {
        for _ in 0..100 {
            let id = autogen_agent_id(role);
            validate_agent_id(&id).unwrap_or_else(|e| {
                panic!("autogen produced invalid id '{id}' for role '{role}': {e}")
            });
        }
    }
}

#[test]
fn autogen_prefix_is_ag_and_within_cap() {
    let id = autogen_agent_id("edit-local");
    assert!(id.starts_with("ag-"));
    assert!(id.len() <= MAX_AGENT_ID_LEN, "len={}", id.len());
}

#[test]
fn slugify_empty_becomes_x() {
    assert_eq!(slugify_role(""), "x");
    assert_eq!(slugify_role("!!!"), "x");
    assert_eq!(slugify_role("---"), "x");
}

#[test]
fn slugify_collapses_disallowed_but_keeps_identity() {
    assert_eq!(slugify_role("edit-local"), "edit-local");
    assert_eq!(slugify_role("Edit/Local"), "Edit_Local");
}

// ---- integration: resolve_agent_id + load_task propagate typed error ----

#[test]
fn resolve_agent_id_rejects_traversal_without_file_side_effect() {
    let mut task = TaskSpec::default();
    task.task.agent_id = "../../../etc/passwd".into();
    let err = resolve_agent_id(&task).expect_err("must reject");
    let msg = format!("{err:#}");
    assert!(msg.contains("rejected"), "error should mention rejection: {msg}");
}

#[test]
fn resolve_agent_id_rejects_slash() {
    let mut task = TaskSpec::default();
    task.task.agent_id = "foo/bar".into();
    assert!(resolve_agent_id(&task).is_err());
}

#[test]
fn resolve_agent_id_passes_valid() {
    let mut task = TaskSpec::default();
    task.task.agent_id = "edit-local-forge-abc123".into();
    let resolved = resolve_agent_id(&task).unwrap();
    assert_eq!(resolved, "edit-local-forge-abc123");
}

#[test]
fn load_task_rejects_hostile_agent_id() {
    let tmp = tempdir().unwrap();
    let path = tmp.path().join("task.toml");
    fs::write(
        &path,
        r#"
[task]
role = "edit-local"
agent-id = "../../../etc/shadow"
"#,
    )
    .unwrap();
    let err = load_task(&path).expect_err("hostile agent-id must be rejected at load");
    let msg = format!("{err:#}");
    assert!(msg.contains("rejected"), "got: {msg}");
}

#[test]
fn load_task_accepts_empty_agent_id() {
    // Empty agent-id is allowed at load (auto-gen happens in prepare()).
    let tmp = tempdir().unwrap();
    let path = tmp.path().join("task.toml");
    fs::write(
        &path,
        r#"
[task]
role = "edit-local"
"#,
    )
    .unwrap();
    let spec = load_task(&path).expect("empty agent-id should parse");
    assert_eq!(spec.task.agent_id, "");
}

// ---- InvalidAgentId is a typed, structured error ------------------------

#[test]
fn invalid_agent_id_is_thiserror_displayable() {
    let err: InvalidAgentId = validate_agent_id("foo/bar").unwrap_err();
    let display = format!("{err}");
    assert!(display.starts_with("invalid agent-id"));
}
