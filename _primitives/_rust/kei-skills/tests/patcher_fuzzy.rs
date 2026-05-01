//! Fuzzy patcher tests — exact match, fuzzy near-match, multi-match
//! ambiguity, atomic write round-trip.

use kei_skills::format::{parse, serialize};
use kei_skills::patcher::{patch_skill, write_atomic, PatchError, FUZZY_THRESHOLD};
use std::path::PathBuf;
use tempfile::tempdir;

fn skill_with_body(body: &str) -> kei_skills::format::Skill {
    let src = format!("---\nname: t\ndescription: t\n---\n{body}");
    parse(&src, PathBuf::from("<inline>")).expect("parse")
}

#[test]
fn exact_match_single_replacement() {
    let s = skill_with_body("alpha\nbeta\ngamma\n");
    let patched = patch_skill(&s, "beta\n", "BETA\n", false).expect("patch");
    assert_eq!(patched.body, "alpha\nBETA\ngamma\n");
}

#[test]
fn exact_match_multiple_without_replace_all_errors() {
    let s = skill_with_body("foo\nfoo\nfoo\n");
    let err = patch_skill(&s, "foo\n", "bar\n", false).expect_err("must err");
    matches!(err, PatchError::MultipleMatches { count: 3 });
}

#[test]
fn replace_all_replaces_every_exact_match() {
    let s = skill_with_body("foo\nfoo\nfoo\n");
    let patched = patch_skill(&s, "foo\n", "bar\n", true).expect("patch");
    assert_eq!(patched.body, "bar\nbar\nbar\n");
}

#[test]
fn fuzzy_match_with_whitespace_drift() {
    // Body has trailing spaces; query lacks them. Similarity well above 0.85.
    let s = skill_with_body("first line\nsecond line   \nthird line\n");
    let patched = patch_skill(&s, "second line\n", "SECOND LINE\n", false).expect("fuzzy");
    assert!(patched.body.contains("SECOND LINE"));
    assert!(!patched.body.contains("second line"));
}

#[test]
fn fuzzy_match_below_threshold_returns_not_found() {
    let s = skill_with_body("alpha\nbeta\ngamma\n");
    let err = patch_skill(&s, "completely unrelated text", "x", false).expect_err("must err");
    matches!(err, PatchError::NotFound);
}

#[test]
fn empty_old_string_errors() {
    let s = skill_with_body("alpha\n");
    let err = patch_skill(&s, "", "x", false).expect_err("must err");
    matches!(err, PatchError::NotFound);
}

#[test]
fn fuzzy_threshold_is_documented_floor() {
    assert!(FUZZY_THRESHOLD >= 0.80);
    assert!(FUZZY_THRESHOLD <= 0.95);
}

#[test]
fn write_atomic_persists_to_disk() {
    let dir = tempdir().expect("tmp");
    let path = dir.path().join("SKILL.md");
    std::fs::write(&path, "---\nname: t\ndescription: t\n---\nold body\n").expect("seed");
    let s = parse(&std::fs::read_to_string(&path).unwrap(), path.clone()).expect("parse");
    let patched = patch_skill(&s, "old body\n", "new body\n", false).expect("patch");
    write_atomic(&patched).expect("write");
    let reread = std::fs::read_to_string(&path).expect("reread");
    assert!(reread.contains("new body"));
    let reparsed = parse(&reread, path).expect("reparse");
    assert_eq!(reparsed.body, "new body\n");
    let _ = serialize(&reparsed).expect("serialize roundtrip");
}
