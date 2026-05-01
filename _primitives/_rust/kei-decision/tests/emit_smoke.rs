//! Smoke: full plan pipeline emits N task.toml files; each is valid TOML.

use kei_decision::{classify, emit_task_toml, parse_master_report, rank_actions};
use std::path::PathBuf;

fn fixture(name: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("tests/fixtures");
    p.push(name);
    p
}

#[test]
fn emit_writes_one_task_per_action_and_each_parses_as_toml() {
    let master = fixture("valid-master.md");
    let raws = parse_master_report(&master).expect("parse");
    let kinds: Vec<_> = raws.iter().map(classify).collect();
    let ranked = rank_actions(raws, kinds);
    let tmp = tempfile::tempdir().expect("tempdir");
    let mut paths = Vec::new();
    for action in &ranked {
        let out = emit_task_toml(action, tmp.path(), &master).expect("emit");
        paths.push(out.path);
    }
    assert_eq!(paths.len(), 5, "expected one task per ranked action");
    for path in &paths {
        let body = std::fs::read_to_string(path).expect("read emitted file");
        let parsed: toml::Value = toml::from_str(&body).expect("emitted file must be valid TOML");
        let task_section = parsed.get("task").expect("[task] section present");
        assert!(task_section.get("role").is_some(), "[task].role required");
        assert!(task_section.get("description").is_some(), "[task].description required");
        let scope = parsed.get("scope").expect("[scope] section present");
        assert!(scope.get("files-whitelist").is_some(), "[scope].files-whitelist required");
    }
}

#[test]
fn emit_filename_includes_action_id_and_slug() {
    let master = fixture("valid-master.md");
    let raws = parse_master_report(&master).expect("parse");
    let kinds: Vec<_> = raws.iter().map(classify).collect();
    let ranked = rank_actions(raws, kinds);
    let tmp = tempfile::tempdir().expect("tempdir");
    let action = &ranked[0];
    let out = emit_task_toml(action, tmp.path(), &master).expect("emit");
    let name = out.path.file_name().unwrap().to_str().unwrap();
    assert!(name.starts_with(&format!("action-{}", action.raw.id)),
            "filename should start with action-{}; got {}", action.raw.id, name);
    assert!(name.ends_with(".toml"));
}
