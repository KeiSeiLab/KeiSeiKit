//! `match_skill_command` finds `<project>/.claude/skills/<name>/SKILL.md`
//! when the user message starts with `/<name>`.
//!
//! Resolution: project-local wins over $HOME-local; missing file -> None;
//! malformed names rejected.

use crate::context::match_skill_command;
use std::fs;

fn project_with_skill(name: &str, body: &str) -> tempfile::TempDir {
    let td = tempfile::tempdir().unwrap();
    let dir = td.path().join(".claude/skills").join(name);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("SKILL.md"), body).unwrap();
    td
}

#[test]
fn matches_leading_slash_command() {
    let td = project_with_skill("onboard", "ONBOARD-BODY");
    let s = match_skill_command("/onboard let's go", td.path()).expect("matched");
    assert_eq!(s.name, "onboard");
    assert_eq!(s.body, "ONBOARD-BODY");
}

#[test]
fn matches_command_only_no_args() {
    let td = project_with_skill("plan", "P");
    let s = match_skill_command("/plan", td.path()).expect("matched");
    assert_eq!(s.name, "plan");
}

#[test]
fn no_leading_slash_is_none() {
    let td = project_with_skill("onboard", "X");
    assert!(match_skill_command("hello", td.path()).is_none());
}

#[test]
fn rejects_path_traversal() {
    let td = tempfile::tempdir().unwrap();
    assert!(match_skill_command("/../etc/passwd", td.path()).is_none());
}

#[test]
fn rejects_empty_after_slash() {
    let td = tempfile::tempdir().unwrap();
    assert!(match_skill_command("/ space", td.path()).is_none());
}

#[test]
fn missing_skill_file_returns_none() {
    let td = tempfile::tempdir().unwrap();
    // override $HOME so we can't accidentally hit the real one
    std::env::set_var("HOME", td.path());
    assert!(match_skill_command("/does-not-exist", td.path()).is_none());
}
