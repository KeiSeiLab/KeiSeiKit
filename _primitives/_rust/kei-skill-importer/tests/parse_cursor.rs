//! Cursor `.mdc` parser tests.

use kei_skill_importer::{import, SourceFormat};
use std::path::PathBuf;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn parses_description() {
    let skill = import(&fixture("cursor-react-rules.mdc"), SourceFormat::Cursor)
        .expect("parse");
    assert!(skill.description.contains("React"),
        "desc: {}", skill.description);
    assert_eq!(skill.source_format, SourceFormat::Cursor);
}

#[test]
fn globs_become_glob_tags() {
    let skill = import(&fixture("cursor-react-rules.mdc"), SourceFormat::Cursor)
        .expect("parse");
    let glob_tags: Vec<&String> =
        skill.tags.iter().filter(|t| t.starts_with("glob:")).collect();
    assert_eq!(glob_tags.len(), 2, "expect 2 glob tags, got {:?}", skill.tags);
    assert!(glob_tags.iter().any(|t| t.contains("design-system/**/*.tsx")));
}

#[test]
fn always_apply_false_does_not_add_tag() {
    let skill = import(&fixture("cursor-react-rules.mdc"), SourceFormat::Cursor)
        .expect("parse");
    assert!(!skill.tags.iter().any(|t| t == "alwaysApply"),
        "alwaysApply=false should not be tagged: {:?}", skill.tags);
}

#[test]
fn auto_detection_picks_cursor_for_mdc() {
    // Auto-detect via extension
    let skill = import(&fixture("cursor-react-rules.mdc"), SourceFormat::Auto)
        .expect("parse");
    assert_eq!(skill.source_format, SourceFormat::Cursor);
}
